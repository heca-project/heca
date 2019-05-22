use clap::{App, Arg, ArgMatches, SubCommand};

pub mod types;
use crate::args::types::*;
use atoi::*;
use chrono::prelude::*;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use serde::Deserialize;
use std::env;
use std::fs;

const DATE_TOKEN: [char; 8] = ['-', '/', '_', '\\', '.', ',', '=', ' '];
pub fn build_args<'a, I, T>(_args: I, output_type: OutputType) -> Result<MainArgs, AppError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    parse_args(App::new("Hebrew Calendar Manipulator")
        .version("0.2.0")
        .about(
            "This program is a fast utility to convert and analyze dates in the Hebrew Calendar.",
        )
        .arg(
     Arg::with_name("configfile")
                .long("config")
                .help("Sets a custom config file (default: $XDG_CONFIG_HOME/heca/config.toml)")
                .takes_value(true)
                .required(false),
        ).arg(
            Arg::with_name("type")
                .long("print")
                .help("Set output type")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("language")
                .long("language")
                .help("Set language")
                .possible_values(&["en_US", "he_IL"])
                .takes_value(true)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("convert")
                .about("Converts Hebrew to Gregorian and back")
                .arg(
                    Arg::with_name("DateFormat")
                        .long("datefmt")
                        .help("Set date format (for Gregorian only): US or M for mm/dd/yyyy, UK or L for dd/mm/yyyy, ISO or B for yyyy/mm/dd")
                        .possible_values(&["US","M","UK","L","ISO","B"])
                        .takes_value(true)
                        .required(false)
                        .default_value("ISO")
                )
                .arg(Arg::with_name("T")
                     .long("type")
                    .long_help("Force conversion from type T, where T is either \"hebrew\" (then date must be written as '5/אדרא/5779'), as \"gregorian\" (where the date must be written as '1996/12/19'), or fuzzy (is Hebrew if year is above 4000, Gregorian otherwise).")
                    .possible_values(&["hebrew", "gregorian", "fuzzy"])
                    .takes_value(true)
                    .required(false)
                    .default_value("fuzzy")
        )
                .arg(Arg::with_name("Date")
                     .required(true)
                     .takes_value(true))

         ).subcommand(SubCommand::with_name("list")
                      .arg(Arg::with_name("YearType")
                           .long("type")
                           .help("Specify if the year is a Hebrew or a Gregorian year.")
                           .possible_values(&["hebrew", "gregorian", "fuzzy"])
                           .default_value("fuzzy")
                           .takes_value(true)
                           .required(false)
                      )
                     .arg(Arg::with_name("NoSort")
                          .long("no-sort")
                          .help("Don't sort output."))
                     .arg(Arg::with_name("Location")
                           .long("location")
                           .help("Are you looking for an Israeli calendar or a Chutz La'aretz calendar?")
                           .takes_value(true)
                           .required(false)
                           .possible_values(&["Chul","Israel"]))
                      .arg(Arg::with_name("AmountYears")
                           .long("years")
                           .help("Generate events for n years")
                           .takes_value(true)
                           .required(false)
                           .default_value("1"))
                      .arg(Arg::with_name("Events")
                           .long("show")
                           .help("What events to list")
                           .takes_value(true)
                           .multiple(true)
                           .required(false)
                           .use_delimiter(true)
                           .possible_values(&["yom-tov","shabbos","special-parshas","chol","minor-holidays", "omer", "custom-holidays"])
                           .default_value("yom-tov"))
                           .arg(Arg::with_name("Year")
                      .required(true)
                      .takes_value(true))
                           )
        .get_matches_safe()?, output_type)
}

