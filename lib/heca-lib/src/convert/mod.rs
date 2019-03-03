extern crate chrono;
extern crate num;
extern crate time;

use chrono::prelude::*;
use num::FromPrimitive;
use std::fmt;
use time::Duration;

use crate::types::ConversionError;
use crate::types::Day;
use crate::types::HebrewMonth;

pub mod year;

/// The amount of Chalakim in an hour.
pub const CHALAKIM_PER_HOUR: u64 = 1080;
/// The amount of Chalakim between two Molads.
// See https://www.chabad.org/library/article_cdo/aid/947923/jewish/Kiddush-HaChodesh-Chapter-Six.htm#footnoteRef8a947923
pub const CHALAKIM_BETWEEN_MOLAD: u64 = 29 * 24 * CHALAKIM_PER_HOUR + 12 * CHALAKIM_PER_HOUR + 793;

//An array documenting which years are leap years. The Hebrew calendar has a 19 year cycle of leap
//years.
const LEAP_YEARS: [bool; 19] = [
    false, false, true, false, false, true, false, true, false, false, true, false, false, true,
    false, false, true, false, true,
];

// There are three starting dates. Right now, we don't work with negative Gregorian dates, so the
// Epoch period is the first year of the first 19 year cycle after year 0.
//
// 1. Epoch - this is the first day, is on 6:00 PM Shabbos (Saturay) afternoon.
// 2. FIRST_MOLAD - the amount of Chalakim from Epoch to the first Molad -(Tishrei 3673). It was on Monday, September 23rd at 12:16:6 Chalakim
// 3. FIRST_YEAR: Self described - this is the first Hebrew calendar since the epoch.
// 4. FIRST_RH: The first Rosh Hashana since the Epoch.
const FIRST_MOLAD: u64 = 24 * 1080 + 18 * 1080 + (16 * 1080 / 60) + 6;
const FIRST_YEAR: u64 = 3763;
lazy_static! {
    static ref FIRST_RH: chrono::DateTime<Utc> = Utc.ymd(2, 9, 23).and_hms(18, 0, 0);
    static ref EPOCH: chrono::DateTime<Utc> = Utc.ymd(2, 9, 21).and_hms(18, 0, 0);
}
// Return the correct schedule for they year. There can be only six possible amount of days, so
// short of a bug on my part, this should never panic.
fn return_year_sched(days: u64) -> usize {
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
const YEAR_SCHED: [[u8; 14]; 6] = [
    [30, 29, 29, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 29, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 30, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 29, 29, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
    [30, 29, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
    [30, 30, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
];

//This calculates the amount of Chalakim per 19 year cycle.
const AMNT_CHALAKIM_PER_CYCLE: u64 =
    7 * 13 * CHALAKIM_BETWEEN_MOLAD + 12 * 12 * CHALAKIM_BETWEEN_MOLAD;

fn get_molad_for_year(year: u64) -> u64 {
    let amnt_of_cycles = (year - FIRST_YEAR) / 19;

    let mut amnt_chalakim = AMNT_CHALAKIM_PER_CYCLE * amnt_of_cycles;
    let cur_year_in_cycle = (year - FIRST_YEAR) % 19;
    for i in 0..cur_year_in_cycle {
        amnt_chalakim += if LEAP_YEARS[i as usize] { 13 } else { 12 } * CHALAKIM_BETWEEN_MOLAD;
    }

    amnt_chalakim
}

//Does short calculation if this year is a leap year.
pub(crate) fn months_per_year(year: u64) -> u64 {
    let year_in_cycle = ((year - FIRST_YEAR) % 19) as usize;
    if LEAP_YEARS[year_in_cycle] {
        13
    } else {
        12
    }
}

//Calculate how many Chalakim between Epoch and Rosh Hashana, and which day of the week does it
//fall out on.
pub(crate) fn get_rosh_hashana(year: u64) -> (u64, Day) {
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

    //This shouldn't panic, as there are seven options in Day (seven days in week).
    let mut dow = Day::from_u64((amnt_days) % 7).unwrap();
    // Lo Adu Rosh

    if dow == Day::Sunday || dow == Day::Wednesday || dow == Day::Friday {
        amnt_days += 1;
        reg_postpone = true;
    }

    // See Hilchos Kiddush HaChodesh Halacha 4

    if !reg_postpone
        && dow == Day::Tuesday
        && amnt_chalakim > 9 * CHALAKIM_PER_HOUR + 204
        && months_per_year(year) == 12
    {
        amnt_days += 2;
    }

    if !reg_postpone
        && months_per_year(year - 1) == 13
        && dow == Day::Monday
        && amnt_chalakim > 12 * CHALAKIM_PER_HOUR + 3 * CHALAKIM_PER_HOUR + 589
    {
        amnt_days += 1;
    }

    //This shouldn't panic, as there are seven options in Day (seven days in week).
    dow = Day::from_u64((amnt_days) % 7).unwrap();

    (amnt_days, dow)
}

/// HebrewDate is a struct containing a Hebrew date. There are two ways to generate it: Either `HebrewDate::from_ymd()` or `HebrewDate::from_gregorian()`.

#[derive(Debug, Copy, Clone)]
pub struct HebrewDate {
    day: u8,
    month: HebrewMonth,
    year: u64,
}

impl Eq for HebrewDate {}
impl PartialEq for HebrewDate {
    fn eq(&self, other: &HebrewDate) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}

use std::cmp::Ordering;
impl Ord for HebrewDate {
    fn cmp(&self, other: &HebrewDate) -> Ordering {
        if self.year < other.year {
            Ordering::Less
        } else if self.year > other.year {
            Ordering::Greater
        } else if (self.month as i32) < (other.month as i32) {
            Ordering::Less
        } else if (self.month as i32) > (other.month as i32) {
            Ordering::Greater
        } else if self.day < other.day {
            Ordering::Less
        } else if self.day > other.day {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for HebrewDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HebrewDate {
    /// Returns a HebrewDate on success, or a ConversionError on failure.
    ///
    /// # Arguments
    /// * `year` - The Hebrew year since creation.
    /// * `month` - The Hebrew month.
    /// * `day` - The Hebrew day of month.
    ///
    /// # Error Values
    /// * YearTooSmall - This algorithm won't work if the year is before 3764.
    /// * DayIsZero - Months start with day 1, not zero. So 0 Adar won't work.
    /// * IsLeapYear - I treat Adar, Adar 1 and Adar 2 as three seperate months, so if you want to
    /// convert a day in Adar 1 or Adar 2 of a leap year, specify which one.
    ///  * IsNotLeapYear - I treat Adar, Adar 1 and Adar 2 as three seperate months, so it won't
    ///  make sense to get the English date of the first of Adar 1 or Adar 2 if the year isn't a
    ///  leap year.
    ///  * TooManyDaysInMonth - There are either 29 or 30 days in a month, so it doesn't make sense
    ///  to find the 50th day of Nissan.
    pub fn from_ymd(year: u64, month: HebrewMonth, day: u8) -> Result<HebrewDate, ConversionError> {
        //Get a HebrewDate object from the Hebrew Year, Month, and Day. Can fail if the year is too
        //small or the day is less than one.

        if year < FIRST_YEAR + 1 {
            return Err(ConversionError::YearTooSmall);
        }
        if day == 0 {
            return Err(ConversionError::DayIsZero);
        }
        let months_per_year = months_per_year(year);
        if months_per_year == 12 && (month == HebrewMonth::Adar1 || month == HebrewMonth::Adar2) {
            return Err(ConversionError::IsNotLeapYear);
        }
        if months_per_year == 13 && month == HebrewMonth::Adar {
            return Err(ConversionError::IsLeapYear);
        }

        let amnt_days_between_rh_and_epoch = get_rosh_hashana(year).0;
        let amnt_days_in_year = get_rosh_hashana(year + 1).0 - amnt_days_between_rh_and_epoch;
        let sched = &YEAR_SCHED[return_year_sched(amnt_days_in_year)];

        if day > sched[month as usize] {
            return Err(ConversionError::TooManyDaysInMonth(sched[month as usize]));
        }

        Ok(HebrewDate {
            year,
            month,
            day: day as u8,
        })
    }

    // Use this carefully, as it can result in code panicing. It assumes that the dates are known
    // to be valid.
    pub(crate) fn from_ymd_unsafe(year: u64, month: HebrewMonth, day: u8) -> HebrewDate {
        HebrewDate {
            year,
            month,
            day: day as u8,
        }
    }

    fn day_of_last_rh(days_since_first_rh: u64) -> u64 {
        let mut cur_year = (FIRST_YEAR) + 19 * days_since_first_rh / 6956;
        if get_rosh_hashana(cur_year).0 > days_since_first_rh {
            panic!("get_rosh_hashana(cur_year).0 < days_since_first_rh ");
        }
        while get_rosh_hashana(cur_year + 1).0 <= days_since_first_rh {
            cur_year += 1;
        }
        return cur_year;
    }

    /// Returns a HebrewDate on success, or a ConversionError on failure.
    ///
    /// # Arguments
    /// * `date` - The Gregorian date.
    ///
    /// # Notes:
    /// Hebrew days start at sundown, not midnight, so there isn't a full 1:1 mapping between
    /// Gregorian days and Hebrew. So when you look up the date of Rosh Hashana 5779, you'll get "Monday, 10th of September 2018", while Rosh Hashana really started at sundown on the 9th of September.
    ///
    /// I'm trying to be a _bit_ more precise, so I made the date cutoff at 6:00 PM. So for
    /// example:
    /// ```
    /// extern crate heca_lib;
    ///
    /// use chrono::Utc;
    /// use chrono::offset::TimeZone;
    /// use heca_lib::{HebrewDate,HebrewMonth};
    ///
    /// assert_eq!(HebrewDate::from_gregorian(Utc.ymd(2018,9,10).and_hms(17,59,59)).unwrap(),HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,1).unwrap());
    /// ```
    ///
    /// while
    ///
    /// ```
    /// extern crate heca_lib;
    ///
    /// use chrono::Utc;
    /// use chrono::offset::TimeZone;
    /// use heca_lib::{HebrewDate,HebrewMonth};
    ///
    /// assert_eq!(HebrewDate::from_gregorian(Utc.ymd(2018,9,10).and_hms(18,0,0)).unwrap(),HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,2).unwrap());
    /// ```
    /// # Error Values:
    /// * YearTooSmall - This algorithm won't work if the year is before roughly year 4.
    ///
    pub fn from_gregorian(date: DateTime<Utc>) -> Result<HebrewDate, ConversionError> {
        let days_since_first_rh = ((date - *FIRST_RH).num_days() + 2) as u64;

        if days_since_first_rh < 365 {
            return Err(ConversionError::YearTooSmall);
        }
        let year = Self::day_of_last_rh(days_since_first_rh);
        let cur_rh = get_rosh_hashana(year).0;
        Ok(Self::get_hebrewdate_from_days_after_rh(
            year,
            days_since_first_rh,
            cur_rh,
        ))
    }
    pub(crate) fn get_hebrewdate_from_days_after_rh(
        year: u64,
        days_since_first_rh: u64,
        cur_rh: u64,
    ) -> HebrewDate {
        let mut remainder = (days_since_first_rh - cur_rh) as u64;
        let amnt_days_in_year = get_rosh_hashana(year + 1).0 - cur_rh;
        let sched = YEAR_SCHED[return_year_sched(amnt_days_in_year)];
        let mut month: u64 = 0;
        for days_in_month in &sched {
            if remainder < u64::from(*days_in_month) {
                break;
            }
            month += 1;
            remainder -= u64::from(*days_in_month);
        }
        HebrewDate {
            year,
            month: HebrewMonth::from_u64(month).unwrap(),
            day: remainder as u8 + 1,
        }
    }

    /// Gets the Grgorian date for the current Hebrew date.
    ///
    /// # Notes
    ///
    /// This function returns the DateTime of the given HebrewDate at nightfall.
    ///
    /// For example, Yom Kippur 5779 started at sunset of September 18, 2018. So
    /// ```
    /// extern crate heca_lib;
    ///
    /// use chrono::Utc;
    /// use chrono::offset::TimeZone;
    /// use heca_lib::{HebrewDate,HebrewMonth};
    ///
    /// assert_eq!(HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,10).unwrap().to_gregorian(),Utc.ymd(2018, 9,18).and_hms(18,00,00));
    /// ```
    pub fn to_gregorian(&self) -> chrono::DateTime<Utc> {
        let amnt_days_between_rh_and_epoch = get_rosh_hashana(self.year).0;
        let amnt_days_in_year = get_rosh_hashana(self.year + 1).0 - amnt_days_between_rh_and_epoch;
        let sched = &YEAR_SCHED[return_year_sched(amnt_days_in_year)];

        let mut amnt_days_in_month: u16 = 0;
        if self.month != HebrewMonth::Tishrei {
            for item in sched.iter().take(self.month as usize) {
                amnt_days_in_month += u16::from(*item);
            }
        }

        let amnt_days =
            amnt_days_between_rh_and_epoch + u64::from(amnt_days_in_month) + u64::from(self.day)
                - 1;
        *EPOCH + Duration::days(amnt_days as i64)
    }
    ///Get the Hebrew day of month.
    #[inline]
    pub fn day(&self) -> u8 {
        self.day
    }

    ///Get the Hebrew month of year
    #[inline]
    pub fn month(&self) -> HebrewMonth {
        self.month
    }

    ///Get the Hebrew year.

    #[inline]
    pub fn year(&self) -> u64 {
        self.year
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn years_correct_sum() {
        assert_eq!(
            YEAR_SCHED[0].into_iter().map(|x| (*x) as u64).sum::<u64>(),
            353
        );
        assert_eq!(
            YEAR_SCHED[1].into_iter().map(|x| (*x) as u64).sum::<u64>(),
            354
        );
        assert_eq!(
            YEAR_SCHED[2].into_iter().map(|x| (*x) as u64).sum::<u64>(),
            355
        );
        assert_eq!(
            YEAR_SCHED[3].into_iter().map(|x| (*x) as u64).sum::<u64>(),
            383
        );
        assert_eq!(
            YEAR_SCHED[4].into_iter().map(|x| (*x) as u64).sum::<u64>(),
            384
        );
        assert_eq!(
            YEAR_SCHED[5].into_iter().map(|x| (*x) as u64).sum::<u64>(),
            385
        );
    }

    #[test]
    fn get_year() {
        for j in 0..100 {
            let mut original_day = Utc.ymd(16 + j, 10, 4).and_hms(18, 0, 0);
            for i in 1..366 {
                let h_day = HebrewDate::from_gregorian(original_day).unwrap();
                let ne_day = h_day.to_gregorian();
                assert_eq!(original_day, ne_day);
                original_day = original_day + Duration::days(1);
            }
        }
    }

    #[test]
    fn years_have_right_days() {
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
    fn compare_hebrew_day_elul_sanity_check() {
        let mut orig_date = Utc.ymd(1901, 8, 15).and_hms(18, 0, 0);
        for j in 1..=29 {
            let heb_day = HebrewDate::from_ymd(5661, HebrewMonth::Elul, j).unwrap();
            let back = heb_day.to_gregorian();
            println!("{}", j);
            assert_eq!(orig_date, back);
            orig_date = orig_date + Duration::days(1);
        }
    }

    #[test]
    fn compare_hebrew_day_tishrei_sanity_check() {
        let mut orig_date = Utc.ymd(1900, 9, 23).and_hms(18, 0, 0);
        for j in 1..=30 {
            let heb_day = HebrewDate::from_ymd(5661, HebrewMonth::Tishrei, j).unwrap();
            let back = heb_day.to_gregorian();
            println!("{}", j);
            assert_eq!(orig_date, back);
            orig_date = orig_date + Duration::days(1);
        }
    }
    #[test]
    fn compare_hebrew_day_adar1_sanity_check() {
        let mut orig_date = Utc.ymd(1900, 1, 30).and_hms(18, 0, 0);
        for j in 1..=30 {
            let heb_day = HebrewDate::from_ymd(5660, HebrewMonth::Adar1, j).unwrap();
            let back = heb_day.to_gregorian();
            println!("{}", j);
            assert_eq!(orig_date, back);
            orig_date = orig_date + Duration::days(1);
        }
    }

    #[test]
    fn test_rh_against_working_list() {
        test_against_working_list("RoshHashanaList", 1, HebrewMonth::Tishrei);
    }
    #[test]
    fn test_adar1_against_working_list() {
        test_against_working_list("Adar1List", 1, HebrewMonth::Adar1);
    }

    fn test_against_working_list(filename: &str, day: u8, month: HebrewMonth) {
        let file_contents = std::fs::read_to_string(format!("./testing/{}", filename)).unwrap();
        file_contents
            .split("\n")
            .filter(|x| *x != "")
            .for_each(|x| {
                let res = x.split(" ").collect::<Vec<&str>>();
                if res.len() != 1 {
                    let eng_day =
                        HebrewDate::from_ymd(res[0].parse::<u64>().unwrap(), month, day as u8)
                            .unwrap()
                            .to_gregorian()
                            + Duration::days(1);
                    println!("{:?}", eng_day);
                    let sp = res[1].split("/").collect::<Vec<&str>>();
                    let (month, day, year) = (sp[0], sp[1], sp[2]);
                    assert_eq!(month.parse::<u64>().unwrap() as u32, eng_day.month());
                    assert_eq!(day.parse::<u64>().unwrap() as u32, eng_day.day());
                    assert_eq!(year.parse::<u64>().unwrap() as i32, eng_day.year());
                }
            });
    }

    extern crate test;
    use test::Bencher;
    #[bench]
    fn time_from_ymd(b: &mut Bencher) {
        b.iter(|| test::black_box(HebrewDate::from_ymd(9999, HebrewMonth::Tishrei, 1)));
    }
    #[bench]
    fn time_from_ymd_unsafe(b: &mut Bencher) {
        b.iter(|| test::black_box(HebrewDate::from_ymd_unsafe(9999, HebrewMonth::Tishrei, 1)));
    }

}
