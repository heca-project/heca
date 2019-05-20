use chrono::prelude::*;
use either::Either;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use serde::ser::*;
use serde::Serialize;
use std::borrow::Cow;

pub struct MainArgs {
    pub custom_days: Option<Vec<DayVal>>,
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
            Name::MinorDays(days) => {
                state.serialize_field("type", "MinorDays")?;
                state.serialize_field("name", days)?;
            }
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
    MinorDays(MinorDays),
    CustomName {
        printable: Cow<'static, str>,
        json: Cow<'static, str>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub enum MinorDays {
    Omer1,
    Omer2,
    Omer3,
    Omer4,
    Omer5,
    Omer6,
    Omer7,
    Omer8,
    Omer9,
    Omer10,
    Omer11,
    Omer12,
    Omer13,
    Omer14,
    Omer15,
    Omer16,
    Omer17,
    Omer18,
    Omer19,
    Omer20,
    Omer21,
    Omer22,
    Omer23,
    Omer24,
    Omer25,
    Omer26,
    Omer27,
    Omer28,
    Omer29,
    Omer30,
    Omer31,
    Omer32,
    Omer33,
    Omer34,
    Omer35,
    Omer36,
    Omer37,
    Omer38,
    Omer39,
    Omer40,
    Omer41,
    Omer42,
    Omer43,
    Omer44,
    Omer45,
    Omer46,
    Omer47,
    Omer48,
    Omer49,
    ErevYomKippur,
    ErevSukkos,
    ErevPesach,
    PesachSheni,
    LagBaOmer,
    ErevShavuos,
    ErevRoshHashanah,
    FifteenShvat,
    FifteenAv,
    PurimKattan,
    ShushanPurimKattan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CustomHoliday {
    Omer,
    Minor,
}

type Month = u32;
type Day = u32;
type Year = i32;
#[derive(Debug)]
pub enum AppError {
    ConversionError(ConversionError),
    ArgError(clap::Error),
    ArgUndefinedError(String),
    DayIsNotANumber(String),
    YearIsNotANumber(String),
    MonthNotParsed(String),
    CannotParseMonth(String),
    CannotParseDay(String),
    CannotParseYear(String),
    InvalidGregorianDate(Year, Month, Day),
    SplitDateError,
    TomlParsingError(String),
    ReadError(String),
    TypeError(String),
}

use clap::ErrorKind;
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AppError", 2)?;
        match self {
            AppError::TypeError(err) => {
                state.serialize_field("Type", "TypeError")?;
                state.serialize_field("Error", err)?;
            }
            AppError::ReadError(err) => {
                state.serialize_field("Type", "SplitDateError")?;
                state.serialize_field("Error", err)?;
            }
            AppError::SplitDateError => {
                state.serialize_field("Type", "SplitDateError")?;
            }
            AppError::TomlParsingError(err) => {
                state.serialize_field("Type", "TomlParsingError")?;
                state.serialize_field("Error", err)?;
            }
            AppError::MonthNotParsed(month) => {
                state.serialize_field("Type", "MonthNotParsed")?;
                state.serialize_field("Error", month)?;
            }
            AppError::CannotParseMonth(month) => {
                state.serialize_field("Type", "CannotParseMonth")?;
                state.serialize_field("Error", month)?;
            }
            AppError::CannotParseDay(day) => {
                state.serialize_field("Type", "CannotParseDay")?;
                state.serialize_field("Error", day)?;
            }
            AppError::CannotParseYear(day) => {
                state.serialize_field("Type", "CannotParseYear")?;
                state.serialize_field("Error", day)?;
            }
            AppError::InvalidGregorianDate(year, month, day) => {
                state.serialize_field("Type", "InvalidGregorianDay")?;
                state.serialize_field("Error", &format!("{}/{}/{}", year, month, day))?;
            }
            AppError::YearIsNotANumber(year) => {
                state.serialize_field("Type", "YearIsNotANumber")?;
                state.serialize_field("Error", year)?;
            }
            AppError::DayIsNotANumber(day) => {
                state.serialize_field("Type", "DayIsNotANumber")?;
                state.serialize_field("Error", day)?;
            }
            AppError::ArgUndefinedError(ce) => {
                state.serialize_field("Type", "ArgUndefinedError")?;
                state.serialize_field("Error", ce)?;
            }
            AppError::ConversionError(ce) => {
                state.serialize_field("Type", "ConversionError")?;
                state.serialize_field("Error", ce)?;
            }
            AppError::ArgError(err) => match err.kind {
                ErrorKind::InvalidValue => state.serialize_field("Type", "InvalidValue")?,
                ErrorKind::UnknownArgument => state.serialize_field("Type", "UnknownArgument")?,
                ErrorKind::InvalidSubcommand => {
                    state.serialize_field("Type", "InvalidSubcommand")?
                }
                ErrorKind::UnrecognizedSubcommand => {
                    state.serialize_field("Type", "UnrecognizedSubcommand")?
                }
                ErrorKind::EmptyValue => state.serialize_field("Type", "EmptyValue")?,
                ErrorKind::ValueValidation => state.serialize_field("Type", "ValueValidation")?,
                ErrorKind::TooManyValues => state.serialize_field("Type", "TooManyValues")?,
                ErrorKind::TooFewValues => state.serialize_field("Type", "TooFewValues")?,
                ErrorKind::WrongNumberOfValues => {
                    state.serialize_field("Type", "WrongNumberOfValues")?
                }
                ErrorKind::ArgumentConflict => {
                    state.serialize_field("Type", "WrongNumberOfValues")?
                }
                ErrorKind::MissingRequiredArgument => {
                    state.serialize_field("Type", "MissingRequiredArgument")?
                }
                ErrorKind::MissingSubcommand => {
                    state.serialize_field("Type", "MissingSubcommand")?
                }
                ErrorKind::MissingArgumentOrSubcommand => {
                    state.serialize_field("Type", "MissingArgumentOrSubcommand")?
                }
                ErrorKind::UnexpectedMultipleUsage => {
                    state.serialize_field("Type", "UnexpectedMultipleUsage")?
                }
                ErrorKind::InvalidUtf8 => state.serialize_field("Type", "InvalidUtf8")?,
                ErrorKind::HelpDisplayed => state.serialize_field("Type", "HelpDisplayed")?,
                ErrorKind::VersionDisplayed => state.serialize_field("Type", "VersionDisplayed")?,
                ErrorKind::ArgumentNotFound => state.serialize_field("Type", "ArgumentNotFound")?,
                ErrorKind::Io => state.serialize_field("Type", "Io")?,
                ErrorKind::Format => state.serialize_field("Type", "Format")?,
            },
        };
        state.end()
    }
}

