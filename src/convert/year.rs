use std::convert::TryFrom;

use smallvec::*;

use crate::convert::*;
use crate::holidays::get_chol_list;
use crate::holidays::get_shabbos_list;
use crate::holidays::get_special_parsha_list;
use crate::holidays::get_yt_list;
use std::num::NonZeroI8;

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
    /// Returns if this year is a leap year.
    ///
    /// ```
    /// use heca_lib::HebrewYear;
    /// assert_eq!(HebrewYear::new(5779).unwrap().is_leap_year(),true);
    /// ```
    pub fn is_leap_year(&self) -> bool {
        self.months_per_year == 13
    }

    #[inline]
    /// Returns the type of year.
    ///
    /// A Hebrew year can be one of 14 combinations of length and starting day.
    ///
    /// # Returns
    ///
    /// A Hebrew Year can be defined by three variables:
    ///
    /// 1. The first day of Rosh Hashana - Monday (the second day of the week, represented by Beis - *Ba**), Tuesday (the third day of the week, represented by Gimmel - **Ga**), Thursday (the fifth day of the week, represented by Hei - **Ha**) and Shabbos (the seventh day of the week, represented by Zayin - **Za**).
    /// 2. The length of the year, specifically, if Cheshvan and Kislev are both full (**She**leima - 30 days long), empty (**Chaseir** - 29 days long), or in regular order ("Kesidra", Cheshvan is 29 days long and Kislev is 30. So the year goes 30,29,30,29 etc.).
    /// 3. The day Pesach starts, defined as on Rosh Hashana above.
    ///
    /// So, for example, 5779 is a BaShaZ year - that is, the first day of Rosh Hashana was on a Monday (Beis - **Ba**), Bosh Cheshvan and Kislev are full (Shleimah - **Sh**in),
    /// and the first night of Pesach was on Friday night (Zain - **Z** for Shabbos).
    ///
    /// # Examples
    ///
    ///
    ///
    /// ~~~
    ///
    /// use heca_lib::HebrewYear;
    /// use heca_lib::prelude::*;
    /// assert_eq!(HebrewYear::new(5779)?.year_type(),MonthSchedule::BaShaZ);
    /// # Ok::<(),ConversionError>(())
    /// ~~~
    ///
    /// ## Find out how often does Pesach start on which days:
    ///
    /// ~~~
    ///
    /// use heca_lib::HebrewYear;
    /// use heca_lib::prelude::*;
    /// let (mut thu, mut tue, mut sun, mut sat) = (0,0,0,0);
    /// for year in 3765..9999 {
    ///     let t = HebrewYear::new(year)?.year_type();
    ///     match t {
    ///         MonthSchedule::GaChaH
    ///         | MonthSchedule::BaShaH
    ///         | MonthSchedule::BaChaH
    ///         | MonthSchedule::ZaShaH => thu += 1,
    ///
    ///         MonthSchedule::HaShaG
    ///         | MonthSchedule::ZaShaG
    ///         | MonthSchedule::ZaChaG
    ///         | MonthSchedule::BaChaG => tue += 1,
    ///
    ///         MonthSchedule::HaShA
    ///         | MonthSchedule::ZaChA
    ///         | MonthSchedule::HaChA => sun += 1,
    ///         
    ///         MonthSchedule::HaKaZ
    ///         | MonthSchedule::BaShaZ
    ///         | MonthSchedule::GaKaZ => sat += 1,
    ///     }
    /// }
    /// assert_eq!(thu, 1782);
    /// assert_eq!(tue, 1988);
    /// assert_eq!(sun, 718); // <-- Note, that Pesach falls out on a Motzei Shabbos only 10% of the time.
    /// assert_eq!(sat, 1746);
    /// # Ok::<(),ConversionError>(())
    ///
    ///
    /// ~~~
    ///
    /// ## Find out when will Pesach start on Motzei Shabbos:
    ///
    /// ~~~
    /// use heca_lib::HebrewYear;
    /// use heca_lib::prelude::*;
    /// let mut years: Vec<u64> = Vec::new();
    /// for year in 5780..5880 {
    ///     let t = HebrewYear::new(year).unwrap().year_type();
    ///     match t {
    ///         MonthSchedule::HaShA
    ///         | MonthSchedule::ZaChA
    ///         | MonthSchedule::HaChA => years.push(year),
    ///
    ///         _ => { }
    ///         
    ///     }
    /// }
    /// assert_eq!(years, vec![5781, 5785, 5805, 5808, 5812, 5832, 5835, 5839, 5859, 5863] ); // <-- We'll have two of them over the next few years, and then Pesach won't fall out on Motzei Shabbos for twenty years!
    ///
    /// ~~~
    ///
    pub fn year_type(&self) -> MonthSchedule {
        if self.months_per_year == 12 {
            match self.day_of_rh {
                Day::Monday => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        MonthSchedule::BaShaH
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        MonthSchedule::BaChaG
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, stars on Monday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Tuesday => {
                    if self.sched[1] == 29 && self.sched[2] == 30 {
                        MonthSchedule::GaChaH
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, starts on Tuesday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Thursday => {
                    if self.sched[1] == 29 && self.sched[2] == 30 {
                        MonthSchedule::HaKaZ
                    } else if self.sched[1] == 30 && self.sched[2] == 30 {
                        MonthSchedule::HaShA
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, starts on Thursday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Shabbos => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        MonthSchedule::ZaShaG
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        MonthSchedule::ZaChA
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, stars on Shabbos, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                x => panic!(format!("Rosh Hashana should never fall out on {:?}", x)),
            }
        } else {
            match self.day_of_rh {
                Day::Monday => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        MonthSchedule::BaShaZ
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        MonthSchedule::BaChaH
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, stars on Monday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Tuesday => {
                    if self.sched[1] == 29 && self.sched[2] == 30 {
                        MonthSchedule::GaKaZ
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, starts on Tuesday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Thursday => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        MonthSchedule::HaShaG
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        MonthSchedule::HaChA
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, starts on Thursday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Shabbos => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        MonthSchedule::ZaShaH
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        MonthSchedule::ZaChaG
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, stars on Shabbos, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                x => panic!(format!("Rosh Hashana should never fall out on {:?}", x)),
            }
        }
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
        day: NonZeroI8,
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
            day: NonZeroI8::new((remainder + 1) as i8).unwrap(),
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
    /// use std::num::NonZeroI8;
    /// use heca_lib::prelude::*;
    /// use heca_lib::{HebrewDate, HebrewYear};
    /// let year = HebrewYear::new(5779)?;
    /// let shabbosim = year.get_holidays(Location::Chul, &[TorahReadingType::Shabbos, TorahReadingType::SpecialParsha, TorahReadingType::Chol, TorahReadingType::YomTov]);
    /// let mut count = 0;
    /// for s in shabbosim.into_iter() {
    ///   if s.name() == TorahReading::Shabbos(Parsha::Bereishis) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Tishrei, NonZeroI8::new(27).unwrap())?);
    ///     count += 1;
    ///   }
    ///   else if s.name() == TorahReading::SpecialParsha(SpecialParsha::Zachor) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Adar2, NonZeroI8::new(9).unwrap())?);
    ///     count += 1;
    ///   }
    ///   else if s.name() == TorahReading::Chol(Chol::Chanukah1) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Kislev, NonZeroI8::new(25).unwrap())?);
    ///     count += 1;
    ///   }
    ///   else if s.name() == TorahReading::YomTov(YomTov::Shavuos1) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,HebrewMonth::Sivan, NonZeroI8::new(6).unwrap())?);
    ///     count += 1;
    ///   }
    /// }
    /// assert_eq!(count,4);
    /// # Ok::<(),ConversionError>(())
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

/// Returns a HebrewDate on success, or a ConversionError on failure.
///
/// # Arguments
/// * `date` - The Gregorian date.
///
/// # Note:
/// Hebrew days start at sundown, not midnight, so there isn't a full 1:1 mapping between
/// Gregorian days and Hebrew. So when you look up the date of Rosh Hashana 5779, most calendars will say that it's on Monday the 10th of September, 2018, while Rosh Hashana really started at sundown on the 9th of September.
///
/// I'm trying to be a _bit_ more precise, so I made the date cutoff at 6:00 PM. So fore xample:
///
/// ```
/// use std::num::NonZeroI8;
/// use std::convert::TryInto;
///
/// use chrono::Utc;
/// use chrono::offset::TimeZone;
/// use heca_lib::prelude::*;
/// use heca_lib::HebrewDate;
///
/// let hebrew_date: HebrewDate = Utc.ymd(2018,9,10).and_hms(17,59,59).try_into()?;
/// assert_eq!(hebrew_date,HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,NonZeroI8::new(1).unwrap())?);
/// # Ok::<(),ConversionError>(())
/// ```
///
/// while
///
/// ```
/// use std::num::NonZeroI8;
/// use std::convert::TryInto;
///
/// use chrono::Utc;
/// use chrono::offset::TimeZone;
/// use heca_lib::prelude::*;
/// use heca_lib::HebrewDate;
///
///
/// let hebrew_date: HebrewDate = Utc.ymd(2018,9,10).and_hms(18,0,0).try_into()?;
/// assert_eq!(hebrew_date, HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,NonZeroI8::new(2).unwrap())?);
/// # Ok::<(),ConversionError>(())
/// ```
/// # Error Values:
/// * YearTooSmall - This algorithm won't work if the year is before year 4.
///
impl TryFrom<chrono::DateTime<Utc>> for HebrewDate {
    type Error = ConversionError;
    fn try_from(original_day: chrono::DateTime<Utc>) -> Result<HebrewDate, ConversionError> {
        HebrewDate::from_gregorian(original_day)
    }
}

/// Gets the Gregorian date for the current Hebrew date.
///
/// # Notes
///
/// This function returns the DateTime of the given HebrewDate at nightfall.
///
/// For example, Yom Kippur 5779 started at sunset of September 18, 2018. So
/// ```
/// use std::num::NonZeroI8;
///
/// use chrono::prelude::*;
/// use heca_lib::prelude::*;
/// use heca_lib::HebrewDate;
///
/// let gregorian_date: DateTime<Utc> = HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,NonZeroI8::new(10).unwrap())?.into();
/// assert_eq!(gregorian_date ,Utc.ymd(2018, 9,18).and_hms(18,00,00));
/// # Ok::<(),ConversionError>(())
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
impl From<HebrewDate> for chrono::DateTime<Utc> {
    fn from(h: HebrewDate) -> Self {
        h.to_gregorian()
    }
}

mod test {
    #[test]
    fn make_new_year() {
        use super::*;

        for i in 4000..5000 {
            println!("{}", i);
            HebrewYear::new(i).unwrap();
        }
    }
    #[test]
    fn check_year_type() {
        use super::*;

        for i in 3765..9999 {
            println!("{}", i);
            let y = HebrewYear::new(i).unwrap();
            let t = y.year_type();
            match t {
                MonthSchedule::GaChaH
                | MonthSchedule::BaShaH
                | MonthSchedule::BaChaH
                | MonthSchedule::ZaShaH => assert_eq!(
                    y.get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(16).unwrap())
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Thu
                ),

                MonthSchedule::HaShaG
                | MonthSchedule::ZaShaG
                | MonthSchedule::ZaChaG
                | MonthSchedule::BaChaG => assert_eq!(
                    y.get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(16).unwrap())
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Tue
                ),
                MonthSchedule::HaShA | MonthSchedule::ZaChA | MonthSchedule::HaChA => assert_eq!(
                    y.get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(16).unwrap())
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Sun
                ),
                MonthSchedule::HaKaZ | MonthSchedule::BaShaZ | MonthSchedule::GaKaZ => assert_eq!(
                    y.get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(16).unwrap())
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Sat
                ),
            }
        }
    }
    #[test]
    fn hi() {
        use crate::convert::HebrewDate;
        use chrono::prelude::*;
        use std::convert::TryInto;

        let o = Utc.ymd(1990, 2, 3).and_hms(1, 2, 3);
        let h: Result<HebrewDate, _> = o.try_into();
    }
}
