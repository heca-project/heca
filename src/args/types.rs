use chrono::prelude::*;
use either::{Either, Left, Right};
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

#[derive(Clone, Debug, Serialize)]
pub struct DayVal {
    pub day: chrono::DateTime<Utc>,
    pub name: Name,
}

#[derive(Debug, Clone)]
pub enum Name {
    TorahReading(TorahReading),
    CustomName {
        printable: Cow<'static, str>,
        json: Cow<'static, str>,
    },
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::types::*;
        let mut state = serializer.serialize_struct("Name", 2)?;
        match self {
            Name::TorahReading(val) => state.serialize_field("TorahReading", val)?,
            Name::CustomName { printable, json } => state.serialize_field("CustomVal", json)?,
        };
        state.end()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CustomHoliday {
    Omer,
    Minor,
}
