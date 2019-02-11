use std::fmt;

enum_from_primitive! {
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Day{
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Shabbos
}
}
enum_from_primitive! {
  #[derive(Debug, PartialEq, Copy, Clone)]
  pub enum HebrewMonth {
    Tishrei = 0,
    Cheshvan = 1,
    Kislev = 2,
    Teves = 3,
    Shvat = 4,
    Adar = 5,
    Adar1 = 6,
    Adar2 = 7,
    Nissan = 8,
    Iyar = 9,
    Sivan = 10,
    Tammuz = 11,
    Av = 12,
    Elul = 13
  }
}

impl HebrewMonth {
    pub fn month_list() -> Vec<&'static str> {
        vec![
            "Tishrei", "Cheshvan", "Kislev", "Teves", "Shvat", "Adar", "Adar1", "Adar2", "Nissan",
            "Iyar", "Sivan", "Tammuz", "Av", "Elul",
        ]
    }
    pub fn try_from(s: &str) -> Result<HebrewMonth, ConversionError> {
        match s {
            "Tishrei" => Ok(HebrewMonth::Tishrei),
            "Cheshvan" => Ok(HebrewMonth::Cheshvan),
            "Kislev" => Ok(HebrewMonth::Kislev),
            "Teves" => Ok(HebrewMonth::Teves),
            "Shvat" => Ok(HebrewMonth::Shvat),
            "Adar" => Ok(HebrewMonth::Adar),
            "Adar1" => Ok(HebrewMonth::Adar1),
            "Adar 1" => Ok(HebrewMonth::Adar1),
            "Adar Aleph" => Ok(HebrewMonth::Adar1),
            "Adar2" => Ok(HebrewMonth::Adar2),
            "Adar 2" => Ok(HebrewMonth::Adar2),
            "Adar Beis" => Ok(HebrewMonth::Adar2),
            "Nissan" => Ok(HebrewMonth::Nissan),
            "Iyar" => Ok(HebrewMonth::Iyar),
            "Sivan" => Ok(HebrewMonth::Sivan),
            "Tammuz" => Ok(HebrewMonth::Tammuz),
            "Av" => Ok(HebrewMonth::Av),
            "Elul" => Ok(HebrewMonth::Elul),
            _ => Err(ConversionError::MonthDoesntExist),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            HebrewMonth::Tishrei => "Tishrei",
            HebrewMonth::Cheshvan => "Cheshvan",
            HebrewMonth::Kislev => "Kislev",
            HebrewMonth::Teves => "Teves",
            HebrewMonth::Shvat => "Shvat",
            HebrewMonth::Adar => "Adar",
            HebrewMonth::Adar1 => "Adar 1",
            HebrewMonth::Adar2 => "Adar 2",
            HebrewMonth::Nissan => "Nissan",
            HebrewMonth::Iyar => "Iyar",
            HebrewMonth::Sivan => "Sivan",
            HebrewMonth::Tammuz => "Tammuz",
            HebrewMonth::Av => "Av",
            HebrewMonth::Elul => "Elul",
        }
    }
}

impl std::fmt::Display for HebrewMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.as_str();
        write!(f, "{}", string)
    }
}

#[derive(Debug)]

pub enum ConversionError {
    IsNotLeapYear,
    TooManyDaysInMonth(u8),
    IsLeapYear,
    MonthDoesntExist,
    YearTooSmall,
    DayIsZero,
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::types::ConversionError::*;
        match self {
        IsNotLeapYear => write!(f, "The year you specified is not a leap year, yet you're trying to convert from an Adar1 or Adar2. Use the regular Adar for a regular year"),
        TooManyDaysInMonth(d) => write!(f,"There aren't {} days in this month",d),
        IsLeapYear => write!(f, "The year you specified is a leap year, yet you're trying to convert from a Regular Adar. Use Adar1 or Adar2 on a leap year"),
        MonthDoesntExist => write!(f, "This month doesn't exist. Please specify another one."),
        YearTooSmall => write!(f, "Cannot work with a year this far back"),
        DayIsZero => write!(f, "Day cannot be zero")
        }
    }
}
