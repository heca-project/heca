use chrono::prelude::*;
use time::Duration;

use crate::prelude::*;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::num::NonZeroI8;

mod year;
#[doc(inline)]
pub use year::*;

#[derive(Debug, Copy, Clone)]
/// HebrewDate holds a specific Hebrew Date. It can be constructed individually or through HebrewYear. 
pub struct HebrewDate {
    
    day: NonZeroI8,
    month: HebrewMonth,
    year: HebrewYear,
}
impl Serialize for HebrewDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("HebrewDate", 3)?;
        state.serialize_field("day", &self.day.get())?;
        state.serialize_field("month", &self.month)?;
        state.serialize_field("year", &self.year())?;
        state.end()
    }
}
impl Eq for HebrewDate {}
impl PartialEq for HebrewDate {
    fn eq(&self, other: &HebrewDate) -> bool {
        self.day == other.day && self.month == other.month && self.year() == other.year()
    }
}

use std::cmp::Ordering;
impl Ord for HebrewDate {
    fn cmp(&self, other: &HebrewDate) -> Ordering {
        if self.year() < other.year() {
            Ordering::Less
        } else if self.year() > other.year() {
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
    /// Returns a HebrewDate on success or a ConversionError on failure.
    ///
    /// # Arguments
    /// * `year` - The Hebrew year since creation.
    /// * `month` - The Hebrew month.
    /// * `day` - The Hebrew day of month.
    ///
    /// # Error Values
    /// * `YearTooSmall` - This algorithm won't work if the year is before 3764.
    /// * `IsLeapYear` - I treat Adar, Adar 1 and Adar 2 as three seperate months, so if you want to
    /// convert a day in Adar 1 or Adar 2 of a leap year, specify which one.
    ///  * `IsNotLeapYear` - I treat Adar, Adar 1 and Adar 2 as three seperate months, so it won't
    ///  make sense to get the English date of the first of Adar 1 or Adar 2 if the year isn't a
    ///  leap year.
    ///  * `TooManyDaysInMonth` - There are either 29 or 30 days in a month, so it doesn't make sense
    ///  to find the 50th day of Nissan.
    pub fn from_ymd(
        year: u64,
        month: HebrewMonth,
        day: NonZeroI8,
    ) -> Result<HebrewDate, ConversionError> {
        HebrewYear::new(year)?.get_hebrew_date(month, day)
    }

    pub(crate) fn from_ymd_internal(
        month: HebrewMonth,
        day: NonZeroI8,
        hebrew_year: HebrewYear,
    ) -> Result<HebrewDate, ConversionError> {
        //Get a HebrewDate object from the Hebrew Year, Month, and Day. Can fail if the year is too
        //small or the day is less than one.
        if !hebrew_year.is_leap_year()
            && (month == HebrewMonth::Adar1 || month == HebrewMonth::Adar2)
        {
            return Err(ConversionError::IsNotLeapYear);
        }

        if hebrew_year.is_leap_year() && month == HebrewMonth::Adar {
            return Err(ConversionError::IsLeapYear);
        }

        if day.get() as u8 > hebrew_year.sched[month as usize] {
            return Err(ConversionError::TooManyDaysInMonth(
                hebrew_year.sched[month as usize],
            ));
        }

        Ok(HebrewDate {
            month,
            day: day,
            year: hebrew_year,
        })
    }

    fn from_gregorian(date: DateTime<Utc>) -> Result<HebrewDate, ConversionError> {
        if date < *crate::convert::year::backend::FIRST_RH + Duration::days(2 + 365) {
            return Err(ConversionError::YearTooSmall);
        }
        let days_since_first_rh =
            ((date - *crate::convert::year::backend::FIRST_RH).num_days() + 2) as u64;

        let hebrew_year = HebrewYear::new(crate::convert::year::backend::day_of_last_rh(
            days_since_first_rh,
        ))
        .unwrap();
        Ok(hebrew_year.get_hebrewdate_from_days_after_rh(days_since_first_rh))
    }

    pub(crate) fn to_gregorian(&self) -> chrono::DateTime<Utc> {
        let amnt_days_between_rh_and_epoch = self.year.days_since_epoch;
        let sched = self.year.sched;
        let mut amnt_days_in_month: u16 = 0;
        if self.month != HebrewMonth::Tishrei {
            for item in sched.iter().take(self.month as usize) {
                amnt_days_in_month += u16::from(*item);
            }
        }

        let amnt_days =
            amnt_days_between_rh_and_epoch + u64::from(amnt_days_in_month) + self.day.get() as u64
                - 1;
        *crate::convert::year::backend::EPOCH + Duration::days(amnt_days as i64)
    }
    ///Get the Hebrew day of month.
    #[inline]
    pub fn day(&self) -> NonZeroI8 {
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
        self.year.year
    }
}

mod tests {
    #[test]
    fn get_year() {
        use super::*;
        for j in 0..100 {
            let mut original_day = Utc.ymd(16 + j, 10, 4).and_hms(18, 0, 0);
            for _i in 1..366 {
                let h_day = HebrewDate::from_gregorian(original_day).unwrap();
                let ne_day = h_day.to_gregorian();
                assert_eq!(original_day, ne_day);
                original_day = original_day + Duration::days(1);
            }
        }
    }

}
