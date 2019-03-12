use smallvec::*;

use crate::convert;
use crate::convert::*;
use crate::holidays::get_chol_list;
use crate::holidays::get_shabbos_list;
use crate::holidays::get_special_parsha_list;
use crate::holidays::get_yt_list;

/// HebrewYear holds data on a given year. Hypothetically, it's faster to get multiple HebrewDates from
/// an existing HebrewYear rather than generating each one on its own.
pub struct HebrewYear {
    year: u64,
    leap_year: bool,
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
        if year < convert::FIRST_YEAR {
            Err(ConversionError::YearTooSmall)
        } else {
            Ok(HebrewYear {
                year,
                leap_year: months_per_year(year) == 13,
            })
        }
    }

    #[inline]
    pub fn is_leap_year(&self) -> bool {
        self.leap_year
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
    pub fn get_hebrew_date(
        &self,
        month: HebrewMonth,
        day: u8,
    ) -> Result<HebrewDate, ConversionError> {
        HebrewDate::from_ymd(self.year, month, day)
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
        yt_types
            .iter()
            .map(|yt_type| match yt_type {
                TorahReadingType::YomTov => get_yt_list(self.year, location),
                TorahReadingType::Chol => get_chol_list(self.year),
                TorahReadingType::Shabbos => get_shabbos_list(self.year, location),
                TorahReadingType::SpecialParsha => get_special_parsha_list(self.year),
            })
            .for_each(|r| return_vec.extend_from_slice(&r));
        return_vec
    }
}
