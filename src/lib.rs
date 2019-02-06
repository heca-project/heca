extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate time;
use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt;
use time::Duration;
#[macro_use]
extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

// ChalakimBetweenMolad is the amount of Chalakim between two Molads
// See https://www.chabad.org/library/article_cdo/aid/947923/jewish/Kiddush-HaChodesh-Chapter-Six.htm#footnoteRef8a947923
const CHALAKIM_PER_HOUR: i64 = 1080;
const CHALAKIM_BETWEEN_MOLAD: i64 = 29 * 24 * CHALAKIM_PER_HOUR + 12 * CHALAKIM_PER_HOUR + 793;
const LEAP_YEARS: [bool; 19] = [
    false, false, true, false, false, true, false, true, false, false, true, false, false, true,
    false, false, true, false, true,
];
//var Epoch = time.Date(1, time.September, 30, 18, 0, 0, 0, time.UTC)
// FirstMolad (of Tishrei 3672) was on Sunday, October 1st at 17:09:12 Chalakim
const FIRST_MOLAD: i64 = 2 * 24 * 1080 + 20 * 1080 + (43 * 1080 / 60) - 11;
const FIRST_YEAR: i64 = 3762;
lazy_static! {
    static ref EPOCH: chrono::DateTime<Utc> = Utc.ymd(1, 9, 4).and_hms(18, 0, 0);
}

lazy_static! {
    static ref YEAR_SCHED: HashMap<i64, [i8; 14]> = {
        let mut m = HashMap::new();
        m.insert(353, [30, 29, 29, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29]);
        m.insert(354, [30, 29, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29]);
        m.insert(355, [30, 30, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29]);

        m.insert(383, [30, 29, 29, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29]);
        m.insert(384, [30, 29, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29]);
        m.insert(385, [30, 30, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29]);
        m
    };
}

lazy_static! {
    static ref AMNT_CHALAKIM_PER_CYCLE: i64 = {
        let mut chalakim = 0;
        for i in LEAP_YEARS.iter() {
            if *i {
                chalakim += CHALAKIM_BETWEEN_MOLAD * 13;
            } else {
                chalakim += CHALAKIM_BETWEEN_MOLAD * 12;
            }
        }
        chalakim
    };
}

fn get_molad_for_year(year: i64) -> i64 {
    let amnt_of_cycles = (year - FIRST_YEAR) / 19;

    //full_cycle_chalakim = (7 full years)(13 month/year)(ChalakimBetweenMolad) + (12 short years)(12 months/year)(ChalakimBetweenMolad)
    let full_cycle_chalakim = 7 * 13 * CHALAKIM_BETWEEN_MOLAD + 12 * 12 * CHALAKIM_BETWEEN_MOLAD;

    let mut amnt_chalakim = full_cycle_chalakim * amnt_of_cycles;
    let cur_year_in_cycle = (year - FIRST_YEAR) % 19;
    for i in 0..cur_year_in_cycle {
        amnt_chalakim += months_per_year(i) * CHALAKIM_BETWEEN_MOLAD;
    }

    return amnt_chalakim;
}

fn months_per_year(year_in_cycle: i64) -> i64 {
    return if LEAP_YEARS[year_in_cycle as usize] {
        13
    } else {
        12
    };
}

fn get_rosh_hashana(year: i64) -> i64 {
    let amnt_chalakim_since_first_molad = get_molad_for_year(year);
    let amnt_chalakim_since_epoch = amnt_chalakim_since_first_molad + FIRST_MOLAD;
    let mut amnt_days = amnt_chalakim_since_epoch / (CHALAKIM_PER_HOUR * 24);
    let amnt_chalakim = amnt_chalakim_since_epoch % (CHALAKIM_PER_HOUR * 24);
    let mut reg_postpone = false;
    //If the Molad is in the afternoon, postpone Rosh Hashana by a day
    if amnt_chalakim >= 18 * CHALAKIM_PER_HOUR {
        amnt_days += 1;
        reg_postpone = true;
    }

    let mut dow = (amnt_days) % 7;
    // Lo Adu Rosh

    if dow == 0 || dow == 3 || dow == 5 {
        amnt_days += 1;
        reg_postpone = true;
    }
    dow = (amnt_days) % 7;

    // See Hilchos Kiddush HaChodesh Halacha 4

    if !reg_postpone
        && dow == 2
        && amnt_chalakim > 9 * CHALAKIM_PER_HOUR + 204
        && months_per_year((year - FIRST_YEAR) % 19) == 12
    {
        amnt_days += 2;
    }

    if !reg_postpone
        && months_per_year((year - FIRST_YEAR - 1) % 19) == 13
        && dow == 1
        && amnt_chalakim > 12 * CHALAKIM_PER_HOUR + 3 * CHALAKIM_PER_HOUR + 589
    {
        amnt_days += 1;
    }

    amnt_days + 1
}

