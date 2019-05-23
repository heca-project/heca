use std::num::NonZeroI8;

use chrono::prelude::*;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use serde::ser::*;
use serde::Serialize;

pub struct MainArgs {
    pub custom_days: Option<Vec<Name>>,
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

#[derive(Eq, PartialEq, Copy, Clone)]
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
    pub events: Vec<Event>,
    pub amnt_years: u64,
    pub no_sort: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    TorahReadingType(TorahReadingType),
    MinorHoliday(MinorHoliday),
    CustomHoliday(CustomHoliday),
}
#[derive(Debug, Clone, PartialEq)]
pub struct CustomHoliday {
    pub printable: String,
    pub json: String,
    pub month: HebrewMonth,
    pub day: NonZeroI8,
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
            Name::CustomHoliday(custom_holiday) => {
                state.serialize_field("type", "CustomHoliday")?;
                state.serialize_field("name", &custom_holiday.json)?;
            }
        };
        state.end()
    }
}

#[derive(Debug, Clone)]
pub enum Name {
    TorahReading(TorahReading),
    MinorDays(MinorDays),
    CustomHoliday(CustomHoliday),
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
pub enum MinorHoliday {
    Omer,
    Minor,
}

type Month = u32;
type Day = u32;
type Year = i32;
#[derive(Debug)]
pub enum AppError {
    DateSyntaxError(String),
    ConversionError(ConversionError),
    ArgError(clap::Error),
    ArgUndefinedError(String),
    DayIsNotAValidNumber(String),
    YearIsNotANumber(String),
    MonthNotParsed(String),
    CannotParseMonth(String),
    CannotParseDay(String),
    CannotParseYear(String),
    InvalidGregorianDate(Year, Month, Day),
    SplitDateError,
    ConfigError(String),
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
            AppError::DateSyntaxError(err) => {
                state.serialize_field("type", "DateSyntaxError")?;
                state.serialize_field("error", err)?;
            }
            AppError::TypeError(err) => {
                state.serialize_field("type", "TypeError")?;
                state.serialize_field("error", err)?;
            }
            AppError::ReadError(err) => {
                state.serialize_field("type", "ReadError")?;
                state.serialize_field("error", err)?;
            }
            AppError::SplitDateError => {
                state.serialize_field("type", "SplitDateError")?;
            }
            AppError::ConfigError(err) => {
                state.serialize_field("type", "ConfigError")?;
                state.serialize_field("error", err)?;
            }
            AppError::MonthNotParsed(month) => {
                state.serialize_field("type", "MonthNotParsed")?;
                state.serialize_field("error", month)?;
            }
            AppError::CannotParseMonth(month) => {
                state.serialize_field("type", "CannotParseMonth")?;
                state.serialize_field("error", month)?;
            }
            AppError::CannotParseDay(day) => {
                state.serialize_field("type", "CannotParseDay")?;
                state.serialize_field("error", day)?;
            }
            AppError::CannotParseYear(day) => {
                state.serialize_field("type", "CannotParseYear")?;
                state.serialize_field("error", day)?;
            }
            AppError::InvalidGregorianDate(year, month, day) => {
                state.serialize_field("type", "InvalidGregorianDay")?;
                state.serialize_field("error", &format!("{}/{}/{}", year, month, day))?;
            }
            AppError::YearIsNotANumber(year) => {
                state.serialize_field("type", "YearIsNotANumber")?;
                state.serialize_field("error", year)?;
            }
            AppError::DayIsNotAValidNumber(day) => {
                state.serialize_field("type", "DayIsNotAValidNumber")?;
                state.serialize_field("error", day)?;
            }
            AppError::ArgUndefinedError(ce) => {
                state.serialize_field("type", "ArgUndefinedError")?;
                state.serialize_field("error", ce)?;
            }
            AppError::ConversionError(ce) => {
                state.serialize_field("type", "ConversionError")?;
                state.serialize_field("error", ce)?;
            }
            AppError::ArgError(err) => match err.kind {
                ErrorKind::InvalidValue => state.serialize_field("type", "InvalidValue")?,
                ErrorKind::UnknownArgument => state.serialize_field("type", "UnknownArgument")?,
                ErrorKind::InvalidSubcommand => {
                    state.serialize_field("type", "InvalidSubcommand")?
                }
                ErrorKind::UnrecognizedSubcommand => {
                    state.serialize_field("type", "UnrecognizedSubcommand")?
                }
                ErrorKind::EmptyValue => state.serialize_field("type", "EmptyValue")?,
                ErrorKind::ValueValidation => state.serialize_field("type", "ValueValidation")?,
                ErrorKind::TooManyValues => state.serialize_field("type", "TooManyValues")?,
                ErrorKind::TooFewValues => state.serialize_field("type", "TooFewValues")?,
                ErrorKind::WrongNumberOfValues => {
                    state.serialize_field("type", "WrongNumberOfValues")?
                }
                ErrorKind::ArgumentConflict => {
                    state.serialize_field("type", "WrongNumberOfValues")?
                }
                ErrorKind::MissingRequiredArgument => {
                    state.serialize_field("type", "MissingRequiredArgument")?
                }
                ErrorKind::MissingSubcommand => {
                    state.serialize_field("type", "MissingSubcommand")?
                }
                ErrorKind::MissingArgumentOrSubcommand => {
                    state.serialize_field("type", "MissingArgumentOrSubcommand")?
                }
                ErrorKind::UnexpectedMultipleUsage => {
                    state.serialize_field("type", "UnexpectedMultipleUsage")?
                }
                ErrorKind::InvalidUtf8 => state.serialize_field("type", "InvalidUtf8")?,
                ErrorKind::HelpDisplayed => state.serialize_field("type", "HelpDisplayed")?,
                ErrorKind::VersionDisplayed => state.serialize_field("type", "VersionDisplayed")?,
                ErrorKind::ArgumentNotFound => state.serialize_field("type", "ArgumentNotFound")?,
                ErrorKind::Io => state.serialize_field("type", "Io")?,
                ErrorKind::Format => state.serialize_field("type", "Format")?,
            },
        };
        state.end()
    }
}

use std::fmt;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DateSyntaxError(err) => write!(
                f,
                r#"Cannot parse date {} in config file. The date must be in the format of YYYY/MM/DD|Name"#,
                err
            ),
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
            AppError::ConfigError(err) => write!(f, "Error in configuration file: {}", err),
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
            AppError::DayIsNotAValidNumber(day) => {
                write!(f, "{} is not a valid day as it's not a number larger than 0", day)
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
        AppError::ConfigError(source.to_string())
    }
}

impl std::convert::From<std::io::Error> for AppError {
    fn from(source: std::io::Error) -> Self {
        AppError::ReadError(source.to_string())
    }
}
