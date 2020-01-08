use crate::args::types::{DayVal, Language, Name};
use crate::prelude::{hebrew_month_english, hebrew_month_hebrew};
use chrono::{DateTime, Datelike, Duration, Timelike, Utc, Weekday};
use heca_lib::prelude::HebrewMonth;
use heca_lib::{HebrewDate, HebrewYear};
use serde::ser::*;
use serde::{Serialize, Serializer};
use std::convert::TryInto;
use std::io::{BufWriter, StdoutLock, Write};
use std::num::NonZeroI8;

pub fn get(year: &HebrewYear) -> Vec<DayVal> {
    let mut v = Vec::new();
    let last_day_of_month: Vec<HebrewDate> = {
        let mut v = vec![
            year.get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Cheshvan, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Shvat, NonZeroI8::new(29).unwrap())
                .unwrap(),
            {
                if year.is_leap_year() {
                    year.get_hebrew_date(HebrewMonth::Adar2, NonZeroI8::new(29).unwrap())
                        .unwrap()
                } else {
                    year.get_hebrew_date(HebrewMonth::Adar, NonZeroI8::new(29).unwrap())
                        .unwrap()
                }
            },
            year.get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Tammuz, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(29).unwrap())
                .unwrap(),
            year.get_hebrew_date(HebrewMonth::Elul, NonZeroI8::new(29).unwrap())
                .unwrap(),
        ];

        if year.is_leap_year() {
            v.push(
                year.get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(29).unwrap())
                    .unwrap(),
            );
        }
        v
    };
    let shabbos_mevarchim: Vec<(DateTime<Utc>, HebrewMonth)> = last_day_of_month
        .into_iter()
        .map(|x| {
            let gregorian_date: DateTime<Utc> = x.try_into().unwrap();
            let offset = match gregorian_date.weekday() {
                Weekday::Fri => 0,
                Weekday::Thu => 6,
                Weekday::Wed => 5,
                Weekday::Tue => 4,
                Weekday::Mon => 3,
                Weekday::Sun => 2,
                Weekday::Sat => 1,
            };

            let m = match x.month() {
                HebrewMonth::Tishrei => HebrewMonth::Cheshvan,
                HebrewMonth::Cheshvan => HebrewMonth::Kislev,
                HebrewMonth::Kislev => HebrewMonth::Teves,
                HebrewMonth::Teves => HebrewMonth::Shvat,
                HebrewMonth::Shvat => {
                    if year.is_leap_year() {
                        HebrewMonth::Adar1
                    } else {
                        HebrewMonth::Adar
                    }
                }
                HebrewMonth::Adar => HebrewMonth::Nissan,
                HebrewMonth::Adar1 => HebrewMonth::Adar2,
                HebrewMonth::Adar2 => HebrewMonth::Nissan,
                HebrewMonth::Nissan => HebrewMonth::Iyar,
                HebrewMonth::Iyar => HebrewMonth::Sivan,
                HebrewMonth::Sivan => HebrewMonth::Tammuz,
                HebrewMonth::Tammuz => HebrewMonth::Av,
                HebrewMonth::Av => HebrewMonth::Elul,
                HebrewMonth::Elul => HebrewMonth::Tishrei,
            };
            (gregorian_date - Duration::days(offset), m)
        })
        .collect();
    for num in shabbos_mevarchim {
        let cur_month = num.1;
        let molad = year.get_molad(cur_month).unwrap();
        let molad_day = molad.get_day_utc();
        v.push(DayVal {
            day: num.0,
            name: Name::ShabbosMevarchim(ShabbosMevarchim {
                hebrew_month: cur_month,
                gregorian_month: molad_day.month(),
                gregorian_dow: molad_day.weekday(),
                gregorian_day: molad_day.day(),
                hour: molad_day.hour(),
                minute: molad_day.minute(),
                chalakim: molad.get_chalakim(),
            }),
        });
    }
    v
}

#[derive(Debug, Clone)]
pub struct ShabbosMevarchim {
    pub hebrew_month: HebrewMonth,
    pub gregorian_month: u32,
    pub gregorian_dow: Weekday,
    pub gregorian_day: u32,
    pub hour: u32,
    pub minute: u32,
    pub chalakim: u16,
}

impl Serialize for ShabbosMevarchim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ShabbosMevarchim", 2)?;
        state.serialize_field("month", &self.gregorian_month)?;
        state.serialize_field(
            "weekday",
            match self.gregorian_dow {
                Weekday::Sun => "Sunday",
                Weekday::Mon => "Monday",
                Weekday::Tue => "Tuesday",
                Weekday::Wed => "Wednesday",
                Weekday::Thu => "Thursday",
                Weekday::Fri => "Friday",
                Weekday::Sat => "Shabbos",
            },
        )?;
        state.serialize_field("hour", &self.hour)?;
        state.serialize_field("minute", &self.minute)?;
        state.serialize_field("chalakim", &self.chalakim)?;
        state.end()
    }
}

