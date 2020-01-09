use clap::{App, Arg, ArgMatches, SubCommand};

mod convert;
mod list;
pub(crate) mod prelude;
pub mod types;

use crate::args::prelude::{Config, ConfigDateFmt, ConfigDateType};
use crate::args::types::*;
use std::env;

const DATE_TOKEN: [char; 8] = ['-', '/', '_', '\\', '.', ',', '=', ' '];

pub fn build_args<I, T>(_args: I, output_type: OutputType) -> Result<MainArgs, AppError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    parse_args(App::new("Hebrew calendar program")
                   .version(env!("CARGO_PKG_VERSION"))
                   .about("Heca is a fast utility to convert and list dates in the Hebrew Calendar.")
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
                           .help("Specify if the year is a Hebrew or a Gregorian year. Default is \"fuzzy\"")
                           .possible_values(&["hebrew", "gregorian", "fuzzy"])
                           .takes_value(true)
                           .required(false))
                       .arg(Arg::with_name("NoSort")
                           .long("no-sort")
                           .help("Don't sort output"))
                       .arg(
                           Arg::with_name("ExactDays")
                               .long("exact-days")
                               .help("If showing modern Israeli holidays, show them on their true days, ignoring the recommendation of the Rabbanut to celebrate them early or late to avoid breaking Shabbos.")
                       )
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
                           .default_value("1"))
                       .arg(Arg::with_name("Events")
                       .long("show")
                       .help("What events to list")
                       .takes_value(true)
                       .multiple(true)
                       .required(false)
                       .use_delimiter(true)
                       .possible_values(&[
                           "yom-tov",
                           "shabbos",
                           "special-parshas",
                           "chol",
                           "minor-holidays",
                           "omer",
                           "custom-holidays",
                           "daf-yomi",
                           "yerushalmi-yomi",
                           "rambam-3-chapters",
                           "rambam-1-chapter",
                           "israeli-holidays",
                           "chabad-holidays",
                           "shabbos-mevarchim"
                       ])
                       .default_value("yom-tov"))
                       .arg(Arg::with_name("Year")
                       .required(true)
                       .takes_value(true))).get_matches_safe()?, output_type)
}

fn get_language(config_language: Option<Language>, passed_language: Option<&str>) -> Language {
    if let Some(language) = passed_language {
        match language {
            "en_US" => Language::English,
            "he_IL" => Language::Hebrew,
            _ => unreachable!(),
        }
    } else if let Some(language) = config_language {
        language
    } else if let Ok(language) = env::var("LANG") {
        if language == "he_IL.UTF-8" || language == "he_IL" {
            Language::Hebrew
        } else {
            Language::English
        }
    } else {
        Language::English
    }
}

fn parse_args(matches: ArgMatches<'_>, output_type: OutputType) -> Result<MainArgs, AppError> {
    let config_file = Config::from_location(matches.value_of("configfile"))?;

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
        list::parse_options(matches, &config_file, language, &config_file.custom_days)?
    } else if let Some(matches) = matches.subcommand_matches("convert") {
        let datefmt = if let Some(datefmt) = matches.value_of("DateFormat") {
            match datefmt {
                "ISO" => ConfigDateFmt::ISO,
                "B" => ConfigDateFmt::B,
                "M" => ConfigDateFmt::M,
                "UK" => ConfigDateFmt::UK,
                "L" => ConfigDateFmt::L,
                "US" => ConfigDateFmt::US,
                _ => unreachable!(),
            }
        } else {
            ConfigDateFmt::ISO
        };
        convert::parse_options(
            matches.value_of("Date").unwrap(),
            language,
            datefmt,
            match matches.value_of("T").unwrap() {
                "hebrew" => ConfigDateType::Hebrew,
                "gregorian" => ConfigDateType::Gregorian,
                "fuzzy" => ConfigDateType::Fuzzy,
                _ => unreachable!(),
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
