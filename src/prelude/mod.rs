mod location;
#[doc(inline)]
pub use location::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Shabbos,
}

impl From<u64> for Day {
    fn from(input: u64) -> Self {
        match input {
            0 => Day::Sunday,
            1 => Day::Monday,
            2 => Day::Tuesday,
            3 => Day::Wednesday,
            4 => Day::Thursday,
            5 => Day::Friday,
            6 => Day::Shabbos,
            _ => panic!(format!("{} Is out of bounds", input)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd)]
pub enum HebrewMonth {
    Tishrei,
    Cheshvan,
    Kislev,
    Teves,
    Shvat,
    Adar,
    Adar1,
    Adar2,
    Nissan,
    Iyar,
    Sivan,
    Tammuz,
    Av,
    Elul,
}
impl From<u64> for HebrewMonth {
    fn from(input: u64) -> Self {
        match input {
            0 => HebrewMonth::Tishrei,
            1 => HebrewMonth::Cheshvan,
            2 => HebrewMonth::Kislev,
            3 => HebrewMonth::Teves,
            4 => HebrewMonth::Shvat,
            5 => HebrewMonth::Adar,
            6 => HebrewMonth::Adar1,
            7 => HebrewMonth::Adar2,
            8 => HebrewMonth::Nissan,
            9 => HebrewMonth::Iyar,
            10 => HebrewMonth::Sivan,
            11 => HebrewMonth::Tammuz,
            12 => HebrewMonth::Av,
            13 => HebrewMonth::Elul,
            _ => panic!(format!("{} Is out of bounds", input)),
        }
    }
}

///Occurs when failing to get a Hebrew Date.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum ConversionError {
    /// Occurs when attempting to get an Adar 1 or Adar 2 in a non-leap year.
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroI8;
    /// #
    /// let result = HebrewDate::from_ymd(5778,HebrewMonth::Adar1,NonZeroI8::new(1).unwrap());
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::IsNotLeapYear);
    /// ```
    IsNotLeapYear,

    /// Occurs when trying to get a Hebrew Date who's day is out of range
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroI8;
    /// #
    /// let result = HebrewDate::from_ymd(5778,HebrewMonth::Adar,NonZeroI8::new(40).unwrap());
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::TooManyDaysInMonth(29));
    /// ```
    TooManyDaysInMonth(u8),

    /// Occurs when attempting to get a regular Adar in a leap year.
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroI8;
    /// #
    /// let result = HebrewDate::from_ymd(5779,HebrewMonth::Adar,NonZeroI8::new(1).unwrap());
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::IsLeapYear);
    /// ```
    IsLeapYear,
    /// Occurs when attempting to get a year that is before the epoch (currently: year 3764/4).
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroI8;
    /// #
    /// let result = HebrewDate::from_ymd(2448,HebrewMonth::Nissan,NonZeroI8::new(15).unwrap()); // What was the English day of the Exodus?
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::YearTooSmall);
    /// ```
    YearTooSmall,
}

impl std::error::Error for ConversionError {}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::IsNotLeapYear => write!(
                f,
                "Can't convert an Adar 1 or Adar 2 of a year which isn't a leap year"
            ),
            ConversionError::TooManyDaysInMonth(d) => {
                write!(f, "Too many days in month. Month only has {} days", d)
            }
            ConversionError::IsLeapYear => write!(
                f,
                "Can't convert an Adar of a year which is a leap year. Specify Adar1 or Adar2"
            ),
            //ConversionError::MonthDoesntExist => write!(f, "Month doesn't exist"),
            ConversionError::YearTooSmall => write!(
                f,
                "Cannot build calendar for years below 3764 (After Creation)"
            ),
        }
    }
}


/// What Torah Readings are we looking for
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum TorahReadingType {
    /// Yom Tov - Pesach, Shavuos, Sukkos, Shmini Atzeres/Simchas Torah, Rosh Hashana, Yom Kippur and Chol HaMoed.
    YomTov,
    /// Weekday Torah reading - Rosh Chodesh, Chanuka and Purim
    Chol,
    /// Weekly Parsha Torah reading
    Shabbos,
    /// One of the four special Torah portions read every winter (Shekalim, Zachor, Parah and HaChodesh).
    SpecialParsha,
}

    /// A Hebrew year can be defined by three variables:
    /// 
    /// 1. The first day of Rosh Hashana - Monday (the second day of the week, represented by Beis - **Ba**), Tuesday (the third day of the week, represented by Gimmel - **Ga**), Thursday (the fifth day of the week, represented by Hei - **Ha**) and Shabbos (the seventh day of the week, represented by Zayin - **Za**).
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum MonthSchedule {
    BaChaG,
    BaShaH,
    GaChaH,
    HaKaZ,
    HaShA,
    ZaChA,
    ZaShaG,

    BaChaH,
    BaShaZ,
    GaKaZ,
    HaChA,
    HaShaG,
    ZaChaG,
    ZaShaH,
}