impl ShabbosMevarchim {
    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let p = match language {
            Language::English => {
                let mut p = lock.write(b"Shabbos Mevarchim ").ok()?;
                p += lock
                    .write(hebrew_month_english(self.hebrew_month).as_ref())
                    .ok()?;
                let gregorian_month = match self.gregorian_month {
                    1 => "January",
                    2 => "February",
                    3 => "March",
                    4 => "April",
                    5 => "May",
                    6 => "June",
                    7 => "July",
                    8 => "August",
                    9 => "September",
                    10 => "October",
                    11 => "November",
                    12 => "December",
                    _ => unreachable!(),
                };
                let mut gregorian_day = [b'\0'; 2];
                let mut hour_arr = [b'\0'; 2];
                let mut minute_arr = [b'\0'; 2];
                let mut chalakim_arr = [b'\0'; 2];
                let count_gd = itoa::write(&mut gregorian_day[..], self.gregorian_day).unwrap();
                let count_h = itoa::write(&mut hour_arr[..], self.hour).unwrap();
                let count_m = itoa::write(&mut minute_arr[..], self.minute).unwrap();
                let count_chalakim = itoa::write(&mut chalakim_arr[..], self.chalakim).unwrap();
                p += lock.write(b": Molad is on ").ok()?;
                p += match self.gregorian_dow {
                    Weekday::Sun => lock.write(b"Sunday").ok()?,
                    Weekday::Mon => lock.write(b"Monday").ok()?,
                    Weekday::Tue => lock.write(b"Tuesday").ok()?,
                    Weekday::Wed => lock.write(b"Wednesday").ok()?,
                    Weekday::Thu => lock.write(b"Thursday").ok()?,
                    Weekday::Fri => lock.write(b"Friday").ok()?,
                    Weekday::Sat => lock.write(b"Shabbos").ok()?,
                };
                lock.write(b" ").ok()?;
                p += lock.write(gregorian_month.as_bytes()).ok()?;
                p += lock.write(b" ").ok()?;
                p += lock.write(&gregorian_day[..count_gd]).ok()?;
                p += lock.write(b", ").ok()?;
                p += lock.write(&hour_arr[..count_h]).ok()?;
                p += lock.write(b":").ok()?;
                if count_m == 1 {
                    p += lock.write(b"0").ok()?;
                }
                p += lock.write(&minute_arr[..count_m]).ok()?;
                p += lock.write(b" and ").ok()?;
                p += lock.write(&chalakim_arr[..count_chalakim]).ok()?;
                p += lock.write(b" Chalakim").ok()?;
                p
            }
            Language::Hebrew => {
                let mut p = lock.write("שבת מברכים ".as_bytes()).ok()?;
                p += lock
                    .write(hebrew_month_hebrew(self.hebrew_month).as_ref())
                    .ok()?;
                let mut gregorian_day = [b'\0'; 2];
                let mut hour_arr = [b'\0'; 2];
                let mut minute_arr = [b'\0'; 2];
                let mut chalakim_arr = [b'\0'; 2];
                let count_gd = itoa::write(&mut gregorian_day[..], self.gregorian_day).unwrap();
                let count_h = itoa::write(&mut hour_arr[..], self.hour).unwrap();
                let count_m = itoa::write(&mut minute_arr[..], self.minute).unwrap();
                let count_chalakim = itoa::write(&mut chalakim_arr[..], self.chalakim).unwrap();
                p += lock.write(" זמן המולד: יום ".as_bytes()).ok()?;
                p += match self.gregorian_dow {
                    Weekday::Sun => lock.write("ראשון".as_bytes()).ok()?,
                    Weekday::Mon => lock.write("שני".as_bytes()).ok()?,
                    Weekday::Tue => lock.write("שלישי".as_bytes()).ok()?,
                    Weekday::Wed => lock.write("רביעי".as_bytes()).ok()?,
                    Weekday::Thu => lock.write("חמישי".as_bytes()).ok()?,
                    Weekday::Fri => lock.write("שישי".as_bytes()).ok()?,
                    Weekday::Sat => lock.write("שבת קודש".as_bytes()).ok()?,
                };
                p += lock.write(b" ").ok()?;
                p += lock.write(&gregorian_day[..count_gd]).ok()?;
                p += lock.write(b", ").ok()?;
                p += lock.write(&hour_arr[..count_h]).ok()?;
                p += lock.write(b":").ok()?;
                if count_m == 1 {
                    p += lock.write(b"0").ok()?;
                }
                p += lock.write(&minute_arr[..count_m]).ok()?;
                p += lock.write(" ו ".as_bytes()).ok()?;
                p += lock.write(&chalakim_arr[..count_chalakim]).ok()?;
                p += lock.write(" חלקים".as_bytes()).ok()?;
                p
            }
        };
        Some(p)
    }
}
