use crate::args::types::{DayVal, Name, MinorDays};
use heca_lib::HebrewYear;

use heca_lib::prelude::{HebrewMonth};


use std::num::NonZeroI8;
use smallvec::{smallvec, SmallVec};

pub fn get_minor_holidays(year: &HebrewYear) -> SmallVec<[DayVal; 16]> {
    let mut holidays = smallvec![
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(9).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevYomKippur)
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevSukkos)
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevPesach)
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::PesachSheni),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(18).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::LagBaOmer),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(5).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevShavuos),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, NonZeroI8::new(29).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevRoshHashanah),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Shvat, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::FifteenShvat),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::FifteenAv),
        },
    ];

    if year.is_leap_year() {
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::PurimKattan),
        });
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ShushanPurimKattan),
        });
    }

    holidays
}