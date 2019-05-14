use chrono::prelude::*;
use either::Either;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use serde::ser::*;
use serde::Serialize;
use std::borrow::Cow;

pub struct MainArgs {
    pub config: Option<Vec<Vec<DayVal>>>,
    pub output_type: OutputType,
    pub language: Language,
    pub command: Command,
}

#[derive(Eq, PartialEq)]
pub enum Language {
    English,
    Hebrew,
}

pub enum Command {
    Convert(ConvertArgs),
    List(ListArgs),
}

#[derive(Eq, PartialEq)]
pub enum OutputType {
    Regular,
    Pretty,
    JSON,
}

pub struct ConvertArgs {
    pub date: ConvertType,
}

pub enum ConvertType {
    Gregorian(chrono::Date<Utc>),
    Hebrew(HebrewDate),
}

pub struct ListArgs {
    pub year: YearType,
    pub location: Location,
    pub events: Vec<Either<TorahReadingType, CustomHoliday>>,
    pub amnt_years: u64,
    pub no_sort: bool,
}

#[derive(Eq, PartialEq)]
pub enum YearType {
    Gregorian(u64),
    Hebrew(u64),
}

#[derive(Clone, Debug)]
pub struct DayVal {
    pub day: chrono::DateTime<Utc>,
    pub name: Name,
}

impl Serialize for DayVal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::types::*;
        let mut state = serializer.serialize_struct("Day", 2)?;
        state.serialize_field("day", &self.day)?;
        match &self.name {
            Name::TorahReading(val) => match val {
                TorahReading::YomTov(yt) => {
                    state.serialize_field("type", "YomTov")?;
                    state.serialize_field("name", yt)?;
                }
                TorahReading::Chol(chol) => {
                    state.serialize_field("type", "Chol")?;
                    state.serialize_field("name", chol)?;
                }
                TorahReading::Shabbos(shabbos) => {
                    state.serialize_field("type", "Shabbos")?;
                    state.serialize_field("name", shabbos)?;
                }
                TorahReading::SpecialParsha(special_parsha) => {
                    state.serialize_field("type", "YomTov")?;
                    state.serialize_field("name", special_parsha)?;
                }
            },
            Name::CustomName { printable: _, json } => {
                state.serialize_field("type", "CustomVal")?;
                state.serialize_field("name", &json)?;
            }
        };
        state.end()
    }
}

#[derive(Debug, Clone)]
pub enum Name {
    TorahReading(TorahReading),
    CustomName {
        printable: Cow<'static, str>,
        json: Cow<'static, str>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CustomHoliday {
    Omer,
    Minor,
}