fn get_hebrew_date(p: chrono::DateTime<Utc>) -> (HebrewMonth, i64, i64) {
    let amnt_chalakim_per_cycle = *AMNT_CHALAKIM_PER_CYCLE;
    let diff_sec = (p - *EPOCH).num_seconds();
    let diff_chalakim = diff_sec * CHALAKIM_PER_HOUR / 60 / 60;
    let amnt_cycles = diff_chalakim / amnt_chalakim_per_cycle;

    let mut remainder_chalakim = diff_chalakim % amnt_chalakim_per_cycle;
    let mut year = FIRST_YEAR + amnt_cycles * 19;
    for i in LEAP_YEARS.iter() {
        let chalakim_this_year = if *i { 13 } else { 12 } * CHALAKIM_BETWEEN_MOLAD;

        if remainder_chalakim - chalakim_this_year < 0 {
            break;
        }
        year += 1;
        remainder_chalakim -= chalakim_this_year;
    }

    let mut month = 0;
    let amnt_days_in_year = get_rosh_hashana(year + 1) - get_rosh_hashana(year);
    let sched = &YEAR_SCHED[&amnt_days_in_year];
    for amnt_days in sched.iter() {
        let chalakim_this_month = *amnt_days as i64 * CHALAKIM_PER_HOUR * 24;
        if (remainder_chalakim - chalakim_this_month as i64) < 0 {
            break;
        }
        month += 1;
        remainder_chalakim -= chalakim_this_month as i64;
    }

    let day = if p.hour() <= 18 { 0 } else { 1 };
    (
        HebrewMonth::from_i32(month).unwrap(),
        day + remainder_chalakim / (CHALAKIM_PER_HOUR * 24) + 1,
        year,
    )
}

fn get_english_date(h: HebrewDate) -> Result<chrono::DateTime<Utc>, ConversionError> {
    if months_per_year((h.year - FIRST_YEAR) % 19) == 12
        && (h.month == HebrewMonth::Adar1 || h.month == HebrewMonth::Adar2)
    {
        return Err(ConversionError::IsNotLeapYear);
    }
    if months_per_year((h.year - FIRST_YEAR) % 19) == 13 && h.month == HebrewMonth::Adar {
        return Err(ConversionError::IsLeapYear);
    }

    let amnt_days_between_rh_and_epoch = get_rosh_hashana(h.year);
    let amnt_days_in_year = get_rosh_hashana(h.year + 1) - get_rosh_hashana(h.year);
    let sched = &YEAR_SCHED[&amnt_days_in_year];

    if h.day > sched[h.month as usize] {
        return Err(ConversionError::TooManyDaysInMonth(sched[h.month as usize]));
    }
    let mut amnt_days_in_month: i16 = 0;
    if h.month != HebrewMonth::Tishrei {
        for i in 0..h.month as usize {
            amnt_days_in_month += sched[i] as i16;
        }
    }
    let amnt_days =
        amnt_days_between_rh_and_epoch + amnt_days_in_month as i64 + h.day as i64 - 1 - 1;
    Ok(*EPOCH + Duration::days(amnt_days))
}
enum_from_primitive! {
  #[derive(Debug, PartialEq, Copy, Clone, Deserialize, Serialize)]
  pub enum HebrewMonth {
    Tishrei = 0,
    Cheshvan = 1,
    Kislev = 2,
    Teves = 3,
    Shvat = 4,
    Adar = 5,
    Adar1 = 6,
    Adar2 = 7,
    Nissan = 8,
    Iyar = 9,
    Sivan = 10,
    Tammuz = 11,
    Av = 12,
    Elul = 13
  }
}

