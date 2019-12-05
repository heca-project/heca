use crate::args::types::{DayVal, MinorDays, Name};
use heca_lib::HebrewYear;

use heca_lib::prelude::HebrewMonth;

use chrono::prelude::*;
use chrono::DateTime;
use smallvec::{smallvec, SmallVec};
use std::num::NonZeroI8;

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
    let first_day_of_pesach: DateTime<Utc> = year
        .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(15).unwrap())
        .unwrap()
        .into();
    let first_day_of_pesach = first_day_of_pesach.weekday();
    let day_in_nissan = match first_day_of_pesach {
        Weekday::Sat => 14,
        Weekday::Mon => 12,
        Weekday::Wed => 10,
        Weekday::Fri => 8,
        _ => panic!("Pesach shouldn't fall out on a {}", first_day_of_pesach),
    };
    holidays.push(DayVal {
        day: year
            .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(day_in_nissan).unwrap())
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::ShabbosHaGadol),
    });

    let day_of_taanis_bechoros = if first_day_of_pesach == Weekday::Sat {
        12
    } else {
        14
    };

    holidays.push(DayVal {
        day: year
            .get_hebrew_date(
                HebrewMonth::Nissan,
                NonZeroI8::new(day_of_taanis_bechoros).unwrap(),
            )
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::TaanisBechoros),
    });

    let day_of_tisha_beav: DateTime<Utc> = year
        .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(9).unwrap())
        .unwrap()
        .into();
    let day_of_month_of_shabbos_chazon = match day_of_tisha_beav.weekday() {
        Weekday::Sat => 8,
        Weekday::Mon => 6,
        Weekday::Wed => 4,
        Weekday::Fri => 9,
        x => panic!("Tisha Beav shouldn't be on {}", x),
    };
    holidays.push(DayVal {
        day: year
            .get_hebrew_date(
                HebrewMonth::Av,
                NonZeroI8::new(day_of_month_of_shabbos_chazon).unwrap(),
            )
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::ShabbosChazon),
    });

    holidays.push(DayVal {
        day: year
            .get_hebrew_date(
                HebrewMonth::Av,
                NonZeroI8::new(day_of_month_of_shabbos_chazon+7).unwrap(),
            )
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::ShabbosNachamu),
    });


    holidays
}
