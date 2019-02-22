extern crate atoi;
extern crate chrono;
extern crate chrono_english;
extern crate cpuprofiler;
extern crate heca_lib;
extern crate itoa;
extern crate time;
use cpuprofiler::PROFILER;

#[macro_use]
extern crate clap;
use clap::App;

use atoi::atoi;
use chrono::prelude::*;
use chrono::Duration;
use heca_lib::*;
use std::io::BufWriter;
use std::io::{self, Write};

fn main() {
    PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(command) = matches.subcommand_matches("convert") {
        let force_hebrew = command.is_present("hebrew");
        let force_gregorian = command.is_present("gregorian");
        if force_hebrew && force_gregorian {
            println!("You can force either Hebrew or Gregorian, not both.");
            std::process::exit(1);
        }
        let convert_type = if force_hebrew {
            ConvertType::FromHebrew
        } else if force_gregorian {
            ConvertType::FromGregorian
        } else {
            ConvertType::FromFuzzy
        };
        if let Err(e) = convert(command.value_of("date").unwrap(), convert_type) {
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
        let year = if let Some(year) = command.value_of("year") {
            atoi::<u64>(year.as_bytes()).expect(&format!("I can't treat {} as a year", year))
        } else {
            Local::now().year() as u64
        };

        let n = if let Some(n) = command.value_of("years") {
            atoi::<u64>(n.as_bytes()).expect(&format!("I can't treat {} as a number", n))
        } else {
            1
        };
        if let Err(e) = list(year, n, year < 4000) {
            panic!(e);
        }
    }
    PROFILER.lock().unwrap().stop().unwrap();
}

fn list(year: u64, n: u64, is_english: bool) -> Result<(), InputError> {
    let stdout = io::stdout();
    let mut lock = BufWriter::with_capacity(100_000, stdout.lock());
    let mut year = year;
    for _i in 0..n {
        let final_list = if is_english {
            let jan_1_orig_year =
                HebrewDate::from_gregorian(Utc.ymd(year as i32, 1, 1).and_hms(0, 0, 0)).unwrap();
            let jan_1_next_year =
                HebrewDate::from_gregorian(Utc.ymd((year + 1) as i32, 1, 1).and_hms(0, 0, 0))
                    .unwrap();

            let this_year = HebrewYear::new(jan_1_orig_year.year()).unwrap();
            let next_year = HebrewYear::new(jan_1_next_year.year()).unwrap();
            let mut yt_list = this_year.get_holidays(YomTovType::YomTov);
            yt_list.append(&mut this_year.get_holidays(YomTovType::SpecialTorahReading));
            yt_list.append(&mut next_year.get_holidays(YomTovType::YomTov));
            yt_list.append(&mut next_year.get_holidays(YomTovType::SpecialTorahReading));
            yt_list.sort();

            yt_list
                .into_iter()
                .filter(|x| (x).day() >= jan_1_orig_year && (x).day() < jan_1_next_year)
                .collect()
        } else {
            let year = HebrewYear::new(year).unwrap();
            let mut yt_list = year.get_holidays(YomTovType::YomTov);
            yt_list.append(&mut year.get_holidays(YomTovType::SpecialTorahReading));
            yt_list.sort();
            yt_list
        };

        final_list
            .iter()
            .map(|x| {
                let ret = x.day().to_gregorian();
                (ret.year(), ret.month(), ret.day(), x.name().as_bytes())
            })
            .for_each(|(year, month, day, name)| {
                let mut year_arr = [b'\0'; 16];
                let mut month_arr = [b'\0'; 2];
                let mut day_arr = [b'\0'; 2];
                let count_y = itoa::write(&mut year_arr[..], year).unwrap();
                let count_m = itoa::write(&mut month_arr[..], month).unwrap();
                let count_d = itoa::write(&mut day_arr[..], day).unwrap();
                lock.write(&year_arr[..count_y as usize]).unwrap();
                lock.write(b"/").unwrap();
                lock.write(&month_arr[..count_m as usize]).unwrap();
                lock.write(b"/").unwrap();
                lock.write(&day_arr[..count_d as usize]).unwrap();
                lock.write(b" ").unwrap();
                lock.write(name).unwrap();
                lock.write(b"\n").unwrap();
            });
        year += 1;
    }
    Ok(())
}

fn convert(date_str: &str, t: ConvertType) -> Result<(), InputError> {
    match t {
        ConvertType::FromHebrew => {
            let heb_date = convert_date_to_fuzzy_hebrew(date_str)?;
            let eng_date = heb_date.to_gregorian();
            println!(
                "{} => {} - {}",
                heb_date,
                eng_date.format("%m/%d/%y"),
                (eng_date + Duration::days(1)).format("%m/%d/%y"),
            );
            Ok(())
        }
        ConvertType::FromGregorian => {
            let vec = date_str.split("-").collect::<Vec<&str>>();
            if vec.len() < 3 {
                return Err(InputError::DateFormatError);
            }

            let v: Vec<u64> = vec
                .iter()
                .filter_map(|x| atoi::<u64>(x.as_bytes()))
                .collect();
            if v.len() < 3 {
                return Err(InputError::DateFormatError);
            }

            let heb_date1 = HebrewDate::from_gregorian(
                Utc.ymd(v[0] as i32, v[1] as u32, v[2] as u32)
                    .and_hms(0, 0, 0),
            )
            .map_err(|x| InputError::ConversionError(x))?;
            let heb_date2 = HebrewDate::from_gregorian(
                Utc.ymd(v[0] as i32, v[1] as u32, v[2] as u32)
                    .and_hms(23, 59, 59),
            )
            .map_err(|x| InputError::ConversionError(x))?;
            println!("{}/{}/{} => {}-{}", v[0], v[1], v[2], heb_date1, heb_date2);
            Ok(())
        }
        ConvertType::FromFuzzy => {
            if let Ok(heb_date) = convert_date_to_fuzzy_hebrew(date_str) {
                let eng_date = heb_date.to_gregorian();
                println!("{}", eng_date);
                Ok(())
            } else if let Ok(eng_date) = convert_date_to_fuzzy_gregorian(&date_str) {
                let heb_date = HebrewDate::from_gregorian(eng_date)
                    .map_err(|x| InputError::ConversionError(x))?;
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

enum ConvertType {
    FromGregorian,
    FromHebrew,
    FromFuzzy,
}
