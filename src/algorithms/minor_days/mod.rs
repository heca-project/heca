use crate::algorithms::minor_days::types::MinorDays;
use crate::args::types::{DayVal, Name};
use crate::prelude::Json;
use crate::prelude::JsonPrinter;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use chrono::Weekday;
use heca_lib::prelude::HebrewMonth;
use heca_lib::HebrewYear;
use std::num::NonZeroI8;

pub mod constants;
pub mod types;

pub fn get(year: &HebrewYear) -> Vec<DayVal> {
    let mut holidays = vec![
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(9).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevYomKippur),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevSukkos),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevPesach),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::PesachSheni),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(18).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::LagBaOmer),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(5).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevShavuos),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, NonZeroI8::new(29).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevRoshHashanah),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Shvat, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::FifteenShvat),
            candle_lighting: None,
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::FifteenAv),
            candle_lighting: None,
        },
    ];

    if year.is_leap_year() {
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::PurimKattan),
            candle_lighting: None,
        });
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ShushanPurimKattan),
            candle_lighting: None,
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
        candle_lighting: None,
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
        candle_lighting: None,
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
        candle_lighting: None,
    });

    holidays.push(DayVal {
        day: year
            .get_hebrew_date(
                HebrewMonth::Av,
                NonZeroI8::new(day_of_month_of_shabbos_chazon + 7).unwrap(),
            )
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::ShabbosNachamu),
        candle_lighting: None,
    });

    let day_of_rh: DateTime<Utc> = year
        .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(1).unwrap())
        .unwrap()
        .into();

    let day_of_month_of_shabbos_shuva = match day_of_rh.weekday() {
        Weekday::Sun => 6,
        Weekday::Mon => 5,
        Weekday::Wed => 3,
        Weekday::Fri => 8,
        x => panic!("Shabbos Shuva shouldn't be on {}", x),
    };

    let day_of_erev_rh: DateTime<Utc> = year
        .get_hebrew_date(HebrewMonth::Elul, NonZeroI8::new(29).unwrap())
        .unwrap()
        .into();

    let day_of_month_of_leil_slichos = match day_of_erev_rh.weekday() {
        Weekday::Sun => 21,
        Weekday::Tue => 26,
        Weekday::Thu => 24,
        Weekday::Sat => 22,
        x => panic!("Leil Slichos shouldn't be on {}", x),
    };
    holidays.push(DayVal {
        day: year
            .get_hebrew_date(
                HebrewMonth::Elul,
                NonZeroI8::new(day_of_month_of_leil_slichos).unwrap(),
            )
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::LeilSlichos),
        candle_lighting: None,
    });
    holidays.push(DayVal {
        day: year
            .get_hebrew_date(
                HebrewMonth::Tishrei,
                NonZeroI8::new(day_of_month_of_shabbos_shuva).unwrap(),
            )
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::ShabbosShuva),
        candle_lighting: None,
    });

    holidays
}

impl JsonPrinter for MinorDays {
    fn json_print(&self, json: &mut Json<'_, '_>) {
        json.print_map_unchecked("type", "MinorDays");
        json.next();
        json.print_map_unchecked("name", self.to_json_string());
    }
}
