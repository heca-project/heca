#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate time;
use chrono::prelude::*;
use std::fmt;
use time::Duration;
#[macro_use]
extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

// ChalakimBetweenMolad is the amount of Chalakim between two Molads
// See https://www.chabad.org/library/article_cdo/aid/947923/jewish/Kiddush-HaChodesh-Chapter-Six.htm#footnoteRef8a947923
pub const CHALAKIM_PER_HOUR: i64 = 1080;
pub const CHALAKIM_BETWEEN_MOLAD: i64 = 29 * 24 * CHALAKIM_PER_HOUR + 12 * CHALAKIM_PER_HOUR + 793;
const LEAP_YEARS: [bool; 19] = [
    false, false, true, false, false, true, false, true, false, false, true, false, false, true,
    false, false, true, false, true,
];
// FirstMolad (of Tishrei 3673) was on Monday, September 23rd at 12:16:6 Chalakim
const FIRST_MOLAD: i64 = 1 * 24 * 1080 + 18 * 1080 + (16 * 1080 / 60) + 6;
const FIRST_YEAR: i64 = 3763;
lazy_static! {
    static ref EPOCH: chrono::DateTime<Utc> = Utc.ymd(2, 9, 21).and_hms(18, 0, 0);
}

fn return_year_sched(days: i64) -> usize {
    match days {
        353 => 0,
        354 => 1,
        355 => 2,

        383 => 3,
        384 => 4,
        385 => 5,
        _ => panic!(format!("Wrong amount of days {}", days)),
    }
}
const YEAR_SCHED: [[i8; 14]; 6] = [
    [30, 29, 29, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 29, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 30, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 29, 29, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
    [30, 29, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
    [30, 30, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
];

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
        amnt_chalakim += if LEAP_YEARS[i as usize] { 13 } else { 12 } * CHALAKIM_BETWEEN_MOLAD;
    }

    return amnt_chalakim;
}

fn months_per_year(year: i64) -> i64 {
    let year_in_cycle = ((year - FIRST_YEAR) % 19) as usize;
    if LEAP_YEARS[year_in_cycle] {
        13
    } else {
        12
    }
}

fn get_rosh_hashana(year: i64) -> (i64, Day) {
    let amnt_chalakim_since_first_molad = get_molad_for_year(year);
    let amnt_chalakim_since_epoch = amnt_chalakim_since_first_molad + FIRST_MOLAD;

    let mut amnt_days = amnt_chalakim_since_epoch / (CHALAKIM_PER_HOUR * 24);
    let amnt_chalakim = amnt_chalakim_since_epoch % (CHALAKIM_PER_HOUR * 24);
    let mut reg_postpone = false;
    //If the Molad is in the afternoon, postpone Rosh Hashana by a day
    if amnt_chalakim > 18 * CHALAKIM_PER_HOUR {
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
        && months_per_year(year) == 12
    {
        amnt_days += 2;
    }

    if !reg_postpone
        && months_per_year(year - 1) == 13
        && dow == 1
        && amnt_chalakim > 12 * CHALAKIM_PER_HOUR + 3 * CHALAKIM_PER_HOUR + 589
    {
        amnt_days += 1;
    }

    (amnt_days + 1, Day::from_i64(dow).unwrap())
}

fn get_english_date(h: HebrewDate) -> Result<chrono::DateTime<Utc>, ConversionError> {
    let months_per_year = months_per_year(h.year);
    if months_per_year == 12 && (h.month == HebrewMonth::Adar1 || h.month == HebrewMonth::Adar2) {
        return Err(ConversionError::IsNotLeapYear);
    }
    if months_per_year == 13 && h.month == HebrewMonth::Adar {
        return Err(ConversionError::IsLeapYear);
    }

    let amnt_days_between_rh_and_epoch = get_rosh_hashana(h.year).0;
    let amnt_days_in_year = get_rosh_hashana(h.year + 1).0 - amnt_days_between_rh_and_epoch;
    let sched = &YEAR_SCHED[return_year_sched(amnt_days_in_year)];

    if h.day > sched[h.month as usize] {
        return Err(ConversionError::TooManyDaysInMonth(sched[h.month as usize]));
    }
    let mut amnt_days_in_month: i16 = 0;
    if h.month != HebrewMonth::Tishrei {
        for i in 0..h.month as usize {
            amnt_days_in_month += sched[i] as i16;
        }
    }
    let amnt_days = amnt_days_between_rh_and_epoch + amnt_days_in_month as i64 + h.day as i64 - 1 -1 ;
    Ok(*EPOCH + Duration::days(amnt_days))
}

#[derive(Debug, Copy, Clone)]
pub struct HebrewDate {
    day: i8,
    month: HebrewMonth,
    year: i64,
    molads_of_month: [i64; 14],
    months_length: &'static [i8; 14],
    rosh_hashana_dow: Day,
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
                "We cannot work with Hebrew years before year {}",
                FIRST_YEAR + 1
            )));
        }

        let amnt_days = get_rosh_hashana(year + 1).0 - get_rosh_hashana(year).0;
        let sched = &YEAR_SCHED[return_year_sched(amnt_days)];

        let mut molads_of_month = [0; 14];
        for i in 0..14 {
            molads_of_month[i] = 0;
        }
        Ok(HebrewDate {
            year: year,
            month: month,
            day: day as i8,
            rosh_hashana_dow: get_rosh_hashana(year).1,
            months_length: sched,
            molads_of_month: molads_of_month,
        })
    }

    pub fn from_eng(time: chrono::DateTime<Utc>) -> Result<HebrewDate, ConversionError> {
        if time.year() < (*EPOCH + Duration::days(365)).year() {
            return Err(ConversionError::YearTooSmall(format!(
                "We cannot work with Gregorian years before year {}",
                (*EPOCH + Duration::days(365)).year()
            )));
        }

        let amnt_chalakim_per_cycle = *AMNT_CHALAKIM_PER_CYCLE;
        let diff_sec = (time - *EPOCH).num_seconds();
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
        let current_rh = get_rosh_hashana(year);
        let amnt_days_in_year = get_rosh_hashana(year + 1).0 - current_rh.0;
        let sched = &YEAR_SCHED[return_year_sched(amnt_days_in_year)];
        for amnt_days in sched.iter() {
            let chalakim_this_month = *amnt_days as i64 * CHALAKIM_PER_HOUR * 24;
            if (remainder_chalakim - chalakim_this_month as i64) < 0 {
                break;
            }
            month += 1;
            remainder_chalakim -= chalakim_this_month as i64;
        }

        let day = if time.hour() <= 18 { 0 } else { 1 };

        let molads_of_month = [0; 14];
        Ok(HebrewDate {
            month: HebrewMonth::from_i32(month).unwrap(),
            day: (day + remainder_chalakim / (CHALAKIM_PER_HOUR * 24)) as i8,
            year: year,
            molads_of_month: molads_of_month,
            months_length: sched,
            rosh_hashana_dow: current_rh.1,
        })
    }
    pub fn to_eng(self) -> Result<chrono::DateTime<Utc>, ConversionError> {
        get_english_date(self)
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Copy, Clone)]
enum Day{
    Sunday,
    Monday,
    Tuesday,
    Wedneday,
    Thurday,
    Friday,
    Shabbos
}
}
enum_from_primitive! {
  #[derive(Debug, PartialEq, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn years_correct_sum() {
        assert_eq!(
            YEAR_SCHED[0].into_iter().map(|x| (*x) as i64).sum::<i64>(),
            353
        );
        assert_eq!(
            YEAR_SCHED[1].into_iter().map(|x| (*x) as i64).sum::<i64>(),
            354
        );
        assert_eq!(
            YEAR_SCHED[2].into_iter().map(|x| (*x) as i64).sum::<i64>(),
            355
        );
        assert_eq!(
            YEAR_SCHED[3].into_iter().map(|x| (*x) as i64).sum::<i64>(),
            383
        );
        assert_eq!(
            YEAR_SCHED[4].into_iter().map(|x| (*x) as i64).sum::<i64>(),
            384
        );
        assert_eq!(
            YEAR_SCHED[5].into_iter().map(|x| (*x) as i64).sum::<i64>(),
            385
        );
    }

    #[test]
    fn years_have_rght_days() {
        extern crate rayon;
        use rayon::prelude::*;

        ((FIRST_YEAR + 1)..1000000)
            .into_par_iter()
            .map(|i| {
                let amnt_days_between_rh_and_epoch = get_rosh_hashana(i).0;
                let amnt_days_in_year = get_rosh_hashana(i + 1).0 - amnt_days_between_rh_and_epoch;
                return_year_sched(amnt_days_in_year);
            })
            .count();
    }

    #[test]
    fn compare_my_rosh_hashana_to_known() {
        extern crate rayon;
        extern crate atoi;
        use atoi::atoi;
        use chrono::Utc;
        use rayon::prelude::*;
        use std::fs;
        use std::path::PathBuf;

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testing/RoshHashanaList");
        let err = format!(
            "Can't find list of Rosh Hashanas in file {}",
            d.to_str().unwrap()
        );
        let contents = fs::read_to_string(d).expect(&err);
        contents
            .split('\n')
            .collect::<Vec<&str>>()
            .into_par_iter()
            .filter(|x| *x != "")
            .map(|x| {
                let split_v = x.split(' ').collect::<Vec<&str>>();
                let n = atoi::<i64>(split_v[0].as_bytes()).unwrap();
                let hd = HebrewDate::from_ymd(n, HebrewMonth::Tishrei, 1).unwrap();
                let ed = hd.to_eng().unwrap();
                let dc_rhd = NaiveDate::parse_from_str(split_v[1], "%m/%d/%Y")
                    .unwrap()
                    .and_hms(0, 0, 0);

                assert_eq!(
                    Utc.ymd(dc_rhd.year(), dc_rhd.month(), dc_rhd.day())
                        .and_hms(0, 0, 0) - Duration::hours(24) - Duration::hours(6),
                    (ed)
                );
            })
            .count();
    }

    #[test]
    fn compare_rosh_chodesh_adar_known() {
        extern crate rayon;
        extern crate atoi;
        use atoi::atoi;
        use chrono::Utc;
        use rayon::prelude::*;
        use std::fs;
        use std::path::PathBuf;

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testing/RoshChodeshRegularAdar");
        let err = format!(
            "Can't find list of Rosh Chodesh Adar in file {}",
            d.to_str().unwrap()
        );
        let contents = fs::read_to_string(d).expect(&err);
        contents
            .split('\n')
            .collect::<Vec<&str>>()
            .into_par_iter()
            .filter(|x| *x != "")
            .map(|x| {
                let split_v = x.split(' ').collect::<Vec<&str>>();
                let n = atoi::<i64>(split_v[0].as_bytes()).unwrap();
                let hd = HebrewDate::from_ymd(n, HebrewMonth::Adar, 1).unwrap();
                let ed = hd.to_eng().unwrap();
                let dc_rhd = NaiveDate::parse_from_str(split_v[1], "%m/%d/%Y")
                    .unwrap()
                    .and_hms(0, 0, 0);
                assert_eq!(
                    Utc.ymd(dc_rhd.year(), dc_rhd.month(), dc_rhd.day())
                        .and_hms(0, 0, 0) - Duration::hours(6),
                    (ed)
                );
 
                            })
            .count();
    }

    #[test]
    fn compare_rosh_chodesh_adar1_known() {
        extern crate rayon;
        extern crate atoi;
        use atoi::atoi;
        use chrono::Utc;
        use rayon::prelude::*;
        use std::fs;
        use std::path::PathBuf;

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testing/RoshChodeshAdarI");
        let err = format!(
            "Can't find list of Rosh Chodesh AdarI in file {}",
            d.to_str().unwrap()
        );
        let contents = fs::read_to_string(d).expect(&err);
        contents
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| *x != &"")
            .map(|x| {
                let split_v = x.split(' ').collect::<Vec<&str>>();
                let n = atoi::<i64>(split_v[0].as_bytes()).unwrap();
                let hd = HebrewDate::from_ymd(n, HebrewMonth::Adar1, 1).unwrap();
                let ed = hd.to_eng().unwrap();
                let dc_rhd = NaiveDate::parse_from_str(split_v[1], "%m/%d/%Y")
                    .unwrap()
                    .and_hms(0, 0, 0);
               assert_eq!(
                    Utc.ymd(dc_rhd.year(), dc_rhd.month(), dc_rhd.day())
                        .and_hms(0, 0, 0) - Duration::hours(6),
                    (ed)
                );

               
            })
            .count();
    }

    #[test]
    fn compare_random_rosh_chodesh_adar2() {
                let hd = HebrewDate::from_ymd(5779, HebrewMonth::Adar1, 1).unwrap();
                let ed = hd.to_eng().unwrap();
                let real_val = Utc.ymd(2019,2,5).and_hms(18,0,0);
                assert_eq!(ed,real_val);
 
    }

    #[test]
    fn compare_rosh_chodesh_adar2_known() {
        extern crate rayon;
        extern crate atoi;
        use atoi::atoi;
        use chrono::Utc;
        use rayon::prelude::*;
        use std::fs;
        use std::path::PathBuf;

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testing/RoshChodeshAdarII");
        let err = format!(
            "Can't find list of Rosh Chodesh Adar2 in file {}",
            d.to_str().unwrap()
        );
        let contents = fs::read_to_string(d).expect(&err);
        contents
            .split('\n')
            .collect::<Vec<&str>>()
            .into_par_iter()
            .filter(|x| *x != "")
            .map(|x| {
                let split_v = x.split(' ').collect::<Vec<&str>>();
                let n = atoi::<i64>(split_v[0].as_bytes()).unwrap();
                let hd = HebrewDate::from_ymd(n, HebrewMonth::Adar2, 1).unwrap();
                let ed = hd.to_eng().unwrap();
                let dc_rhd = NaiveDate::parse_from_str(split_v[1], "%m/%d/%Y")
                    .unwrap()
                    .and_hms(0, 0, 0);
                assert_eq!(
                    Utc.ymd(dc_rhd.year(), dc_rhd.month(), dc_rhd.day())
                        .and_hms(0, 0, 0)  - Duration::hours(6),
                    (ed)
                );


            })
            .count();
    }

}
