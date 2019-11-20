use clap::{App, Arg, ArgMatches, SubCommand};

pub mod types;

use crate::args::types::*;
use chrono::prelude::*;
use chrono::prelude::*;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use serde::Deserialize;
use std::convert::TryInto;
use std::env;
use std::fs;
use std::num::NonZeroI8;

const DATE_TOKEN: [char; 8] = ['-', '/', '_', '\\', '.', ',', '=', ' '];

pub fn build_args<I, T>(_args: I, output_type: OutputType) -> Result<MainArgs, AppError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    parse_args(App::new("Hebrew Calendar Manipulator")
                   .version("0.2.0")
                   .about("This program is a fast utility to convert and analyze dates in the Hebrew Calendar.")
                   .arg(Arg::with_name("configfile")
                       .long("config")
                       .help("Sets a custom config file (default: $XDG_CONFIG_HOME/heca/config.toml)")
                       .takes_value(true)
                       .required(false))
                   .arg(Arg::with_name("type")
                            .long("print")
                            .help(r#"Set output type. Options are ["regular", "pretty", "json"]"#)
                            .takes_value(true)
                            .required(false), )
                   .arg(Arg::with_name("language")
                       .long("language")
                       .help("Set language")
                       .possible_values(&["en_US", "he_IL"])
                       .takes_value(true)
                       .required(false))
                   .subcommand(SubCommand::with_name("convert")
                       .about("Converts Hebrew to Gregorian and back")
                       .arg(Arg::with_name("DateFormat")
                           .long("datefmt")
                           .help("Set date format (for Gregorian only): US or M for mm/dd/yyyy, UK or L for dd/mm/yyyy, ISO or B for yyyy/mm/dd")
                           .possible_values(&["US", "M", "UK", "L", "ISO", "B"])
                           .takes_value(true)
                           .required(false)
                           .default_value("ISO"))
                       .arg(Arg::with_name("T")
                           .long("type")
                           .long_help("Force conversion from type T, where T is either \"hebrew\" (then date must be written as '5/אדרא/5779'), as \"gregorian\" (where the date must be written as '1996/12/19'), or fuzzy (is Hebrew if year is above 4000, Gregorian otherwise).")
                           .possible_values(&["hebrew", "gregorian", "fuzzy"])
                           .takes_value(true)
                           .required(false)
                           .default_value("fuzzy"))
                       .arg(Arg::with_name("Date")
                           .required(true)
                           .takes_value(true)))
                   .subcommand(SubCommand::with_name("list")
                       .arg(Arg::with_name("YearType")
                           .long("type")
                           .help("Specify if the year is a Hebrew or a Gregorian year.")
                           .possible_values(&["hebrew", "gregorian", "fuzzy"])
                           .default_value("fuzzy")
                           .takes_value(true)
                           .required(false))
                       .arg(Arg::with_name("NoSort")
                           .long("no-sort")
                           .help("Don't sort output."))
                       .arg(Arg::with_name("Location")
                           .long("location")
                           .help("Are you looking for an Israeli calendar or a Chutz La'aretz calendar?")
                           .takes_value(true)
                           .required(false)
                           .possible_values(&["Chul", "Israel"]))
                       .arg(Arg::with_name("AmountYears")
                           .long("years")
                           .help("Generate events for n years")
                           .takes_value(true)
                           .required(false)
                           .default_value("1")).arg(Arg::with_name("Events")
                       .long("show")
                       .help("What events to list")
                       .takes_value(true)
                       .multiple(true)
                       .required(false)
                       .use_delimiter(true)
                       .possible_values(&["yom-tov", "shabbos", "special-parshas", "chol", "minor-holidays", "omer", "custom-holidays"]).default_value("yom-tov")).arg(Arg::with_name("Year")
                       .required(true)
                       .takes_value(true))).get_matches_safe()?, output_type)
}

struct Config {
    custom_days: Vec<CustomHoliday>,
    language: Option<Language>,
    location: Option<Location>,
}

