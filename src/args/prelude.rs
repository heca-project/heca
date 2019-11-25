use crate::args::types::{AppError, CustomHoliday, DayMonth, Language};
use crate::args::DATE_TOKEN;
use heca_lib::prelude::{HebrewMonth, Location};
use serde::Deserialize;
use std::convert::TryInto;
use std::fs;
use std::num::NonZeroI8;

pub fn str_to_location(location: &str) -> Result<Location, AppError> {
    match location.to_lowercase().as_ref() {
        "chul" => Ok(Location::Chul),
        "israel" => Ok(Location::Israel),
        x => Err(AppError::LocationError(x.into())),
    }
}

pub struct Config {
    pub custom_days: Vec<CustomHoliday>,
    pub language: Option<Language>,
    pub location: Option<Location>,
}

impl Config {
    pub fn from_location(pass_value: Option<&str>) -> Result<Self, AppError> {
        let config_file = if let Some(v) = pass_value {
            Some(String::from(v))
        } else if let Ok(base_dir) = xdg::BaseDirectories::with_prefix("heca") {
            if let Some(path) = base_dir.find_config_file("config.toml") {
                Some(String::from(path.to_string_lossy()))
            } else {
                None
            }
        } else {
            None
        };
        let mut custom_days = vec![];
        let mut language = None;
        let mut location = None;
        if let Some(ref file) = config_file {
            let f = &fs::read_to_string(file)?;
            let config: ConfigFile = toml::from_str(f)?;
            if let Some(loc) = &config.location {
                location = Some(str_to_location(loc.as_ref())?);
            }

            language = config
                .language
                .and_then(|lang_string| match lang_string.as_ref() {
                    "en_US" => Some(Language::English),
                    "he_IL" => Some(Language::Hebrew),
                    l => panic!(
                        "Wrong language type {} in config file. Must be \"en_US\" or \"he_IL\"",
                        l
                    ),
                });
            if let Some(days) = config.days {
                for e in days {
                    let date = e.date;
                    let printable = e.title;
                    let json = e.json;
                    let h_date = date.split(&DATE_TOKEN[..]).collect::<Vec<&str>>();

                    if h_date.len() != 2 {
                        return Err(AppError::DateSyntaxError(date));
                    }
                    let (day, month, _) = parse_hebrew(&h_date)?;

                    custom_days.push(CustomHoliday {
                        date: DayMonth {
                            month,
                            day: NonZeroI8::new(
                                day.try_into().map_err(|_| {
                                    AppError::DayIsNotAValidNumber(format!("{}", day))
                                })?,
                            )
                            .ok_or_else(|| AppError::DayIsNotAValidNumber(format!("{}", day)))?,
                        },
                        printable,
                        json,
                        if_not_exists: None,
                    });
                }
            }
        }
        Ok(Self {
            language,
            custom_days,
            location,
        })
    }
}

pub fn parse_hebrew(sp: &[&str]) -> Result<(NonZeroI8, HebrewMonth, Option<u64>), AppError> {
    let day: i8 = sp[0]
        .parse()
        .map_err(|_| AppError::DayIsNotAValidNumber(sp[0].to_owned()))?;
    let day = if let Some(day) = NonZeroI8::new(
        day.try_into()
            .map_err(|_| AppError::DayIsNotAValidNumber(sp[0].into()))?,
    ) {
        day
    } else {
        return Err(AppError::DayIsNotAValidNumber(sp[0].to_owned()));
    };
    let year = if let Some(y) = sp.get(2) {
        Some(
            y.parse()
                .map_err(|_| AppError::YearIsNotANumber(sp[2].to_owned()))?,
        )
    } else {
        None
    };
    let month = str_to_month(sp[1])
        .or_else(|| str_to_month(&(String::from(sp[1]).to_lowercase())))
        .ok_or_else(|| AppError::MonthNotParsed(sp[1].to_owned()))?;
    Ok((day, month, year))
}

fn str_to_month(text: &str) -> Option<HebrewMonth> {
    match text {
        "תשרי" => Some(HebrewMonth::Tishrei),
        "חשוון" => Some(HebrewMonth::Cheshvan),
        "כסלו" => Some(HebrewMonth::Kislev),
        "טבת" => Some(HebrewMonth::Teves),
        "שבט" => Some(HebrewMonth::Shvat),
        "אדר" => Some(HebrewMonth::Adar),
        "אדרא" => Some(HebrewMonth::Adar1),
        "אדרב" => Some(HebrewMonth::Adar2),
        "ניסן" => Some(HebrewMonth::Nissan),
        "אייר" => Some(HebrewMonth::Iyar),
        "סיוון" => Some(HebrewMonth::Sivan),
        "תמוז" => Some(HebrewMonth::Tammuz),
        "אב" => Some(HebrewMonth::Av),
        "אלול" => Some(HebrewMonth::Elul),
        title => match title {
            "tishrei" | "tishre" => Some(HebrewMonth::Tishrei),
            "cheshvan" | "marcheshvan" | "mar cheshvan" => Some(HebrewMonth::Cheshvan),
            "kislev" => Some(HebrewMonth::Kislev),
            "teves" | "tevet" | "teiveis" => Some(HebrewMonth::Teves),
            "shvat" | "shevat" => Some(HebrewMonth::Shvat),
            "adar" => Some(HebrewMonth::Adar),
            "adar1" | "adar 1" | "adar aleph" | "adar rishon" => Some(HebrewMonth::Adar1),
            "adar2" | "adar 2" | "adar beis" | "adar bet" | "adar sheini" => {
                Some(HebrewMonth::Adar2)
            }
            "nissan" | "Nisan" => Some(HebrewMonth::Nissan),
            "iyar" => Some(HebrewMonth::Iyar),
            "sivan" => Some(HebrewMonth::Sivan),
            "tammuz" | "tamuz" => Some(HebrewMonth::Tammuz),
            "av" | "menachem av" => Some(HebrewMonth::Av),
            "elul" | "ellul" => Some(HebrewMonth::Elul),
            _ => None,
        },
    }
}

#[derive(Deserialize)]
struct ConfigFile {
    days: Option<Vec<InnerDate>>,
    language: Option<String>,
    location: Option<String>,
}

#[derive(Deserialize)]
struct InnerDate {
    date: String,
    title: String,
    json: String,
    #[serde(rename = "ifNotExists")]
    if_not_exists: Option<Vec<String>>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ConfigDateFmt {
    ISO,
    US,
    UK,
    M,
    L,
    B,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ConfigLocation {
    Israel,
    Chul,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ConfigDateType {
    Hebrew,
    Gregorian,
    Fuzzy,
}
