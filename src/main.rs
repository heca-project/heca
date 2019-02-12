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
use heca_lib::holidays::*;
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
                InputError::DayOverflowError => println!("The day you selected is out of range"),
            }
        }
    } else if let Some(command) = matches.subcommand_matches("list") {
        if let Err(e) = list(command.value_of("year")) {
            panic!(e);
        }
    }
}

fn list(year: Option<&str>) -> Result<(), InputError> {
    let year = if let Some(year) = year {
        atoi::<u64>(year.as_bytes()).ok_or(InputError::DateFormatError)?
    } else {
        let utc: DateTime<Local> = Local::now();
        utc.year() as u64
    };

    let final_list = if year < 3000 {
        let jan_1_orig_year =
            HebrewDate::from_gregorian(Utc.ymd(year as i32, 1, 1).and_hms(0, 0, 0)).unwrap();
        let jan_1_next_year =
            HebrewDate::from_gregorian(Utc.ymd((year + 1) as i32, 1, 1).and_hms(0, 0, 0)).unwrap();
        let mut yt_list = get_yt_list(jan_1_orig_year.year());
        yt_list.append(&mut get_yt_list(jan_1_next_year.year()));
        yt_list.append(&mut get_torah_reading_days_list(jan_1_orig_year.year()));
        yt_list.append(&mut get_torah_reading_days_list(jan_1_next_year.year()));
        yt_list.sort();

        yt_list
            .into_iter()
            .filter(|x| (x).day() >= jan_1_orig_year && (x).day() < jan_1_next_year)
            .collect()
    } else {
        let mut yt_list = get_yt_list(year);
        yt_list.append(&mut get_torah_reading_days_list(year));
        yt_list.sort();
        yt_list
    };
    final_list
        .iter()
        .for_each(|x| println!("{} {}", x.day().to_gregorian(), x.name()));
    Ok(())
}

fn convert(date_str: &str) -> Result<(), InputError> {
    if let Ok(heb_date) = convert_date_to_fuzzy_hebrew(date_str) {
        let eng_date = heb_date.to_gregorian();
        println!("{}", eng_date);
        Ok(())
    } else if let Ok(eng_date) = convert_date_to_fuzzy_gregorian(&date_str) {
        let heb_date =
            HebrewDate::from_gregorian(eng_date).map_err(|x| InputError::ConversionError(x))?;
        println!(
            "{} -> {} {} {}",
            eng_date,
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
    let mut hebrew_month = v
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
        .filter_map(|x| atoi::<u64>(x.as_bytes()))
        .collect::<Vec<u64>>();
    let (days, years) = if int[0] > 100 {
        (int[1], int[0])
    } else {
        (int[0], int[1])
    };

    (hebrew_month[0].1) = (hebrew_month[0].1).to_lowercase();
    &(hebrew_month[0].1)
        .get_mut(0..1)
        .unwrap()
        .make_ascii_uppercase();
    if days > std::u8::MAX.into() {
        return Err(InputError::DayOverflowError);
    }
    HebrewDate::from_ymd(
        years,
        HebrewMonth::try_from(&(hebrew_month[0].1)).unwrap(),
        days as u8,
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
    let res = parse_date_string(
        date,
        Utc.ymd(cur_dt.year(), cur_dt.month(), cur_dt.day())
            .and_hms(1, 0, 0),
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
    DayOverflowError,
}