fn get_config(pass_value: Option<&str>) -> Result<Config, AppError> {
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
        let config: ConfigFile = toml::from_str(&fs::read_to_string(file)?)?;
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
                let date = e.0;
                let printable = e.1;
                let json = e.2;
                let h_date = date.split(&DATE_TOKEN[..]).collect::<Vec<&str>>();

                if h_date.len() != 2 {
                    return Err(AppError::DateSyntaxError(date));
                }
                let (day, month) = if h_date[0].parse::<i8>().is_ok() {
                    (
                        h_date[0]
                            .parse::<i8>()
                            .unwrap_or_else(|e| panic!("{} {} {}", e, file!(), line!())),
                        h_date[1],
                    )
                } else {
                    (
                        h_date[1]
                            .parse::<i8>()
                            .unwrap_or_else(|e| panic!("{} {} {}", e, file!(), line!())),
                        h_date[0],
                    )
                };
                let month = str_to_month(month, true).ok_or_else(|| {
                    AppError::ConfigError(format!("Month {} was unable to be parsed", month))
                })?;

                custom_days.push(CustomHoliday {
                    month,
                    day: NonZeroI8::new(
                        day.try_into()
                            .map_err(|_| AppError::DayIsNotAValidNumber(format!("{}", day)))?,
                    )
                    .ok_or_else(|| AppError::DayIsNotAValidNumber(format!("{}", day)))?,
                    printable,
                    json,
                });
            }
        }
    }
    Ok(Config {
        language,
        custom_days,
        location,
    })
}

fn get_language(config_language: Option<Language>, passed_language: Option<&str>) -> Language {
    if let Some(language) = passed_language {
        match language {
            "en_US" => Language::English,
            "he_IL" => Language::Hebrew,
            x => panic!(format!("Assertion Error! How did {} get here?", x)),
        }
    } else if let Some(language) = config_language {
        language
    } else if let Some(language) = env::vars().find(|x| x.0 == "LANG") {
        if language.1 == "he_IL.UTF-8" {
            Language::Hebrew
        } else {
            Language::English
        }
    } else {
        Language::English
    }
}

fn parse_args(matches: ArgMatches, output_type: OutputType) -> Result<MainArgs, AppError> {
    let config_file = get_config(matches.value_of("configfile"))?;

    let _ = match matches.value_of("type") {
        Some(x) => match x {
            "regular" => Some(OutputType::Regular),
            "pretty" => Some(OutputType::Pretty),
            "json" => Some(OutputType::JSON),
            x => return Err(AppError::TypeError(x.into())),
        },
        None => None,
    };

    let language = get_language(config_file.language, matches.value_of("language"));

    let command = if let Some(matches) = matches.subcommand_matches("list") {
        parse_list_options(matches, &config_file, language, &config_file.custom_days)?
    } else if let Some(matches) = matches.subcommand_matches("convert") {
        let datefmt = if let Some(datefmt) = matches.value_of("DateFormat") {
            match datefmt {
                "ISO" => ConfigDateFmt::ISO,
                "B" => ConfigDateFmt::B,
                "M" => ConfigDateFmt::M,
                "UK" => ConfigDateFmt::UK,
                "L" => ConfigDateFmt::L,
                "US" => ConfigDateFmt::US,
                x => panic!(format!(
                    "Assertion error!, how did DateFormat get a value of {}",
                    x
                )),
            }
        } else {
            ConfigDateFmt::ISO
        };
        parse_convert_options(
            matches.value_of("Date").unwrap(),
            language,
            datefmt,
            match matches.value_of("T").unwrap() {
                "hebrew" => ConfigDateType::Hebrew,
                "gregorian" => ConfigDateType::Gregorian,
                "fuzzy" => ConfigDateType::Fuzzy,
                x => panic!("How did you pass a T of {}", x),
            },
        )?
    } else {
        return Err(AppError::ArgUndefinedError(String::from(matches.usage())));
    };

    Ok(MainArgs {
        custom_days: None,
        output_type,
        language,
        command,
    })
}

