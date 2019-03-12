use clap::{App, Arg, ArgMatches, SubCommand};

pub mod types;
use crate::args::types::*;
use heca_lib::prelude::*;
use std::env;

pub fn build_args<'a>() -> MainArgs {
    parse_args(App::new("Hebrew Calendar Manipulator")
        .version("0.2.0")
        .about(
            "This program is a fast utility to convert and analyze dates in the Hebrew Calendar.",
        )
        .arg(
            Arg::with_name("configfile")
                .long("config")
                .help("Sets a custom config file (default: $XDG_CONFIG_HOME/heca/config.yaml)")
                .takes_value(true)
                .required(false),
        )
.arg(
            Arg::with_name("type")
                .long("print")
                .help("Set output type")
                .possible_values(&["regular", "pretty", "json"])
                .takes_value(true)
                .default_value("pretty")
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
                        .help("Set date format: US or M for mm/dd/yyyy, UK or L for dd/mm/yyyy, ISO or B for yyyy/mm/dd, or fuzzy")
                        .possible_values(&["US","M","UK","L","ISO","B", "fuzzy"])
                        .takes_value(true)
                        .required(false)
                        .default_value("fuzzy")
                )
                .arg(Arg::with_name("T")
                     .long("type")
                    .long_help("Force conversion from type T, where T is either \"hebrew\" (then date must be written as '5-אדרא-5779'), as \"gregorian\" (where the date must be written as '1996-12-19'), or as \"fuzzy\" (default, where we'll try to figure it out for you, but don't blame us when it breaks!!).")
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
                      .arg(Arg::with_name("Don't sort")
                           .long("nosort")
                           .help("Avoid sorting results")
                           )
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
                           .possible_values(&["yom-tov","shabbos","special-parshas","chol"])
                           .default_value("yom-tov"))
                      .arg(Arg::with_name("Year")
.required(true)
                     .takes_value(true))
                           )
        .get_matches())
}

fn parse_args(matches: ArgMatches) -> MainArgs {
    let config = {
        if let Some(v) = matches.value_of("configfile") {
            Some(String::from(v))
        } else {
            if let Ok(base_dir) = xdg::BaseDirectories::with_prefix("heca") {
                if let Some(path) = base_dir.find_config_file("config.yaml") {
                    Some(String::from(path.to_string_lossy()))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    let output_type = match matches.value_of("type").unwrap() {
        "regular" => OutputType::Regular,
        "pretty" => OutputType::Pretty,
        "json" => OutputType::JSON,
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let language = match matches.value_of("language").unwrap_or("") {
        "en_US" => Language::English,
        "he_IL" => Language::Hebrew,
        "" => {
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
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let command = if let Some(matches) = matches.subcommand_matches("list") {
        parse_list(matches, language == Language::Hebrew)
    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    };

    MainArgs {
        config,
        output_type,
        language,
        command,
    }
}

fn parse_list(matches: &ArgMatches, hebrew: bool) -> Command {
    use atoi::atoi;
    let year_num = atoi::<u64>(matches.value_of("Year").unwrap().as_bytes())
        .expect("The supplied year must be a number");
    let amnt_years = atoi::<u64>(matches.value_of("AmountYears").unwrap().as_bytes())
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

    let shuffle = matches.is_present("nosort");
    let location = match matches.value_of("Location").unwrap_or("") {
        "Chul" => Location::Chul,
        "Israel" => Location::Israel,
        "" => {
            if hebrew {
                Location::Israel
            } else {
                Location::Chul
            }
        }
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let events = matches
        .values_of("Events")
        .unwrap()
        .into_iter()
        .map(|x| match x {
            "yom-tov" => TorahReadingType::YomTov,
            "chol" => TorahReadingType::Chol,
            "shabbos" => TorahReadingType::Shabbos,
            "special-parshas" => TorahReadingType::SpecialParsha,
            x => panic!(format!("Assertion Error! How did {} get here?", x)),
        })
        .collect::<Vec<TorahReadingType>>();
    Command::List(ListArgs {
        year,
        location,
        events,
        shuffle,
        amnt_years,
    })
}
