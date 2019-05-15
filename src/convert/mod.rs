use chrono::prelude::*;
use time::Duration;

use crate::prelude::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

mod year;
#[doc(inline)]
pub use year::*;

#[derive(Debug, Copy, Clone)]
pub struct HebrewDate {
    day: u8,
    month: HebrewMonth,
    year: HebrewYear,
}
impl Serialize for HebrewDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("HebrewDate", 3)?;
        state.serialize_field("day", &self.day)?;
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
    /// Returns a HebrewDate on success, or a ConversionError on failure.
    ///
    /// # Arguments
    /// * `year` - The Hebrew year since creation.
    /// * `month` - The Hebrew month.
    /// * `day` - The Hebrew day of month.
    ///
    /// # Error Values
    /// * `YearTooSmall` - This algorithm won't work if the year is before 3764.
    /// * `DayIsZero` - Months start with day 1, not zero. So 0 Adar won't work.
    /// * `IsLeapYear` - I treat Adar, Adar 1 and Adar 2 as three seperate months, so if you want to
    /// convert a day in Adar 1 or Adar 2 of a leap year, specify which one.
    ///  * `IsNotLeapYear` - I treat Adar, Adar 1 and Adar 2 as three seperate months, so it won't
    ///  make sense to get the English date of the first of Adar 1 or Adar 2 if the year isn't a
    ///  leap year.
    ///  * `TooManyDaysInMonth` - There are either 29 or 30 days in a month, so it doesn't make sense
    ///  to find the 50th day of Nissan.
    pub fn from_ymd(year: u64, month: HebrewMonth, day: u8) -> Result<HebrewDate, ConversionError> {
        HebrewYear::new(year)?.get_hebrew_date(month, day)
    }

    pub fn from_ymd_unchecked(year: u64, month: HebrewMonth, day: u8) -> HebrewDate {
        HebrewYear::new(year)
            .unwrap()
            .get_hebrew_date(month, day)
            .unwrap()
    }

    pub(crate) fn from_ymd_internal(
        month: HebrewMonth,
        day: u8,
        hebrew_year: HebrewYear,
    ) -> Result<HebrewDate, ConversionError> {
        //Get a HebrewDate object from the Hebrew Year, Month, and Day. Can fail if the year is too
        //small or the day is less than one.

        if day == 0 {
            return Err(ConversionError::DayIsZero);
        }

        if !hebrew_year.is_leap_year()
            && (month == HebrewMonth::Adar1 || month == HebrewMonth::Adar2)
        {
            return Err(ConversionError::IsNotLeapYear);
        }

        if day > hebrew_year.sched[month as usize] {
            return Err(ConversionError::TooManyDaysInMonth(
                hebrew_year.sched[month as usize],
            ));
        }

        Ok(HebrewDate {
            month,
            day: day as u8,
            year: hebrew_year,
        })
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
    /// use heca_lib::prelude::*;
    /// use heca_lib::HebrewDate;
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
    /// use heca_lib::prelude::*;
    /// use heca_lib::HebrewDate;
    ///
    /// assert_eq!(HebrewDate::from_gregorian(Utc.ymd(2018,9,10).and_hms(18,0,0)).unwrap(),HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,2).unwrap());
    /// ```
    /// # Error Values:
    /// * YearTooSmall - This algorithm won't work if the year is before year 4.
    ///
    pub fn from_gregorian(date: DateTime<Utc>) -> Result<HebrewDate, ConversionError> {
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

    /// Gets the Gregorian date for the current Hebrew date.
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
    /// use heca_lib::prelude::*;
    /// use heca_lib::HebrewDate;
    ///
    /// assert_eq!(HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,10).unwrap().to_gregorian(),Utc.ymd(2018, 9,18).and_hms(18,00,00));
    /// ```
    /// ## Algorithm:
    /// The conversion is done (at the moment) according to the calculation of the Rambam (Maimonidies), as is documented in [Hilchos Kiddush Ha'chodesh](https://www.sefaria.org/Mishneh_Torah%2C_Sanctification_of_the_New_Month.6.1?lang=bi&with=all&lang2=en).
    ///
    /// The algorithm is as follows:
    ///
    /// 1. There are exactly 1080 Chalakim (parts) in an hour.
    /// 2. There are exactly (well, not really. But it's close enough that we use that number as exact.) 29 days, 12 hours, and 793 Chalakim between new moons.
    ///
    ///  So that's the basic numbers. Regarding the calendar itself:
    ///
    /// 3. All months are either 29 or 30 days long.
    /// 4. There are either 12 or 13 months in the Hebrew calendar, depending if it's a leap year. When it's a leap year, Adar (which generally is in the late winter or early spring) is doubled into a "first Adar" (Adar1) and a "second Adar" (Adar2).
    /// 5. There is a 19 year cycle of leap years. So the first two years of the cycle are regular years, the one after that's a leap year. Then another two are regular, then a leap year. Then it's regular, leap, regular, regular, leap, regular, regular, leap.
    /// 6. Year 3763 was the first year of its 19 year cycle.
    /// 7. Now you can calculate when's the New Moon before a given Rosh Hashana.
    ///
    ///  So how to calculate Rosh Hashana:
    ///
    /// 8. If the New Moon is in the afternoon, Rosh Hashana is postponed to the next day.
    /// 9. If Rosh Hashana's starting on a Sunday (Saturday night), Wednesday (Tuesday night), or Friday (Thursday night) - postpone it by a day.
    ///
    ///  If any of the above two conditions were fulfilled. Good. You just found Rosh Hashana. If not:
    ///
    /// 10. If the New Moon is on a Tuesday after 3am+204 Chalakim and the coming year is not a leap year, Rosh Hashana is postponed to that upcoming Thursday instead.
    /// 11. If the New Moon is on a Monday after 9am+589 Chalakim, and the previous year was a leap year, then Rosh Hashana is postponed to Tuesday.
    ///
    ///
    ///  Now you have all the Rosh Hashanas.
    ///
    /// 12. In general, months alternate between “Full” (30 days long) and “Empty” (29 days long) months. So Tishrei is full, Teves is empty, Shvat is full, Adar is empty, Nissan is full.
    /// 13. When the year is a leap year, Adar 1 is full and Adar 2 is empty. (So a full Shvat is followed by a full Adar1).
    ///
    ///  Knowing this, you can calculate any other date of the year.
    ///
    ///  But wait! We're playing with the date when Rosh Hashana will start, so not every year will be the same length! How do we make up these days?
    ///
    ///  So there's a last little bit:
    ///
    /// 14. Cheshvan and Kislev are variable length months – some years both are full, some years both are empty, and some years Cheshvan is full and Kislev is empty - depending on the day Rosh Hashana starts (and the day _the next Rosh Hashana starts_) and how many days are in the year.

    pub fn to_gregorian(&self) -> chrono::DateTime<Utc> {
        let amnt_days_between_rh_and_epoch = self.year.days_since_epoch;
        let sched = self.year.sched;
        let mut amnt_days_in_month: u16 = 0;
        if self.month != HebrewMonth::Tishrei {
            for item in sched.iter().take(self.month as usize) {
                amnt_days_in_month += u16::from(*item);
            }
        }

        let amnt_days =
            amnt_days_between_rh_and_epoch + u64::from(amnt_days_in_month) + u64::from(self.day)
                - 1;
        *crate::convert::year::backend::EPOCH + Duration::days(amnt_days as i64)
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
        self.year.year
    }
}

mod tests {
    use super::*;
    #[test]
    fn get_year() {
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