fn parse_hebrew(sp: &[&str], language: Language) -> Result<Command, AppError> {
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
    let year = sp[2]
        .parse()
        .map_err(|_| AppError::YearIsNotANumber(sp[2].to_owned()))?;
    let month = str_to_month(sp[1], language == Language::Hebrew)
        .or_else(|| str_to_month(&(String::from(sp[1]).to_lowercase()), false))
        .ok_or_else(|| AppError::MonthNotParsed(sp[1].to_owned()))?;

    Ok(Command::Convert(ConvertArgs {
        date: ConvertType::Hebrew(HebrewDate::from_ymd(year, month, day)?),
    }))
}

fn parse_gregorian(sp: &[&str], format: ConfigDateFmt) -> Result<Command, AppError> {
    let (day, month, year) = match format {
        ConfigDateFmt::ISO | ConfigDateFmt::B => {
            let year = sp[0]
                .parse()
                .map_err(|_| AppError::CannotParseYear(sp[0].into()))?;
            let month = sp[1]
                .parse()
                .map_err(|_| AppError::CannotParseMonth(sp[1].into()))?;
            let day = sp[2]
                .parse()
                .map_err(|_| AppError::CannotParseDay(sp[2].into()))?;
            (day, month, year)
        }
        ConfigDateFmt::US | ConfigDateFmt::M => {
            let year = sp[2]
                .parse()
                .map_err(|_| AppError::CannotParseYear(sp[2].into()))?;
            let month = sp[0]
                .parse()
                .map_err(|_| AppError::CannotParseMonth(sp[0].into()))?;
            let day = sp[1]
                .parse()
                .map_err(|_| AppError::CannotParseDay(sp[1].into()))?;

            (day, month, year)
        }
        ConfigDateFmt::UK | ConfigDateFmt::L => {
            let year = sp[2]
                .parse()
                .map_err(|_| AppError::CannotParseYear(sp[2].into()))?;
            let month = sp[1]
                .parse()
                .map_err(|_| AppError::CannotParseMonth(sp[1].into()))?;
            let day = sp[0]
                .parse()
                .map_err(|_| AppError::CannotParseDay(sp[0].into()))?;

            (day, month, year)
        }
    };
    Ok(Command::Convert(ConvertArgs {
        date: ConvertType::Gregorian(
            Utc.ymd_opt(year, month, day)
                .single()
                .ok_or_else(|| AppError::InvalidGregorianDate(year, month, day))?,
        ),
    }))
}

fn parse_convert_options(
    date: &str,
    language: Language,
    datefmt: ConfigDateFmt,
    date_type: ConfigDateType,
) -> Result<Command, AppError> {
    let sp = date.split(&DATE_TOKEN[..]).collect::<Vec<&str>>();
    if sp.len() != 3 {
        return Err(AppError::SplitDateError);
    }

    Ok(match date_type {
        ConfigDateType::Hebrew => parse_hebrew(&sp, language)?,
        ConfigDateType::Gregorian => parse_gregorian(&sp, datefmt)?,
        ConfigDateType::Fuzzy => {
            if sp[1].parse::<u8>().is_ok() {
                parse_gregorian(&sp, datefmt)?
            } else {
                parse_hebrew(&sp, language)?
            }
        }
    })
}

