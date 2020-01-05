use crate::args::types::{DayVal, Language, Name};

use heca_lib::prelude::{HebrewMonth, MonthSchedule};
use heca_lib::HebrewYear;
use std::convert::TryInto;
use std::io::{BufWriter, StdoutLock, Write};
use std::num::NonZeroI8;

pub fn get(year: &HebrewYear, exact_days: bool) -> Vec<DayVal> {
    let mut return_vec = vec![];
    return_vec.extend_from_slice(&get_yom_haatzmaut_and_yom_hazikaron(year, exact_days));
    get_yom_yerushalayim(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    get_yom_hashoah(year, exact_days).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    get_yom_haaliyah(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    get_sigd(year).and_then(|x| Some(return_vec.extend(std::iter::once(x))));
    return_vec
}

fn get_yom_haaliyah(year: &HebrewYear) -> Option<DayVal> {
    if year.year() < 5777 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroI8::new(7).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::IsraeliHoliday(IsraeliHoliday::YomHaAliyah),
        })
    }
}

fn get_yom_yerushalayim(year: &HebrewYear) -> Option<DayVal> {
    if year.year() < 5727 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(28).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::IsraeliHoliday(IsraeliHoliday::YomYerushalayim),
        })
    }
}

fn get_sigd(year: &HebrewYear) -> Option<DayVal> {
    if year.year() < 5769 {
        None
    } else {
        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroI8::new(29).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::IsraeliHoliday(IsraeliHoliday::Sigd),
        })
    }
}

fn get_yom_hashoah(year: &HebrewYear, exact_days: bool) -> Option<DayVal> {
    if year.year() < 5711 {
        None
    } else {
        let offset = if exact_days {
            0
        } else {
            match year.year_type() {
                MonthSchedule::BaChaG
                | MonthSchedule::HaShaG
                | MonthSchedule::ZaChaG
                | MonthSchedule::ZaShaG => 1,
                MonthSchedule::GaKaZ | MonthSchedule::BaShaZ | MonthSchedule::HaKaZ => 0,
                MonthSchedule::BaChaH
                | MonthSchedule::BaShaH
                | MonthSchedule::GaChaH
                | MonthSchedule::ZaShaH => 0,
                MonthSchedule::ZaChA | MonthSchedule::HaShA | MonthSchedule::HaChA => -1,
            }
        };

        Some(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(27 + offset).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            name: Name::IsraeliHoliday(IsraeliHoliday::YomHaShoah),
        })
    }
}

fn get_yom_haatzmaut_and_yom_hazikaron(year: &HebrewYear, exact_days: bool) -> Vec<DayVal> {
    if year.year() < 5709 {
        return vec![];
    }
    let offset = if exact_days {
        0
    } else {
        match year.year_type() {
            MonthSchedule::BaChaG
            | MonthSchedule::HaShaG
            | MonthSchedule::ZaChaG
            | MonthSchedule::ZaShaG => {
                if year.year() < 5764 {
                    0
                } else {
                    1
                }
            }
            MonthSchedule::GaKaZ | MonthSchedule::BaShaZ | MonthSchedule::HaKaZ => -1,
            MonthSchedule::BaChaH
            | MonthSchedule::BaShaH
            | MonthSchedule::GaChaH
            | MonthSchedule::ZaShaH => 0,
            MonthSchedule::ZaChA | MonthSchedule::HaShA | MonthSchedule::HaChA => -2,
        }
    };

    let yom_hazikaron = DayVal {
        day: year
            .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(4 + offset).unwrap())
            .unwrap()
            .try_into()
            .unwrap(),
        name: Name::IsraeliHoliday(IsraeliHoliday::YomHaZikaron),
    };
    let yom_haatzmaut = DayVal {
        day: year
            .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(5 + offset).unwrap())
            .unwrap()
            .try_into()
            .unwrap(),
        name: Name::IsraeliHoliday(IsraeliHoliday::YomHaAtzmaut),
    };
    vec![yom_hazikaron, yom_haatzmaut]
}

#[derive(Debug, Clone)]
pub enum IsraeliHoliday {
    YomHaZikaron,
    YomHaAtzmaut,
    YomYerushalayim,
    YomHaShoah,
    YomHaAliyah,
    Sigd,
}

impl IsraeliHoliday {
    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let p = match language {
            Language::English => match self {
                Self::YomHaAtzmaut => lock.write(b"Yom HaAtzmaut").ok()?,
                Self::YomHaZikaron => lock.write(b"Yom HaZikaron").ok()?,
                Self::YomYerushalayim => lock.write(b"Yom Yerushalayim").ok()?,
                Self::YomHaShoah => lock.write(b"Yom HaShoah").ok()?,
                Self::YomHaAliyah => lock.write(b"Yom HaAliyah").ok()?,
                Self::Sigd => lock.write(b"Sigd").ok()?,
            },
            Language::Hebrew => match self {
                Self::YomHaAtzmaut => lock.write("יום העצמאות".as_bytes()).ok()?,
                Self::YomHaZikaron => lock.write("יום הזיכרון".as_bytes()).ok()?,
                Self::YomYerushalayim => lock.write("יום ירושלים".as_bytes()).ok()?,
                Self::YomHaShoah => lock.write("יום השואה".as_bytes()).ok()?,
                Self::YomHaAliyah => lock.write("יום העלייה".as_bytes()).ok()?,
                Self::Sigd => lock.write("סיגד".as_bytes()).ok()?,
            },
        };
        Some(p)
    }
}
