use crate::algorithms::candle_lighting::CITIES;
use crate::algorithms::chabad_holidays::ChabadHoliday;
use crate::algorithms::custom_holidays::CustomHoliday;
use crate::algorithms::daily_study::{DailyStudy, DailyStudyOutput};
use crate::algorithms::israeli_holidays::IsraeliHoliday;
use crate::algorithms::minor_days::types::MinorDays;
use crate::algorithms::minor_days::types::MinorHoliday;
use crate::algorithms::torah_reading::json_print;
use crate::prelude::JsonPrinter;
use crate::prelude::{string_to_json, Json};

use chrono::prelude::*;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use std::io::{BufWriter, StderrLock, StdoutLock, Write};

use crate::algorithms::candle_lighting::City;
use crate::algorithms::shabbos_mevarchim::ShabbosMevarchim;
use std::fmt;

pub struct MainArgs {
    pub custom_days: Option<Vec<Name>>,
    pub output_type: OutputType,
    pub language: Language,
    pub command: Command,
}

#[derive(Eq, PartialEq, Clone, Copy)]
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
    pub language: Language,
}

#[derive(Debug)]
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
    pub exact_days: bool,
    pub city: Option<City>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    TorahReadingType(TorahReadingType),
    MinorHoliday(MinorHoliday),
    CustomHoliday(CustomHoliday),
    DailyStudy(DailyStudy),
    IsraeliHolidays,
    ChabadHolidays,
    ShabbosMevarchim,
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
    pub candle_lighting: Option<Option<DateTime<FixedOffset>>>,
}

impl DayVal {
    pub(crate) fn json_print<'a, 'b>(&self, lock: &'a mut BufWriter<StdoutLock<'b>>) {
        use crate::types::*;
        let mut json = Json::new(lock);
        json.start();
        json.print_map_unchecked("day", &self.day.to_rfc3339_opts(SecondsFormat::Secs, true));
        json.next();

        match &self.name {
            Name::TorahReading(tr) => {
                json_print(tr, &self, &mut json);
            }
            Name::MinorDays(days) => {
                days.json_print(&mut json);
            }
            Name::CustomHoliday(custom_holiday) => {
                custom_holiday.json_print(&mut json);
            }
            Name::IsraeliHoliday(holiday) => {
                holiday.json_print(&mut json);
            }
            Name::ChabadHoliday(holiday) => {
                holiday.json_print(&mut json);
            }
            Name::DailyStudy(daily_study) => {
                daily_study.json_print(&mut json);
            }
            Name::ShabbosMevarchim(shabbos_mevarchim) => {
                shabbos_mevarchim.json_print(&mut json);
            }
        };
        json.end();
    }
}

#[derive(Debug, Clone)]
pub enum Name {
    TorahReading(TorahReading),
    MinorDays(MinorDays),
    CustomHoliday(CustomHoliday),
    DailyStudy(DailyStudyOutput),
    IsraeliHoliday(IsraeliHoliday),
    ChabadHoliday(ChabadHoliday),
    ShabbosMevarchim(ShabbosMevarchim),
}

type Month = u32;
type Day = u32;
type Year = i32;

#[derive(Debug)]
pub enum AppError {
    LocationError(String),
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
    CityNotFound(String),
}

