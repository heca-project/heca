use crate::convert;
use crate::convert::*;
use crate::holidays::get_torah_reading_days_list;
use crate::holidays::*;
use crate::types::*;
use std::borrow::Cow;

pub struct HebrewYear {
    year: u64,
}

impl HebrewYear {
    #[inline]
    pub fn new(year: u64) -> Result<HebrewYear, ConversionError> {
        if year < convert::FIRST_YEAR {
            Err(ConversionError::YearTooSmall)
        } else {
            Ok(HebrewYear { year })
        }
    }

    #[inline]
    pub fn add_year(&mut self) {
        self.year += 1;
    }
    #[inline]
    pub fn year(&self) -> u64 {
        self.year
    }
    pub fn to_hebrew_date(
        &self,
        month: HebrewMonth,
        day: u8,
    ) -> Result<HebrewDate, ConversionError> {
        HebrewDate::from_ymd(self.year, month, day)
    }

    pub fn get_holidays(&self, yt_type: YomTovType) -> Cow<'static, [SpecialDay]> {
        match yt_type {
            YomTovType::YomTov => get_yt_list(self.year),
            YomTovType::SpecialTorahReading => get_torah_reading_days_list(self.year),
            YomTovType::RegularTorahReading => vec![SpecialDay {
                day: HebrewDate::from_ymd(self.year, HebrewMonth::Tishrei, 30).unwrap(),
                name: TorahReading::Shabbos(Parsha::YomTov),
            }]
            .into(),
        }
    }
}

pub enum YomTovType {
    YomTov,
    SpecialTorahReading,
    RegularTorahReading,
}
