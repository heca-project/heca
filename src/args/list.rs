use crate::algorithms::candle_lighting::{City, CITIES};
use crate::algorithms::custom_holidays::CustomHoliday;
use crate::algorithms::daily_study::{rambam::RambamChapters, DailyStudy};
use crate::algorithms::minor_days::types::*;
use crate::args::prelude::{str_to_location, Config};
use crate::args::types::{AppError, Command, Event, Language, ListArgs, YearType};
use clap::ArgMatches;
use heca_lib::prelude::{Location, TorahReadingType};

use std::env;
use std::ops::Deref;

pub fn parse_options(
    matches: &ArgMatches<'_>,
    config: &Config,
    language: Language,
    custom_days: &[CustomHoliday],
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
        .expect("Amount of years must be a number");

    let year = if let Some(year_type) = matches.value_of("YearType") {
        match year_type {
            "hebrew" => YearType::Hebrew(year_num),
            "gregorian" => YearType::Gregorian(year_num),
            "fuzzy" => {
                if year_num > 3000 {
                    YearType::Hebrew(year_num)
                } else {
                    YearType::Gregorian(year_num)
                }
            }
            _ => unreachable!(),
        }
    } else if let Some(no_sort) = env::var_os("HECA_YEAR_TYPE") {
        match no_sort.to_string_lossy().deref() {
            "HEBREW" => YearType::Hebrew(year_num),
            "GREGORIAN" => YearType::Gregorian(year_num),
            "FUZZY" => {
                if year_num > 3000 {
                    YearType::Hebrew(year_num)
                } else {
                    YearType::Gregorian(year_num)
                }
            }
            _ => panic!(r#"HECA_YEAR_TYPE must be "HEBREW", "GREGORIAN" or "FUZZY""#),
        }
    } else if year_num > 3000 {
        YearType::Hebrew(year_num)
    } else {
        YearType::Gregorian(year_num)
    };

    let no_sort = if matches.occurrences_of("NoSort") > 0 {
        true
    } else if let Some(no_sort) = env::var_os("HECA_NOSORT") {
        if no_sort == "1" {
            true
        } else {
            false
        }
    } else {
        false
    };

    let exact_days = if matches.occurrences_of("ExactDays") > 0 {
        true
    } else if let Some(exact) = config.exact_days {
        exact
    } else {
        false
    };

    let location = if let Some(location) = matches.value_of("Location") {
        str_to_location(location)?
    } else if let Some(location) = env::var_os("LOC") {
        let location: String = location.into_string().unwrap();
        str_to_location(&location)?
    } else if let Some(location) = env::var_os("HECA_LOCATION") {
        let location: String = location.into_string().unwrap();
        str_to_location(&location)?
    } else if let Some(location) = &config.location {
        *location
    } else if language == Language::Hebrew {
        Location::Israel
    } else {
        Location::Chul
    };

    let city = if let Some(city) = matches.value_of("City") {
        Some(String::from(city))
    } else if let Some(city) = env::var_os("HECA_CITY") {
        Some(city.to_string_lossy().into_owned())
    } else if let Some(city) = &config.default_city {
        Some(city.clone())
    } else {
        None
    };

    let city: Option<City> = if let Some(city) = city {
        let res = config
            .cities
            .as_ref()
            .and_then(|config_city_vec| {
                config_city_vec
                    .iter()
                    .find(|config_city| config_city.name == city)
                    .and_then(|x| Some(x.clone()))
            })
            .and_then(|config_city| {
                Some(City {
                    name: config_city.name.into(),
                    time_zone: config_city.time_zone.clone(),
                    latitude: config_city.latitude,
                    longitude: config_city.longitude,
                    candlelighting_to_sunset: config_city.light_candles_before_shkiya,
                })
            })
            .or_else(|| {
                CITIES.iter().find(|x| city == x.name).and_then(|x| {
                    Some(City {
                        candlelighting_to_sunset: x.candlelighting_to_sunset.clone(),
                        latitude: x.latitude.clone(),
                        longitude: x.longitude.clone(),
                        name: x.name.clone(),
                        time_zone: x.time_zone.clone(),
                    })
                })
            });
        match res {
            Some(res) => Some(res),
            None => return Err(AppError::CityNotFound(city)),
        }
    } else {
        None
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
            "daf-yomi" => vec![Event::DailyStudy(DailyStudy::DafYomi)],
            "yerushalmi-yomi" => vec![Event::DailyStudy(DailyStudy::YerushalmiYomi)],
            "rambam-3-chapters" => {
                vec![Event::DailyStudy(DailyStudy::Rambam(RambamChapters::Three))]
            }
            "rambam-1-chapter" => vec![Event::DailyStudy(DailyStudy::Rambam(RambamChapters::One))],

            "israeli-holidays" => vec![Event::IsraeliHolidays],
            "chabad-holidays" => vec![Event::ChabadHolidays],

            "shabbos-mevarchim" => vec![Event::ShabbosMevarchim],
            _ => unreachable!("{}", x),
        })
        .collect::<Vec<Event>>();
    Ok(Command::List(ListArgs {
        city,
        year,
        location,
        events,
        amnt_years,
        no_sort,
        exact_days,
    }))
}
