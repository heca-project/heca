use smallvec::*;

use crate::convert::*;
use crate::holidays::get_chol_list;
use crate::holidays::get_shabbos_list;
use crate::holidays::get_special_parsha_list;
use crate::holidays::get_yt_list;
use crate::prelude::*;

pub(crate) mod backend;
use crate::convert::year::backend::{
    get_rosh_hashana, months_per_year, return_year_sched, FIRST_YEAR, YEAR_SCHED,
};

/// HebrewYear holds data on a given year. It's faster to get multiple HebrewDates from
/// an existing HebrewYear rather than generating each one on its own.

#[derive(Copy, Clone, Debug)]
pub struct HebrewYear {
    pub(crate) year: u64,
    pub(crate) day_of_rh: Day,
    pub(crate) day_of_next_rh: Day,
    pub(crate) months_per_year: u64,
    pub(crate) sched: [u8; 14],
    pub(crate) year_len: u64,
    pub(crate) days_since_epoch: u64,
}

impl HebrewYear {
    /// Returns a new HebrewYear.
    ///
    /// # Arguments
    ///
    /// `year` - The Hebrew year
    ///
    #[inline]
    pub fn new(year: u64) -> Result<HebrewYear, ConversionError> {
        if year < FIRST_YEAR + 1 {
            Err(ConversionError::YearTooSmall)
        } else {
            let cur_rh = get_rosh_hashana(year);
            let next_rh = get_rosh_hashana(year + 1);
            let days_since_epoch = cur_rh.0;
            let amnt_days_in_year = next_rh.0 - cur_rh.0;
            let months_per_year = months_per_year(year);
            let sched = &YEAR_SCHED[return_year_sched(amnt_days_in_year)];

            Ok(HebrewYear {
                day_of_rh: get_rosh_hashana(year).1,
                year,
                day_of_next_rh: get_rosh_hashana(year + 1).1,
                months_per_year,
                sched: sched.clone(),
                days_since_epoch,
                year_len: amnt_days_in_year,
            })
        }
    }

    #[inline]
    pub fn is_leap_year(&self) -> bool {
        self.months_per_year == 13
    }
    #[inline]
    pub fn year(&self) -> u64 {
        self.year
    }
    /// Returns a HebrewDate from the current year and a supplied month and day.
    ///
    /// # Arguments:
    ///
    /// `month` - The Hebrew month.
    ///
    /// `day` - The day of the Hebrew month.
    ///
    #[inline]
    pub fn get_hebrew_date(
        self,
        month: HebrewMonth,
        day: u8,
    ) -> Result<HebrewDate, ConversionError> {
        HebrewDate::from_ymd_internal(month, day, self)
    }

    pub(crate) fn get_hebrewdate_from_days_after_rh(self, amnt_days: u64) -> HebrewDate {
        let mut remainder = amnt_days - self.days_since_epoch;
        let mut month: u64 = 0;
        for days_in_month in self.sched.iter() {
            if remainder < u64::from(*days_in_month) {
                break;
            }
            month += 1;
            remainder -= u64::from(*days_in_month);
        }
        HebrewDate {
            year: self,
            month: HebrewMonth::from(month),
            day: remainder as u8 + 1,
        }
    }
    /// Returns all the days when the Torah is read.
    ///
    /// # Arguments
    ///
    /// `location` - Specify if you're looking for the calendar in Israel or in the Diaspora. Is
    /// relevent as there's one day of Yom Tov in Israel and two outside. This also affects the
    /// Weekly parsha if the last day of Pesach or the second day of Shavuos is on Shabbos, when in
    /// Israel we move to the next Parsha while outside we're still reading the Yom Tov reading.
    ///
    /// `yt_types` - An array containing `TorahReadingType`. This should be used as a flag to
    /// specify which types of Torah readings you want to list.
    ///
    /// # Returns
    ///
    /// Returns an array (or a vec) of days.
    ///
    /// **Note**
    ///
    /// This may unsorted, and is returned under no defined order.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate heca_lib;
    ///
    /// use heca_lib::prelude::*;
    /// use heca_lib::{HebrewDate, HebrewYear};
    ///
    /// let year = HebrewYear::new(5779).unwrap();
    /// let shabbosim = year.get_holidays(Location::Chul, &[TorahReadingType::Shabbos, TorahReadingType::SpecialParsha, TorahReadingType::Chol, TorahReadingType::YomTov]);
    /// let mut count = 0;
    /// for s in shabbosim.into_iter() {
    ///   if s.name() == TorahReading::Shabbos(Parsha::Bereishis) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Tishrei, 27).unwrap());
    ///     count += 1;
    ///   }
    ///   else if s.name() == TorahReading::SpecialParsha(SpecialParsha::Zachor) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Adar2, 9).unwrap());
    ///     count += 1;
    ///   }
    ///   else if s.name() == TorahReading::Chol(Chol::Chanukah1) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Kislev, 25).unwrap());
    ///     count += 1;
    ///   }
    ///   else if s.name() == TorahReading::YomTov(YomTov::Shavuos1) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Sivan, 6).unwrap());
    ///     count += 1;
    ///   }
    /// }
    /// assert_eq!(count,4);
    /// ```
    pub fn get_holidays(
        &self,
        location: Location,
        yt_types: &[TorahReadingType],
    ) -> SmallVec<[TorahReadingDay; 256]> {
        let mut return_vec: SmallVec<[TorahReadingDay; 256]> = SmallVec::new();
        if yt_types.contains(&TorahReadingType::YomTov) {
            return_vec.extend_from_slice(&get_yt_list(self.clone(), location));
        }
        if yt_types.contains(&TorahReadingType::Chol) {
            return_vec.extend_from_slice(&get_chol_list(self.clone()));
        }
        if yt_types.contains(&TorahReadingType::Shabbos) {
            return_vec.extend_from_slice(&get_shabbos_list(self.clone(), location));
        }
        if yt_types.contains(&TorahReadingType::SpecialParsha) {
            return_vec.extend_from_slice(&get_special_parsha_list(self.clone()));
        }
        return_vec
    }
}

mod test {
    use super::*;
    #[test]
    fn make_new_year() {
        for i in 4000..5000 {
            println!("{}", i);
            HebrewYear::new(i).unwrap();
        }
    }
}
