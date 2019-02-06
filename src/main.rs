extern crate atoi;
extern crate chrono;
extern crate chrono_english;
extern crate heca_lib;
extern crate time;

#[macro_use]
extern crate clap;
use clap::App;

use atoi::atoi;
use chrono::prelude::*;
use heca_lib::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(command) = matches.subcommand_matches("convert") {
        if let Err(e) = convert(command.value_of("date").unwrap()) {
            match e {
                InputError::DateFormatError => println!("I can't parse the date you gave"),
                InputError::TooManyMonths => println!("I need only one hebrew month"),
                InputError::NotEnoughMonths => println!("I need one hebrew month"),
                InputError::WrongAmntHebDateOptions => println!("I need something looking like Month-Day-Year or Day-Month-Year (For example: 10 Shvat 5710)."),
                InputError::ConversionError(s) => println!("{}",s),
            }
        }
    }
}

fn convert(date_str: &str) -> Result<(), InputError> {
    if let Ok(heb_date) = convert_date_to_fuzzy_hebrew(date_str) {
        let eng_date = HebrewDate::to_eng(heb_date).map_err(|x| InputError::ConversionError(x))?;
        println!("{}", eng_date);
        Ok(())
    } else if let Ok(eng_date) = convert_date_to_fuzzy_gregorian(&date_str) {
        let heb_date =
            HebrewDate::from_eng(eng_date).map_err(|x| InputError::ConversionError(x))?;
        println!(
            "{} {} {}",
            heb_date.day(),
            heb_date.month(),
            heb_date.year()
        );
        Ok(())
    } else {
        Err(InputError::DateFormatError)
    }
}

fn convert_date_to_fuzzy_hebrew(date: &str) -> Result<HebrewDate, InputError> {
    let mut v = date
        .split(|x: char| x == '-' || x == '_' || x == '/' || x == ' ')
        .collect::<Vec<&str>>();
    if v.len() != 3 {
        return Err(InputError::WrongAmntHebDateOptions);
    }
    let hebrew_month = v
        .iter()
        .map(|x| String::from(*x))
        .enumerate()
        .filter_map(|(a, x)| {
            if HebrewMonth::month_list()
                .iter()
                .filter(|y| x.to_uppercase() == String::from(**y).to_uppercase())
                .count()
                == 1
            {
                Some((a, x))
            } else {
                None
            }
        })
        .collect::<Vec<(usize, String)>>();
    if hebrew_month.len() == 0 {
        return Err(InputError::NotEnoughMonths);
    } else if hebrew_month.len() > 1 {
        return Err(InputError::TooManyMonths);
    }
    v.remove(hebrew_month[0].0);
    let int = v
        .iter()
        .filter_map(|x| atoi::<i64>(x.as_bytes()))
        .collect::<Vec<i64>>();
    let (days, years) = if int[0] > 100 {
        (int[1], int[0])
    } else {
        (int[0], int[1])
    };
    HebrewDate::from_ymd(
        years,
        HebrewMonth::try_from(&(hebrew_month[0].1)).unwrap(),
        days,
    )
    .map_err(|x| InputError::ConversionError(x))
}
fn convert_date_to_fuzzy_gregorian(
    date: &str,
) -> Result<chrono::DateTime<Utc>, chrono_english::DateError> {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use chrono_english::{parse_date_string, Dialect};
    use std::time::{SystemTime, UNIX_EPOCH};
    let cur_dt = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(
                {
                    let start = SystemTime::now();
                    let time = start.duration_since(UNIX_EPOCH).unwrap();
                    time.as_secs() as i64
                },
                0,
            ),
            Utc,
        );
    let res  = parse_date_string(
        date,
        Utc.ymd(cur_dt.year(), cur_dt.month(), cur_dt.day()).and_hms(1,0,0),
        Dialect::Us,
    );
    res
}

enum InputError {
    DateFormatError,
    TooManyMonths,
    NotEnoughMonths,
    WrongAmntHebDateOptions,
    ConversionError(ConversionError),
}
