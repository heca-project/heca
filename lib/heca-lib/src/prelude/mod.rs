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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConversionError {
    IsNotLeapYear,
    TooManyDaysInMonth(u8),
    IsLeapYear,
    MonthDoesntExist,
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
            ConversionError::TooManyDaysInMonth(d) => write!(f, "Too many days ({}) in month", d),
            ConversionError::IsLeapYear => write!(
                f,
                "Can't convert an Adar of a year which is a leap year. Specify Adar1 or Adar2"
            ),
            ConversionError::MonthDoesntExist => write!(f, "Month doesn't exist"),
            ConversionError::YearTooSmall => write!(
                f,
                "Cannot build calendar for years below 3764 (After Creation)"
            ),
        }
    }
}

/// There are four types of Torah Readings:
///
/// 1. Yom Tov.
/// 2. Shabbos - the weekly Parsha.
/// 3. Chol (such as Chanuka and Purim).
/// 4. Special Parshas (there are four extra portions read every winter).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum TorahReadingType {
    YomTov,
    Chol,
    Shabbos,
    SpecialParsha,
}

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
