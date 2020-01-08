use crate::algorithms::chabad_holidays::ChabadHoliday;
use crate::algorithms::israeli_holidays::IsraeliHoliday;
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

#[derive(Debug, Clone, PartialEq)]
pub struct CustomHoliday {
    pub printable: String,
    pub json: String,
    pub date: DayMonth,
    pub if_not_exists: Option<Vec<DayMonth>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DailyStudy {
    DafYomi,
    Rambam(RambamChapters),
    YerushalmiYomi,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RambamChapters {
    Three,
    One,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayMonth {
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
            Name::IsraeliHoliday(holiday) => {
                state.serialize_field("type", "IsraeliHoliday")?;
                match holiday {
                    IsraeliHoliday::YomHaAtzmaut => {
                        state.serialize_field("name", "YomHaAtzmaut")?
                    }
                    IsraeliHoliday::YomHaZikaron => {
                        state.serialize_field("name", "YomHaZikaron")?
                    }
                    IsraeliHoliday::YomYerushalayim => {
                        state.serialize_field("name", "YomYerushalayim")?
                    }
                    IsraeliHoliday::YomHaShoah => state.serialize_field("name", "YomHaShoah")?,
                    IsraeliHoliday::YomHaAliyah => state.serialize_field("name", "YomHaAliyah")?,
                    IsraeliHoliday::Sigd => state.serialize_field("name", "Sigd")?,
                }
            }
            Name::ChabadHoliday(holiday) => {
                state.serialize_field("type", "ChabadHoliday")?;
                match holiday {
                    ChabadHoliday::YudKislev => state.serialize_field("name", "YudKislev")?,
                    ChabadHoliday::YudTesKislev => state.serialize_field("name", "YudTesKislev")?,
                    ChabadHoliday::ChofKislev => state.serialize_field("name", "ChofKislev")?,
                    ChabadHoliday::YudBeisTammuz => {
                        state.serialize_field("name", "YudBeisTammuz")?
                    }
                    ChabadHoliday::YudGimmelTammuz => {
                        state.serialize_field("name", "YudGimmelTammuz")?
                    }
                }
            }
            Name::DailyStudy(daily_study) => {
                match daily_study {
                    DailyStudyOutput::Daf(daf) => {
                        let mut m = HashMap::new();
                        m.insert("masechta", daf.masechta_json.to_string());
                        m.insert("daf", daf.daf.to_string());
                        state.serialize_field("type", "DafYomi")?;
                        state.serialize_field("topic", &daf)?;
                    }
                    DailyStudyOutput::RambamThreeChapters(halacha) => {
                        state.serialize_field("type", "Rambam3Chapters")?;
                        let v = vec![&halacha.ch1, &halacha.ch2, &halacha.ch3];
                        state.serialize_field("topic", &v)?;
                    }
                    DailyStudyOutput::RambamOneChapters(halacha) => {
                        state.serialize_field("type", "Rambam1Chapter")?;
                        state.serialize_field("topic", &halacha)?;
                    }
                    DailyStudyOutput::YerushalmiYomi(yerushalmi_yomi) => {
                        state.serialize_field("type", "Yerushalmi")?;
                        state.serialize_field("topic", &yerushalmi_yomi)?;
                    }
                };
            }
            Name::ShabbosMevarchim(shabbos_mevarchim) => {
                state.serialize_field("type", "ShabbosMevarchim")?;
                state.serialize_field("month", &shabbos_mevarchim.hebrew_month)?;
                state.serialize_field("molad", &shabbos_mevarchim)?;
            }
        };
        state.end()
    }
}

#[derive(Debug, Clone)]
pub enum DailyStudyOutput {
    Daf(Daf),
    RambamThreeChapters(RambamThreeChapter),
    RambamOneChapters(RambamChapter),
    YerushalmiYomi(YerushalmiYomi),
}

#[derive(Debug, Clone)]
pub struct RambamThreeChapter {
    ch1: RambamChapter,
    ch2: RambamChapter,
    ch3: RambamChapter,
}

impl RambamThreeChapter {
    pub fn from_days(day: u16) -> Self {
        let day_1 = day * 3;
        let day_2 = day * 3 + 1;
        let day_3 = day * 3 + 2;
        let ch1 = RambamChapter::from_days(day_1);
        let ch2 = RambamChapter::from_days(day_2);
        let ch3 = RambamChapter::from_days(day_3);
        Self { ch1, ch2, ch3 }
    }

    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let mut sum = self.ch1.pretty_print(lock, language)?;
        sum += lock.write(b" - ").ok()?;
        sum += self.ch3.pretty_print(lock, language)?;
        Some(sum)
    }
}

#[derive(Debug, Clone)]
pub struct RambamChapter {
    halacha_english: &'static str,
    halacha_json: &'static str,
    halacha_hebrew: &'static str,
    chapter: u8,
}

impl Serialize for RambamChapter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::types::*;
        let mut state = serializer.serialize_struct("Day", 2)?;
        state.serialize_field("halacha", &self.halacha_json)?;
        state.serialize_field("chapter", &self.chapter)?;
        state.end()
    }
}