impl HebrewMonth {
    pub fn month_list() -> Vec<&'static str> {
        vec![
            "Tishrei", "Cheshvan", "Kislev", "Teves", "Shvat", "Adar", "Adar1", "Adar2", "Nissan",
            "Iyar", "Sivan", "Tammuz", "Av", "Elul",
        ]
    }
    pub fn try_from(s: &str) -> Result<HebrewMonth, ConversionError> {
        match s {
            "Tishrei" => Ok(HebrewMonth::Tishrei),
            "Cheshvan" => Ok(HebrewMonth::Cheshvan),
            "Kislev" => Ok(HebrewMonth::Kislev),
            "Teves" => Ok(HebrewMonth::Teves),
            "Shvat" => Ok(HebrewMonth::Shvat),
            "Adar" => Ok(HebrewMonth::Adar),
            "Adar1" => Ok(HebrewMonth::Adar1),
            "Adar 1" => Ok(HebrewMonth::Adar1),
            "Adar Aleph" => Ok(HebrewMonth::Adar1),
            "Adar2" => Ok(HebrewMonth::Adar2),
            "Adar 2" => Ok(HebrewMonth::Adar2),
            "Adar Beis" => Ok(HebrewMonth::Adar2),
            "Nissan" => Ok(HebrewMonth::Nissan),
            "Iyar" => Ok(HebrewMonth::Iyar),
            "Sivan" => Ok(HebrewMonth::Sivan),
            "Tammuz" => Ok(HebrewMonth::Tammuz),
            "Av" => Ok(HebrewMonth::Av),
            "Elul" => Ok(HebrewMonth::Elul),
            _ => Err(ConversionError::MonthDoesntExist),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            HebrewMonth::Tishrei => "Tishrei",
            HebrewMonth::Cheshvan => "Cheshvan",
            HebrewMonth::Kislev => "Kislev",
            HebrewMonth::Teves => "Teves",
            HebrewMonth::Shvat => "Shvat",
            HebrewMonth::Adar => "Adar",
            HebrewMonth::Adar1 => "Adar 1",
            HebrewMonth::Adar2 => "Adar 2",
            HebrewMonth::Nissan => "Nissan",
            HebrewMonth::Iyar => "Iyar",
            HebrewMonth::Sivan => "Sivan",
            HebrewMonth::Tammuz => "Tammuz",
            HebrewMonth::Av => "Av",
            HebrewMonth::Elul => "Elul",
        }
    }
}

impl std::fmt::Display for HebrewMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.as_str();
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub struct HebrewDate {
    day: i8,
    month: HebrewMonth,
    year: i64,
}

impl HebrewDate {
    pub fn day(&self) -> i8 {
        return self.day;
    }
    pub fn month(&self) -> HebrewMonth {
        return self.month;
    }
    pub fn year(&self) -> i64 {
        return self.year;
    }

    pub fn from_ymd(
        year: i64,
        month: HebrewMonth,
        day: i64,
    ) -> Result<HebrewDate, ConversionError> {
        if year < FIRST_YEAR + 1 {
            return Err(ConversionError::YearTooSmall(format!(
                "We cannot work with Gregorian years before year {}",
                FIRST_YEAR + 1
            )));
        }
        Ok(HebrewDate {
            year: year,
            month: month,
            day: day as i8,
        })
    }

    pub fn from_eng(time: chrono::DateTime<Utc>) -> Result<HebrewDate, ConversionError> {
        if time.year() < (*EPOCH + Duration::days(365)).year() {
            return Err(ConversionError::YearTooSmall(format!(
                "We cannot work with Gregorian years before year {}",
                (*EPOCH + Duration::days(365)).year()
            )));
        }
        let res = get_hebrew_date(time);
        Ok(HebrewDate {
            month: res.0,
            day: res.1 as i8,
            year: res.2,
        })
    }
    pub fn to_eng(self) -> Result<chrono::DateTime<Utc>, ConversionError> {
        get_english_date(self)
    }
}

#[derive(Debug)]

pub enum ConversionError {
    IsNotLeapYear,
    TooManyDaysInMonth(i8),
    IsLeapYear,
    MonthDoesntExist,
    YearTooSmall(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::ConversionError::*;
        match self {
        IsNotLeapYear => write!(f, "The year you specified is not a leap year, yet you're trying to convert from an Adar1 or Adar2. Use the regular Adar for a regular year"),
        TooManyDaysInMonth(d) => write!(f,"There aren't {} days in this month",d),
        IsLeapYear => write!(f, "The year you specified is a leap year, yet you're trying to convert from a Regular Adar. Use Adar1 or Adar2 on a leap year"),
        MonthDoesntExist => write!(f, "This month doesn't exist. Please specify another one."),
        YearTooSmall(s) => write!(f, "{}",s)
        }
    }
}