use clap::ErrorKind;
impl AppError {
    pub(crate) fn json_print(&self, lock: &mut BufWriter<StderrLock<'_>>) {
        match self {
            AppError::DateSyntaxError(err) => {
                let out = format!(
                    r#"{{"type":"DateSyntaxError","error":"{}"}}"#,
                    string_to_json(err)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::TypeError(err) => {
                let out = format!(
                    r#"{{"type":"TypeError","error":"{}"}}"#,
                    string_to_json(err)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::ReadError(err) => {
                let out = format!(
                    r#"{{"type":"ReadError","error":"{}"}}"#,
                    string_to_json(err)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::SplitDateError => {
                lock.write(r#"{"type":"SplitDateError"}"#.as_bytes())
                    .unwrap();
            }
            AppError::ConfigError(err) => {
                let out = format!(
                    r#"{{"type":"ConfigError","error":"{}"}}"#,
                    string_to_json(err)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::MonthNotParsed(month) => {
                let out = format!(
                    r#"{{"type":"MonthNotParsed","error":"{}"}}"#,
                    string_to_json(month)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::CannotParseMonth(month) => {
                let out = format!(
                    r#"{{"type":"CannotParseMonth","error":"{}"}}"#,
                    string_to_json(month)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::CannotParseDay(day) => {
                let out = format!(
                    r#"{{"type":"CannotParseDay","error":"{}"}}"#,
                    string_to_json(day)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::CannotParseYear(day) => {
                let out = format!(
                    r#"{{"type":"CannotParseYear","error":"{}"}}"#,
                    string_to_json(day)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::InvalidGregorianDate(year, month, day) => {
                let out = format!(
                    r#"{{"type":"InvalidGregorianDay","error":"{}"}}"#,
                    &format!("{}/{}/{}", year, month, day)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::YearIsNotANumber(year) => {
                let out = format!(
                    r#"{{"type":"YearIsNotANumber","error":"{}"}}"#,
                    string_to_json(year)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::DayIsNotAValidNumber(day) => {
                let out = format!(
                    r#"{{"type":"DayIsNotAValidNumber","error":"{}"}}"#,
                    string_to_json(day)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::ArgUndefinedError(ce) => {
                let out = format!(
                    r#"{{"type":"ArgUndefinedError","error":"{}"}}"#,
                    string_to_json(ce)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::ConversionError(ce) => {
                let out = format!(
                    r#"{{"type":"ConversionError","error":{}}}"#,
                    match ce {
                        ConversionError::IsNotLeapYear => String::from(r#""IsNotLeapYear""#),
                        ConversionError::IsLeapYear => String::from(r#""IsLeapYear""#),
                        ConversionError::YearTooSmall => String::from(r#""YearTooSmall""#),
                        ConversionError::TooManyDaysInMonth(days) =>
                            format!(r#"{{"TooManyDaysInMonth":{}}}"#, days),
                    }
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::ArgError(err) => match err.kind {
                ErrorKind::InvalidValue => {
                    lock.write(r#"{"type":"InvalidValue"}"#.as_bytes()).unwrap();
                }
                ErrorKind::UnknownArgument => {
                    lock.write(r#"{"type":"UnknownArgument"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::InvalidSubcommand => {
                    lock.write(r#"{"type":"InvalidSubcommand"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::UnrecognizedSubcommand => {
                    lock.write(r#"{"type":"UnrecognizedSubcommand"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::EmptyValue => {
                    lock.write(r#"{"type":"EmptyValue"}"#.as_bytes()).unwrap();
                }
                ErrorKind::ValueValidation => {
                    lock.write(r#"{"type": "ValueValidation"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::TooManyValues => {
                    lock.write(r#"{"type": "TooManyValues"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::TooFewValues => {
                    lock.write(r#"{"type": "TooFewValues"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::WrongNumberOfValues => {
                    lock.write(r#"{"type": "WrongNumberOfValues"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::ArgumentConflict => {
                    lock.write(r#"{"type": "ArgumentConflict"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::MissingRequiredArgument => {
                    lock.write(r#"{"type": "MissingRequiredArgument"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::MissingSubcommand => {
                    lock.write(r#"{"type": "MissingSubcommand"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::MissingArgumentOrSubcommand => {
                    lock.write(r#"{"type": "MissingArgumentOrSubcommand"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::UnexpectedMultipleUsage => {
                    lock.write(r#"{"type": "UnexpectedMultipleUsage"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::InvalidUtf8 => {
                    lock.write(r#"{"type": "InvalidUtf8"}"#.as_bytes()).unwrap();
                }
                ErrorKind::HelpDisplayed => {
                    lock.write(r#"{"type": "HelpDisplayed"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::VersionDisplayed => {
                    lock.write(r#"{"type": "VersionDisplayed"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::ArgumentNotFound => {
                    lock.write(r#"{"type": "ArgumentNotFound"}"#.as_bytes())
                        .unwrap();
                }
                ErrorKind::Io => {
                    lock.write(r#"{"type": "Io"}"#.as_bytes()).unwrap();
                }
                ErrorKind::Format => {
                    lock.write(r#"{"type": "Format"}"#.as_bytes()).unwrap();
                }
            },
            AppError::LocationError(e) => {
                let out = format!(
                    r#"{{"type": "LocationError", "error": "{}"}}"#,
                    string_to_json(e)
                );
                lock.write(out.as_bytes()).unwrap();
            }
            AppError::CityNotFound(e) => {
                let out = format!(
                    r#"{{"type": "CityNotFoundError", "error": "{}"}}"#,
                    string_to_json(e)
                );
                lock.write(out.as_bytes()).unwrap();
            }
        };
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DateSyntaxError(err) => write!(
                f,
                r#"Cannot parse date {} in config file. The date must be in the format of YYYY/MM/DD|Name"#,
                err
            ),
            AppError::TypeError(err) => write!(
                f,
                r#"Cannot understand output format: {}. Options are ["regular", "pretty", "json"]"#,
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
            AppError::DayIsNotAValidNumber(day) => write!(
                f,
                "{} is not a valid day as it's not a number larger than 0",
                day
            ),
            AppError::ArgUndefinedError(ce) => write!(f, "{}", ce),
            AppError::ConversionError(ce) => write!(f, "{}", ce),
            AppError::ArgError(err) => write!(f, "{}", err),
            AppError::LocationError(e) => write!(
                f,
                "{} is not a valid location. Must be either \"Chul\" or \"Israel\"",
                e
            ),
            AppError::CityNotFound(e) => {
                let mut cities_sorted: Vec<_> = CITIES.iter().collect();
                cities_sorted.sort_by(|a, b| a.name.cmp(&b.name));
                let list_of_city_names = cities_sorted
                    .iter()
                    .map(|x| x.name.clone())
                    .fold(String::new(), |old, new| old + "\n" + &new);
                write!(
                    f,
                    "Could not find city {}. Possible options are: {}",
                    e, list_of_city_names
                )
            }
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