fn str_to_month(text: &str, exact: bool) -> Option<HebrewMonth> {
    let ret = match text {
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
        _ => None,
    };
    if exact || ret.is_some() {
        return ret;
    }
    match text {
        "tishrei" | "tishre" => Some(HebrewMonth::Tishrei),
        "cheshvan" | "marcheshvan" | "mar cheshvan" => Some(HebrewMonth::Cheshvan),
        "kislev" => Some(HebrewMonth::Kislev),
        "teves" | "tevet" | "teiveis" => Some(HebrewMonth::Teves),
        "shvat" | "shevat" => Some(HebrewMonth::Shvat),
        "adar" => Some(HebrewMonth::Adar),
        "adar1" | "adar 1" | "adar aleph" | "adar rishon" => Some(HebrewMonth::Adar1),
        "adar2" | "adar 2" | "adar beis" | "adar bet" | "adar sheini" => Some(HebrewMonth::Adar2),
        "nissan" | "Nisan" => Some(HebrewMonth::Nissan),
        "iyar" => Some(HebrewMonth::Iyar),
        "sivan" => Some(HebrewMonth::Sivan),
        "tammuz" | "tamuz" => Some(HebrewMonth::Tammuz),
        "av" | "menachem av" => Some(HebrewMonth::Av),
        "elul" | "ellul" => Some(HebrewMonth::Elul),
        _ => None,
    }
}

fn parse_list_options(
    matches: &ArgMatches,
    config: &Config,
    language: Language,
    custom_days: &Vec<CustomHoliday>,
) -> Result<Command, AppError> {
    let year_num = matches
        .value_of("Year")
        .unwrap()
        .parse()
        .expect("The supplied year must be a number");
    let amnt_years = matches
        .value_of("AmountYears")
        .unwrap()
        .parse()
        .expect("The supplied year must be a number");

    let year = match matches.value_of("YearType").unwrap() {
        "hebrew" => YearType::Hebrew(year_num),
        "gregorian" => YearType::Gregorian(year_num),
        "fuzzy" => {
            if year_num > 3000 {
                YearType::Hebrew(year_num)
            } else {
                YearType::Gregorian(year_num)
            }
        }
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let no_sort = matches.occurrences_of("NoSort") > 0;

    let location = if let Some(location) = matches.value_of("Location") {
        str_to_location(location)?
    } else {
        if let Some(location) = env::var_os("LOC") {
            let location: String = location.into_string().unwrap();
            str_to_location(&location)?
        } else if let Some(location) = &config.location {
            *location
        } else {
            if language == Language::Hebrew {
                Location::Israel
            } else {
                Location::Chul
            }
        }
    };

    let events = matches
        .values_of("Events")
        .unwrap_or_else(|| panic!("{}, {}", file!(), line!()))
        .flat_map(|x| match x {
            "yom-tov" => vec![Event::TorahReadingType(TorahReadingType::YomTov)],
            "chol" => vec![Event::TorahReadingType(TorahReadingType::Chol)],
            "shabbos" => vec![Event::TorahReadingType(TorahReadingType::Shabbos)],
            "special-parshas" => vec![Event::TorahReadingType(TorahReadingType::SpecialParsha)],
            "omer" => vec![Event::MinorHoliday(MinorHoliday::Omer)],
            "custom-holidays" => custom_days
                .iter()
                .map(|x| Event::CustomHoliday(x.clone()))
                .collect(),
            "minor-holidays" => vec![Event::MinorHoliday(MinorHoliday::Minor)],
            x => panic!(format!("Assertion Error! How did {} get here?", x)),
        })
        .collect::<Vec<Event>>();
    Ok(Command::List(ListArgs {
        year,
        location,
        events,
        amnt_years,
        no_sort,
    }))
}

fn str_to_location(location: &str) -> Result<Location, AppError> {
    match location.to_lowercase().as_ref() {
        "chul" => Ok(Location::Chul),
        "israel" => Ok(Location::Israel),
        x => Err(AppError::LocationError(x.into())),
    }
}

#[derive(Deserialize)]
struct ConfigFile {
    days: Option<Vec<(String, String, String)>>,
    language: Option<String>,
    location: Option<String>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
enum ConfigDateFmt {
    ISO,
    US,
    UK,
    M,
    L,
    B,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
enum ConfigLocation {
    Israel,
    Chul,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
enum ConfigDateType {
    Hebrew,
    Gregorian,
    Fuzzy,
}