impl RambamChapter {
    pub fn from_days(day: u16) -> Self {
        let mut day = day;
        let mut index = 0;
        let mut halacha_english;
        let mut halacha_json;
        let mut halacha_hebrew;

        let chapter = loop {
            halacha_english = RAMBAM[index].0;
            halacha_hebrew = RAMBAM[index].1;
            halacha_json = RAMBAM[index].2;

            if day < (RAMBAM[index].3 as u16) {
                break day as u8 + 1;
            } else {
                day -= RAMBAM[index].3 as u16;
                index += 1;
            }
        };
        Self {
            halacha_english,
            halacha_json,
            halacha_hebrew,
            chapter,
        }
    }

    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let mut p = if language == Language::English {
            lock.write(self.halacha_english.as_bytes()).ok()?
        } else {
            lock.write(self.halacha_hebrew.as_bytes()).ok()?
        };
        p += lock.write(b" ").ok()?;
        let mut daf_arr = [b'\0'; 3];
        let count_y = itoa::write(&mut daf_arr[..], self.chapter).unwrap();
        p += lock.write(&daf_arr[..count_y]).ok()?;
        Some(p)
    }
}

#[derive(Debug, Clone)]
pub struct YerushalmiYomi {
    masechta_english: &'static str,
    masechta_json: &'static str,
    masechta_hebrew: &'static str,
    daf: u8,
}

impl YerushalmiYomi {
    pub fn from_days(day: u16) -> Self {
        let mut day = day;
        let mut index = 0;
        let mut masechta_english;
        let mut masechta_json;
        let mut masechta_hebrew;

        let daf = loop {
            masechta_english = YERUSHALMI[index].0;
            masechta_hebrew = YERUSHALMI[index].1;
            masechta_json = YERUSHALMI[index].2;

            if day < (YERUSHALMI[index].3 as u16) {
                break day as u8;
            } else {
                day -= YERUSHALMI[index].3 as u16;
                index += 1;
            }
        };
        Self {
            masechta_english,
            masechta_json,
            masechta_hebrew,
            daf,
        }
    }
    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let mut p = if language == Language::English {
            lock.write(self.masechta_english.as_bytes()).ok()?
        } else {
            lock.write(self.masechta_hebrew.as_bytes()).ok()?
        };
        p += lock.write(b" ").ok()?;
        let mut daf_arr = [b'\0'; 3];
        let count_y = itoa::write(&mut daf_arr[..], self.daf + 1).unwrap();
        p += lock.write(&daf_arr[..count_y]).ok()?;
        Some(p)
    }
}

impl Serialize for YerushalmiYomi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::types::*;
        let mut state = serializer.serialize_struct("Day", 2)?;
        state.serialize_field("masechta", &self.masechta_json)?;
        state.serialize_field("daf", &(self.daf + 1))?;
        state.end()
    }
}

#[derive(Debug, Clone)]
pub struct Daf {
    masechta_english: &'static str,
    masechta_json: &'static str,
    masechta_hebrew: &'static str,
    daf: u8,
}

impl Serialize for Daf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::types::*;
        let mut state = serializer.serialize_struct("day", 2)?;
        state.serialize_field("masechta", &self.masechta_json)?;
        state.serialize_field("daf", &(self.daf + 2))?;
        state.end()
    }
}

impl Daf {
    pub fn from_days(
        day: u16,
        gemaras: &[(&'static str, &'static str, &'static str, u8); 37],
    ) -> Self {
        let mut day = day;
        let mut index = 0;
        let mut masechta_english;
        let mut masechta_json;
        let mut masechta_hebrew;

        let daf = loop {
            masechta_english = gemaras[index].0;
            masechta_hebrew = gemaras[index].1;
            masechta_json = gemaras[index].2;

            if day < (gemaras[index].3 as u16 - 1) {
                break day as u8;
            } else {
                day -= gemaras[index].3 as u16 - 1;
                index += 1;
            }
        };
        Self {
            masechta_english,
            masechta_json,
            masechta_hebrew,
            daf,
        }
    }

    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let mut p = if language == Language::English {
            lock.write(self.masechta_english.as_bytes()).ok()?
        } else {
            lock.write(self.masechta_hebrew.as_bytes()).ok()?
        };
        p += lock.write(b" ").ok()?;
        let mut daf_arr = [b'\0'; 3];
        let count_y = itoa::write(&mut daf_arr[..], self.daf + 2).unwrap();
        p += lock.write(&daf_arr[..count_y]).ok()?;
        Some(p)
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
    ShabbosHaGadol,
    TaanisBechoros,
    ShabbosChazon,
    ShabbosNachamu,
    LeilSlichos,
    ShabbosShuva,
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
            AppError::LocationError(e) => {
                state.serialize_field("type", "LocationError")?;
                state.serialize_field("error", e)?;
            }
        };
        state.end()
    }
}

use crate::algorithms::shabbos_mevarchim::ShabbosMevarchim;
use crate::prelude::constants::{RAMBAM, YERUSHALMI};
use std::collections::HashMap;
use std::fmt;
use std::io::{BufWriter, StdoutLock, Write};

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