use std::fmt;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::TypeError(err) => write!(
                f,
                r#"Cannot understand output format: {}. Options are ["regular", "pretty", json"]"#,
                err
            ),
            AppError::ReadError(err) => write!(f, "Read error: {}", err),
            AppError::SplitDateError => write!(
                f,
                "Cannot split the date. Deliminators are: '-', '/', '_', '\\', '.', ',', '=']"
            ),
            AppError::TomlParsingError(err) => write!(f, "Read error: {}", err),
            AppError::MonthNotParsed(month) => write!(f, "{} does not seem to be a month", month),
            AppError::CannotParseMonth(month) => write!(f, "Cannot parse month {}", month),
            AppError::CannotParseDay(day) => write!(f, "Cannot parse day {}", day),
            AppError::CannotParseYear(year) => write!(f, "Cannot parse year {}", year),
            AppError::InvalidGregorianDate(year, month, day) => write!(
                f,
                "{}/{}/{} (in Y/M/D) is not a valid Gregorian date",
                year, month, day
            ),
            AppError::YearIsNotANumber(year) => {
                write!(f, "{} is not a valid year as it's not a number", year)
            }
            AppError::DayIsNotANumber(day) => {
                write!(f, "{} is not a valid day as it's not a number", day)
            }
            AppError::ArgUndefinedError(ce) => write!(f, "{}", ce),
            AppError::ConversionError(ce) => write!(f, "{}", ce),
            AppError::ArgError(err) => write!(f, "{}", err),
        }
    }
}

impl std::convert::From<heca_lib::prelude::ConversionError> for AppError {
    fn from(source: ConversionError) -> Self {
        AppError::ConversionError(source)
    }
}

impl std::convert::From<clap::Error> for AppError {
    fn from(source: clap::Error) -> Self {
        AppError::ArgError(source)
    }
}

impl std::convert::From<toml::de::Error> for AppError {
    fn from(source: toml::de::Error) -> Self {
        AppError::TomlParsingError(source.to_string())
    }
}

impl std::convert::From<std::io::Error> for AppError {
    fn from(source: std::io::Error) -> Self {
        AppError::ReadError(source.to_string())
    }
}
