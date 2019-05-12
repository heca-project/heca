mod location;
#[doc(inline)]
pub use location::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
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
    DayIsZero,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum TorahReadingType {
    YomTov,
    Chol,
    Shabbos,
    SpecialParsha,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum MonthSchedule {
    Short,
    Regular,
    Year,
}
