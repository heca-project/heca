extern crate test;
use chrono::prelude::*;
use enum_primitive::FromPrimitive;

use crate::types::ConversionError;
use crate::types::Day;
use crate::types::HebrewMonth;

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

#[derive(Debug, Clone, Copy)]
pub struct HebrewYear {
    last_year: InnerHebrewYear,
    this_year: InnerHebrewYear,
    next_year: InnerHebrewYear,
}

#[derive(Debug, Clone, Copy)]
struct InnerHebrewYear {
    rosh_hashana_since_epoch: u64,
    amnt_days_in_year: u64,
    amnt_cycles: u64,
    cur_year_in_cycle: u8,
    amnt_chalakim_until_rh: u64,
    amnt_chalakim_until_next_rh: u64,
    day_of_week: Day,
}

impl HebrewYear {
    fn new(year: u64) -> Result<HebrewYear, ConversionError> {
        if year < FIRST_YEAR + 2 {
            return Err(ConversionError::YearTooSmall);
        }
        let last_year = InnerHebrewYear::new(year - 1);
        let this_year = last_year.clone().add_year();
        let next_year = this_year.clone().add_year();
        Ok(HebrewYear {
            last_year,
            this_year,
            next_year,
        })
    }
}

impl InnerHebrewYear {
    fn new(year: u64) -> InnerHebrewYear {
        let (amnt_cycles, cur_year_in_cycle) = get_year_in_cycle(year);
        let amnt_chalakim_until_rh = get_chalakim_until_rh(amnt_cycles, cur_year_in_cycle);
        let is_leap_year = months_per_year(year) == 12;
        let is_last_year_leap = months_per_year(year - 1) == 13;
        let (rosh_hashana_since_epoch, day_of_week) =
            get_rosh_hashana(amnt_chalakim_until_rh, is_leap_year, is_last_year_leap);

        let (amnt_days_in_year, amnt_chalakim_until_next_rh) = {
            let next_year = year + 1;
            let (amnt_cycles, cur_year_in_cycle) = get_year_in_cycle(next_year);
            let amnt_chalakim_until_rh = get_chalakim_until_rh(amnt_cycles, cur_year_in_cycle);

            let is_last_year_leap = is_leap_year;
            let is_leap_year = months_per_year(next_year) == 12;
            let last_year_rh_since_epoch = rosh_hashana_since_epoch;
            let (rosh_hashana_since_epoch, _) =
                get_rosh_hashana(amnt_chalakim_until_rh, is_leap_year, is_last_year_leap);
            (
                rosh_hashana_since_epoch - last_year_rh_since_epoch,
                amnt_chalakim_until_rh,
            )
        };

        InnerHebrewYear {
            amnt_chalakim_until_rh,
            amnt_chalakim_until_next_rh,
            amnt_cycles,
            cur_year_in_cycle,
            day_of_week,
            rosh_hashana_since_epoch,
            amnt_days_in_year,
        }
    }
    fn add_year(&mut self) -> InnerHebrewYear {
        if self.cur_year_in_cycle != 18 {
            self.cur_year_in_cycle += 1;
        } else {
            self.cur_year_in_cycle = 0;
            self.amnt_cycles += 1;
        }

        *self
    }
}

fn get_year_in_cycle(year: u64) -> (u64, u8) {
    let amnt_of_cycles = (year - FIRST_YEAR) / 19;

    let cur_year_in_cycle = (year - FIRST_YEAR) % 19;
    (amnt_of_cycles, cur_year_in_cycle as u8)
}

fn get_chalakim_until_rh(amnt_cycles: u64, cur_year_in_cycle: u8) -> u64 {
    let mut amnt_chalakim = AMNT_CHALAKIM_PER_CYCLE * amnt_cycles;
    for i in 0..cur_year_in_cycle {
        amnt_chalakim += if LEAP_YEARS[i as usize] { 13 } else { 12 } * CHALAKIM_BETWEEN_MOLAD;
    }
    amnt_chalakim
}

//Does short calculation if this year is a leap year.
#[inline]
fn months_per_year(year: u64) -> u64 {
    let year_in_cycle = ((year - FIRST_YEAR) % 19) as usize;
    if LEAP_YEARS[year_in_cycle] {
        13
    } else {
        12
    }
}

//Calculate how many Chalakim between Epoch and Rosh Hashana, and which day of the week does it
//fall out on.

fn get_rosh_hashana(
    amnt_chalakim_since_first_molad: u64,
    is_leap_year: bool,
    last_year_leap: bool,
) -> (u64, Day) {
    dbg!(amnt_chalakim_since_first_molad);
    dbg!(is_leap_year);
    dbg!(last_year_leap);

    let amnt_chalakim_since_epoch = amnt_chalakim_since_first_molad + FIRST_MOLAD;

    let mut amnt_days = amnt_chalakim_since_epoch / (CHALAKIM_PER_HOUR * 24);
    let amnt_chalakim = amnt_chalakim_since_epoch % (CHALAKIM_PER_HOUR * 24);
    use enum_primitive::FromPrimitive;
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

    //This shouldn't panic, as there are seven options in Day (seven days in week).
    dow = Day::from_u64((amnt_days) % 7).unwrap();

    // See Hilchos Kiddush HaChodesh Halacha 4

    if !reg_postpone
        && dow == Day::Tuesday
        && amnt_chalakim > 9 * CHALAKIM_PER_HOUR + 204
        && !is_leap_year
    {
        amnt_days += 2;
    }

    if !reg_postpone
        && last_year_leap
        && dow == Day::Monday
        && amnt_chalakim > 12 * CHALAKIM_PER_HOUR + 3 * CHALAKIM_PER_HOUR + 589
    {
        amnt_days += 1;
    }

    (amnt_days, dow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
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
    fn get_rh() {
        assert_eq!(
            HebrewYear::new(5779).unwrap().last_year.day_of_week,
            Day::Thursday
        );
        assert_eq!(
            HebrewYear::new(5780).unwrap().last_year.day_of_week,
            Day::Monday
        );
        assert_eq!(
            HebrewYear::new(5781).unwrap().last_year.day_of_week,
            Day::Monday
        );
    }
    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| HebrewYear::new(15781));
    }
}
