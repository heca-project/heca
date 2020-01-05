use crate::args::types::{DayVal, Language, Name};
use heca_lib::prelude::HebrewMonth;
use heca_lib::HebrewYear;
use std::convert::TryInto;
use std::io::{BufWriter, StdoutLock, Write};
use std::num::NonZeroI8;

pub fn get(year: &HebrewYear) -> Vec<DayVal> {
    let mut return_vec = vec![];
    get_yud_kislev(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    get_yud_tes_kislev(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    get_chof_kislev(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    get_yud_beis_tammuz(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    get_yud_gimmel_tammuz(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    return_vec
}

fn get_yud_kislev(year: &HebrewYear) -> Option<DayVal> {
    if year.year() < 5588 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(10).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::ChabadHoliday(ChabadHoliday::YudKislev),
        })
    }
}

fn get_yud_tes_kislev(year: &HebrewYear) -> Option<DayVal> {
    if year.year() <= 5559 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(19).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::ChabadHoliday(ChabadHoliday::YudTesKislev),
        })
    }
}

fn get_chof_kislev(year: &HebrewYear) -> Option<DayVal> {
    if year.year() <= 5559 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(20).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::ChabadHoliday(ChabadHoliday::ChofKislev),
        })
    }
}

fn get_yud_beis_tammuz(year: &HebrewYear) -> Option<DayVal> {
    if year.year() <= 5687 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tammuz, NonZeroI8::new(12).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::ChabadHoliday(ChabadHoliday::YudBeisTammuz),
        })
    }
}

fn get_yud_gimmel_tammuz(year: &HebrewYear) -> Option<DayVal> {
    if year.year() <= 5687 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tammuz, NonZeroI8::new(13).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::ChabadHoliday(ChabadHoliday::YudGimmelTammuz),
        })
    }
}

#[derive(Debug, Clone)]
pub enum ChabadHoliday {
    YudKislev,
    YudTesKislev,
    ChofKislev,
    YudBeisTammuz,
    YudGimmelTammuz,
}

impl ChabadHoliday {
    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let p = match language {
            Language::English => match self {
                Self::YudKislev => lock.write(b"Yud Kislev").ok()?,
                Self::YudTesKislev => lock.write(b"Yud Tes Kislev").ok()?,
                Self::ChofKislev => lock.write(b"Chof Kislev").ok()?,
                Self::YudBeisTammuz => lock.write(b"Yud Beis Tammuz").ok()?,
                Self::YudGimmelTammuz => lock.write(b"Yud Gimmel Tammuz").ok()?,
            },
            Language::Hebrew => match self {
                Self::YudKislev => lock.write("י בכסלו".as_bytes()).ok()?,
                Self::YudTesKislev => lock.write("י\"ט כסלו".as_bytes()).ok()?,
                Self::ChofKislev => lock.write("כ כסלו".as_bytes()).ok()?,
                Self::YudBeisTammuz => lock.write("י\"ב תמוז".as_bytes()).ok()?,
                Self::YudGimmelTammuz => lock.write("י\"ג תמוז".as_bytes()).ok()?,
            },
        };
        Some(p)
    }
}