fn parse_args(matches: ArgMatches, output_type: OutputType) -> Result<MainArgs, AppError> {
    let config_file = {
        if let Some(v) = matches.value_of("configfile") {
            Some(String::from(v))
        } else {
            if let Ok(base_dir) = xdg::BaseDirectories::with_prefix("heca") {
                if let Some(path) = base_dir.find_config_file("config.toml") {
                    Some(String::from(path.to_string_lossy()))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    let mut custom_days: Vec<CustomHoliday> = Vec::new();
    if let Some(ref file) = config_file {
        let config: Config = toml::from_str(&fs::read_to_string(file)?)?;
        if let Some(days) = config.days {
            for e in days {
                let date = e.0;
                let printable = e.1;
                let json = e.2;
                let h_date = date.split(&DATE_TOKEN[..]).collect::<Vec<&str>>();

                if h_date.len() != 2 {
                    return Err(AppError::ConfigError(format!(
                        "Date {} was unable to be parsed",
                        date
                    )));
                }
                let (day, month) = if h_date[0].parse::<u8>().is_ok() {
                    (
                        h_date[0].parse::<u8>().expect(&format!("{}", line!())),
                        h_date[1],
                    )
                } else {
                    (
                        h_date[1].parse::<u8>().expect(&format!("{}", line!())),
                        h_date[0],
                    )
                };
                let month = str_to_month(month, true).ok_or(AppError::ConfigError(format!(
                    "Month {} was unable to be parsed",
                    month
                )))?;
                custom_days.push(CustomHoliday {
                    month,
                    day,
                    printable,
                    json,
                });
            }
        }
    }

    let _ = match matches.value_of("type") {
        Some(x) => match x {
            "regular" => Some(OutputType::Regular),
            "pretty" => Some(OutputType::Pretty),
            "json" => Some(OutputType::JSON),
            x => return Err(AppError::TypeError(x.into())),
        },
        None => None,
    };

    let language = match matches.value_of("language") {
        None => {
            let lang = env::vars().into_iter().find(|x| x.0 == "LANG");
            match lang {
                None => Language::English,
                Some(x) => {
                    if x.1 == "he_IL.UTF-8" {
                        Language::Hebrew
                    } else {
                        Language::English
                    }
                }
            }
        }
        Some(language) => match language {
            "en_US" => Language::English,
            "he_IL" => Language::Hebrew,
            x => panic!(format!("Assertion Error! How did {} get here?", x)),
        },
    };

    let command = if let Some(matches) = matches.subcommand_matches("list") {
        parse_list(matches, language == Language::Hebrew, custom_days)?
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
        parse_convert(
            matches
                .value_of("Date")
                .expect(&format!("{}", line!()))
                .into(),
            language == Language::Hebrew,
            datefmt,
            match matches.value_of("T").expect(&format!("{}", line!())) {
                "hebrew" => ConfigDateType::Hebrew,
                "gregorian" => ConfigDateType::Gregorian,
                "fuzzy" => ConfigDateType::Fuzzy,
                x => panic!(format!("How did you pass a T of {}", x)),
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

fn parse_hebrew(sp: &[&str], is_hebrew: bool) -> Result<Command, AppError> {
    let day = atoi::<u8>(sp[0].as_bytes()).ok_or(AppError::DayIsNotANumber(sp[0].to_owned()))?;
    let year = atoi::<u64>(sp[2].as_bytes()).ok_or(AppError::YearIsNotANumber(sp[2].to_owned()))?;
    let month = {
        let mut a = str_to_month(sp[1], is_hebrew);
        if !a.is_some() {
            a = str_to_month(&(String::from(sp[1]).to_lowercase()), false);
        }
        a
    }
    .ok_or(AppError::MonthNotParsed(sp[1].to_owned()))?;
    Ok(Command::Convert(ConvertArgs {
        date: ConvertType::Hebrew(HebrewDate::from_ymd(year, month, day)?),
    }))
}

fn parse_gregorian(sp: &[&str], format: ConfigDateFmt) -> Result<Command, AppError> {
    let (day, month, year) = match format {
        ConfigDateFmt::ISO | ConfigDateFmt::B => {
            let year =
                atoi::<i32>(sp[0].as_bytes()).ok_or(AppError::CannotParseYear(sp[0].into()))?;
            let month =
                atoi::<u32>(sp[1].as_bytes()).ok_or(AppError::CannotParseMonth(sp[1].into()))?;
            let day =
                atoi::<u32>(sp[2].as_bytes()).ok_or(AppError::CannotParseDay(sp[2].into()))?;
            (day, month, year)
        }
        ConfigDateFmt::US | ConfigDateFmt::M => {
            let year =
                atoi::<i32>(sp[2].as_bytes()).ok_or(AppError::CannotParseYear(sp[2].into()))?;
            let month =
                atoi::<u32>(sp[0].as_bytes()).ok_or(AppError::CannotParseMonth(sp[0].into()))?;
            let day =
                atoi::<u32>(sp[1].as_bytes()).ok_or(AppError::CannotParseDay(sp[1].into()))?;

            (day, month, year)
        }
        ConfigDateFmt::UK | ConfigDateFmt::L => {
            let year =
                atoi::<i32>(sp[2].as_bytes()).ok_or(AppError::CannotParseYear(sp[2].into()))?;
            let month =
                atoi::<u32>(sp[1].as_bytes()).ok_or(AppError::CannotParseMonth(sp[1].into()))?;
            let day =
                atoi::<u32>(sp[0].as_bytes()).ok_or(AppError::CannotParseDay(sp[0].into()))?;

            (day, month, year)
        }
    };
    Ok(Command::Convert(ConvertArgs {
        date: ConvertType::Gregorian(
            Utc.ymd_opt(year, month, day)
                .single()
                .ok_or(AppError::InvalidGregorianDate(year, month, day))?,
        ),
    }))
}
fn parse_convert(
    date: String,
    hebrew_lang: bool,
    datefmt: ConfigDateFmt,
    date_type: ConfigDateType,
) -> Result<Command, AppError> {
    let sp = date.split(&DATE_TOKEN[..]).collect::<Vec<&str>>();
    if sp.len() != 3 {
        return Err(AppError::SplitDateError);
    }

    Ok(match date_type {
        ConfigDateType::Hebrew => parse_hebrew(&sp, hebrew_lang)?,
        ConfigDateType::Gregorian => parse_gregorian(&sp, datefmt)?,
        ConfigDateType::Fuzzy => {
            if sp[1].parse::<u8>().is_ok() {
                parse_gregorian(&sp, datefmt)?
            } else {
                parse_hebrew(&sp, hebrew_lang)?
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

fn parse_list(
    matches: &ArgMatches,
    hebrew: bool,
    custom_days: Vec<CustomHoliday>,
) -> Result<Command, AppError> {
    use atoi::atoi;
    let year_num = atoi::<u64>(
        matches
            .value_of("Year")
            .expect(&format!("{}", line!()))
            .as_bytes(),
    )
    .expect("The supplied year must be a number");
    let amnt_years = atoi::<u64>(
        matches
            .value_of("AmountYears")
            .expect(&format!("{}", line!()))
            .as_bytes(),
    )
    .expect("The supplied year must be a number");

    let year = match matches.value_of("YearType").expect(&format!("{}", line!())) {
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

    let location = match matches.value_of("Location") {
        None => {
            if hebrew {
                Location::Israel
            } else {
                Location::Chul
            }
        }
        Some(location) => match location {
            "Chul" => Location::Chul,
            "Israel" => Location::Israel,
            x => panic!(format!("Assertion Error! How did {} get here?", x)),
        },
    };

    let events = matches
        .values_of("Events")
        .expect(&format!("{}", line!()))
        .into_iter()
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

#[derive(Deserialize)]
struct Config {
    days: Option<Vec<(String, String, String)>>,
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
