use smallvec::*;

use crate::convert::*;
use crate::holidays::get_chol_list;
use crate::holidays::get_shabbos_list;
use crate::holidays::get_special_parsha_list;
use crate::holidays::get_yt_list;

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
    /// use heca_lib::HebrewYear;
    /// use heca_lib::prelude::*;
    /// assert_eq!(HebrewYear::new(5779).unwrap().year_type(),MonthSchedule::BaShaZ);
    /// ~~~
    /// 
    /// ## Find out how often does Pesach start on which days:
    /// 
    /// ~~~
    /// use heca_lib::HebrewYear;
    /// use heca_lib::prelude::*;
    /// let (mut thu, mut tue, mut sun, mut sat) = (0,0,0,0);
    /// for year in 3765..9999 {
    ///     let t = HebrewYear::new(year).unwrap().year_type();
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

use std::convert::TryFrom;
impl TryFrom<chrono::DateTime<Utc>> for HebrewDate {
    type Error = ConversionError;
    fn try_from(original_day: chrono::DateTime<Utc>) -> Result<Self, Self::Error> {
        HebrewDate::from_gregorian(original_day)
    }
}

impl Into<chrono::DateTime<Utc>> for HebrewDate {
    fn into(self) -> chrono::DateTime<Utc> {
        self.to_gregorian()
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
                    y.get_hebrew_date(HebrewMonth::Nissan, 16)
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Thu
                ),

                MonthSchedule::HaShaG
                | MonthSchedule::ZaShaG
                | MonthSchedule::ZaChaG
                | MonthSchedule::BaChaG => assert_eq!(
                    y.get_hebrew_date(HebrewMonth::Nissan, 16)
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Tue
                ),
                MonthSchedule::HaShA | MonthSchedule::ZaChA | MonthSchedule::HaChA => assert_eq!(
                    y.get_hebrew_date(HebrewMonth::Nissan, 16)
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Sun
                ),
                MonthSchedule::HaKaZ | MonthSchedule::BaShaZ | MonthSchedule::GaKaZ => assert_eq!(
                    y.get_hebrew_date(HebrewMonth::Nissan, 16)
                        .unwrap()
                        .to_gregorian()
                        .weekday(),
                    Weekday::Sat
                ),
            }
        }
    }
}
