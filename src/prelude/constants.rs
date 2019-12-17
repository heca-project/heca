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
                NonZeroI8::new(day_of_month_of_shabbos_chazon + 7).unwrap(),
            )
            .unwrap()
            .into(),
        name: Name::MinorDays(MinorDays::ShabbosNachamu),
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
    });

    holidays
}

pub const GEMARAS_SECOND_CYCLE: [(&'static str, &'static str, &'static str, u8); 37] = [
    ("Berakhot", "ברכות", "ברכות", 64),
    ("Shabbat", "שבת", "שבת", 157),
    ("Eruvin", "עירובין", "עירובין", 105),
    ("Pesachim", "פסחים", "פסחים", 121),
    ("Shekalim", "שקלים", "שקלים", 22),
    ("Yoma", "יומא", "יומא", 88),
    ("Sukkah", "סוכה", "סוכה", 56),
    ("Beitzah", "ביצה", "ביצה", 40),
    ("Rosh Hashanah", "ראש השנה", "ראשהשנה", 35),
    ("Taanit", "תענית", "תענית", 31),
    ("Megillah", "מגילה", "מגילה", 32),
    ("Moed Katan", "מועד קטן", "מועדקטן", 29),
    ("Chagigah", "חגיגה", "חגיגה", 27),
    ("Yevamot", "יבמות", "יבמות", 122),
    ("Ketubot", "כתובות", "כתובות", 112),
    ("Nedarim", "נדרים", "נדרים", 91),
    ("Nazir", "נזיר", "נזיר", 66),
    ("Sotah", "סוטה", "סוטה", 49),
    ("Gittin", "גיטין", "גיטין", 90),
    ("Kiddushin", "קידושין", "קידושין", 82),
    ("Bava Kamma", "בבא קמא", "בבאקמא", 119),
    ("Bava Metzia", "בבא מציעא", "בבאמציעא", 119),
    ("Bava Batra", "בבא בתרא", "בבאבתרא", 176),
    ("Sanhedrin", "סנהדרין", "סנהדרין", 113),
    ("Makkot", "מכות", "מכות", 24),
    ("Shevuot", "שבועות", "שבועות", 49),
    ("Avodah Zarah", "עבודה זרה", "עבודהזרה", 76),
    ("Horayot", "הוריות", "הוריות", 14),
    ("Zevachim", "זבחים", "זבחים", 120),
    ("Menachot", "מנחות", "מנחות", 110),
    ("Chullin", "חולין", "חולין", 142),
    ("Bekhorot", "בכורות", "בכורות", 61),
    ("Arakhin", "ערכין", "ערכין", 34),
    ("Temurah", "תמורה", "תמורה", 34),
    ("Keritot", "כריתות", "כריתות", 28),
    ("Meilah", "מעילה", "מעילה", 37),
    ("Niddah", "נדה", "נדה", 73),
];

pub const GEMARAS_FIRST_CYCLE: [(&'static str, &'static str, &'static str, u8); 37] = [
    ("Berakhot", "ברכות", "ברכות", 64),
    ("Shabbat", "שבת", "שבת", 157),
    ("Eruvin", "עירובין", "עירובין", 105),
    ("Pesachim", "פסחים", "פסחים", 121),
    ("Shekalim", "שקלים", "שקלים", 13),
    ("Yoma", "יומא", "יומא", 88),
    ("Sukkah", "סוכה", "סוכה", 56),
    ("Beitzah", "ביצה", "ביצה", 40),
    ("Rosh Hashanah", "ראש השנה", "ראשהשנה", 35),
    ("Taanit", "תענית", "תענית", 31),
    ("Megillah", "מגילה", "מגילה", 32),
    ("Moed Katan", "מועד קטן", "מועדקטן", 29),
    ("Chagigah", "חגיגה", "חגיגה", 27),
    ("Yevamot", "יבמות", "יבמות", 122),
    ("Ketubot", "כתובות", "כתובות", 112),
    ("Nedarim", "נדרים", "נדרים", 91),
    ("Nazir", "נזיר", "נזיר", 66),
    ("Sotah", "סוטה", "סוטה", 49),
    ("Gittin", "גיטין", "גיטין", 90),
    ("Kiddushin", "קידושין", "קידושין", 82),
    ("Bava Kamma", "בבא קמא", "בבאקמא", 119),
    ("Bava Metzia", "בבא מציעא", "בבאמציעא", 119),
    ("Bava Batra", "בבא בתרא", "בבאבתרא", 176),
    ("Sanhedrin", "סנהדרין", "סנהדרין", 113),
    ("Makkot", "מכות", "מכות", 24),
    ("Shevuot", "שבועות", "שבועות", 49),
    ("Avodah Zarah", "עבודה זרה", "עבודהזרה", 76),
    ("Horayot", "הוריות", "הוריות", 14),
    ("Zevachim", "זבחים", "זבחים", 120),
    ("Menachot", "מנחות", "מנחות", 110),
    ("Chullin", "חולין", "חולין", 142),
    ("Bekhorot", "בכורות", "בכורות", 61),
    ("Arakhin", "ערכין", "ערכין", 34),
    ("Temurah", "תמורה", "תמורה", 34),
    ("Keritot", "כריתות", "כריתות", 28),
    ("Meilah", "מעילה", "מעילה", 37),
    ("Niddah", "נדה", "נדה", 73),
];

pub const RAMBAM: [(&'static str, &'static str, &'static str, u8); 88] = [
    (
        "Transmission of the Oral Law",
        "מסירת תורה שבעל פה",
        "מסירתתורהשבעלפה",
        3,
    ),
    (
        "Positive Mitzvot",
        "מצוות עשה",
        "מצוותעשה",
        3,
    ),
    (
        "Negative Mitzvot",
        "מצוות לא תעשה",
        "מצוותלאתעשה",
        3,
    ),
    (
        "Overview of Mishneh Torah Contents",
        "תוכן החיבור",
        "תוכןהחיבור",
        3,
    ),
    (
        "Foundations of the Torah",
        "הלכות יסודי התורה",
        "הלכותיסודיהתורה",
        10,
    ),
    (
        "Human Dispositions",
        "הלכות דעות",
        "הלכותדעות",
        7,
    ),
    (
        "Torah Study",
        "הלכות תלמוד תורה",
        "הלכותתלמודתורה",
        7,
    ),
    (
        "Foreign Worship and Customs of the Nations",
        "הלכות עבודה זרה וחוקות הגויים",
        "הלכותעבודהזרהוחוקותהגויים",
        12,
    ),
    (
        "Repentance",
        "הלכות תשובה",
        "הלכותתשובה",
        10,
    ),
    (
        "Reading the Shema",
        "הלכות קריאת שמע",
        "הלכותקריאתשמע",
        4,
    ),
    (
        "Prayer and the Priestly Blessing",
        "הלכות תפילה וברכת כהנים",
        "הלכותתפילהוברכתכהנים",
        15,
    ),
    (
        "Tefillin, Mezuzah and the Torah Scroll",
        "הלכות תפילין ומזוזה וספר תורה",
        "הלכותתפיליןומזוזהוספרתורה",
        10,
    ),
    (
        "Fringes",
        "הלכות ציצית",
        "הלכותציצית",
        3,
    ),
    (
        "Blessings",
        "הלכות ברכות",
        "הלכותברכות",
        11,
    ),
    (
        "Circumcision",
        "הלכות מילה",
        "הלכותמילה",
        3,
    ),
    (
        "The Order of Prayer",
        "סדר התפילה",
        "סדרהתפילה",
        4,
    ),
    ("Sabbath", "הלכות שבת", "הלכותשבת", 30),
    (
        "Eruvin",
        "הלכות עירובין",
        "הלכותעירובין",
        8,
    ),
    (
        "Rest on the Tenth of Tishrei",
        "הלכות שביתת עשור",
        "הלכותשביתתעשור",
        3,
    ),
    (
        "Rest on a Holiday",
        "הלכות שביתת יום טוב",
        "הלכותשביתתיוםטוב",
        8,
    ),
    (
        "Leavened and Unleavened Bread",
        "הלכות חמץ ומצה",
        "הלכותחמץומצה",
        9,
    ),
    (
        "Shofar, Sukkah and Lulav",
        "הלכות שופר וסוכה ולולב",
        "הלכותשופרוסוכהולולב",
        8,
    ),
    (
        "Sheqel Dues",
        "הלכות שקלים",
        "הלכותשקלים",
        4,
    ),
    (
        "Sanctification of the New Month",
        "הלכות קידוש החודש",
        "הלכותקידושהחודש",
        19,
    ),
    (
        "Fasts",
        "הלכות תעניות",
        "הלכותתעניות",
        5,
    ),
    (
        "Scroll of Esther and Hanukkah",
        "הלכות מגילה וחנוכה",
        "הלכותמגילהוחנוכה",
        4,
    ),
    (
        "Marriage",
        "הלכות אישות",
        "הלכותאישות",
        25,
    ),
    (
        "Divorce",
        "הלכות גירושין",
        "הלכותגירושין",
        13,
    ),
    (
        "Levirate Marriage and Release",
        "הלכות יבום וחליצה",
        "הלכותיבוםוחליצה",
        8,
    ),
    (
        "Virgin Maiden",
        "הלכות נערה בתולה",
        "הלכותנערהבתולה",
        3,
    ),
    (
        "Woman Suspected of Infidelity",
        "הלכות סוטה",
        "הלכותסוטה",
        4,
    ),
    (
        "Forbidden Intercourse",
        "הלכות איסורי ביאה",
        "הלכותאיסוריביאה",
        22,
    ),
    (
        "Forbidden Foods",
        "הלכות מאכלות אסורות",
        "הלכותמאכלותאסורות",
        17,
    ),
    (
        "Ritual Slaughter",
        "הלכות שחיטה",
        "הלכותשחיטה",
        14,
    ),
    (
        "Oaths",
        "הלכות שבועות",
        "הלכותשבועות",
        12,
    ),
    ("Vows", "הלכות נדרים", "הלכותנדרים", 13),
    (
        "Nazariteship",
        "הלכות נזירות",
        "הלכותנזירות",
        10,
    ),
    (
        "Appraisals and Devoted Property",
        "הלכות ערכים וחרמין",
        "הלכותערכיםוחרמין",
        8,
    ),
    (
        "Diverse Species",
        "הלכות כלאים",
        "הלכותכלאים",
        10,
    ),
    (
        "Gifts to the Poor",
        "הלכות מתנות עניים",
        "הלכותמתנותעניים",
        10,
    ),
    (
        "Heave Offerings",
        "הלכות תרומות",
        "הלכותתרומות",
        15,
    ),
    (
        "Tithes",
        "הלכות מעשרות",
        "הלכותמעשרות",
        14,
    ),
    (
        "Second Tithes and Fourth Year's Fruit",
        "הלכות מעשר שני ונטע רבעי",
        "הלכותמעשרשניונטערבעי",
        11,
    ),
    (
        "First Fruits and other Gifts to Priests Outside the Sanctuary",
        "הלכות ביכורים ושאר מתנות כהונה שבגבולין",
        "הלכותביכוריםושארמתנותכהונהשבגבולין",
        12,
    ),
    (
        "Sabbatical Year and the Jubilee",
        "הלכות שמיטה ויובל",
        "הלכותשמיטהויובל",
        13,
    ),
    (
        "The Chosen Temple",
        "הלכות בית הבחירה",
        "הלכותביתהבחירה",
        8,
    ),
    (
        "Vessels of the Sanctuary and Those who Serve Therein",
        "הלכות כלי המקדש והעובדין בו",
        "הלכותכליהמקדשוהעובדיןבו",
        10,
    ),
    (
        "Admission into the Sanctuary",
        "הלכות ביאת מקדש",
        "הלכותביאתמקדש",
        9,
    ),
    (
        "Things Forbidden on the Altar",
        "הלכות איסורי המזבח",
        "הלכותאיסוריהמזבח",
        7,
    ),
    (
        "Sacrificial Procedure",
        "הלכות מעשה הקרבנות",
        "הלכותמעשההקרבנות",
        19,
    ),
    (
        "Daily Offerings and Additional Offerings",
        "הלכות תמידים ומוספין",
        "הלכותתמידיםומוספין",
        10,
    ),
    (
        "Sacrifices Rendered Unfit",
        "הלכות פסולי המוקדשין",
        "הלכותפסוליהמוקדשין",
        19,
    ),
    (
        "Service on the Day of Atonement",
        "הלכות עבודת יום הכפורים",
        "הלכותעבודתיוםהכפורים",
        5,
    ),
    (
        "Trespass",
        "הלכות מעילה",
        "הלכותמעילה",
        8,
    ),
    (
        "Paschal Offering",
        "הלכות קרבן פסח",
        "הלכותקרבןפסח",
        10,
    ),
    (
        "Festival Offering",
        "הלכות חגיגה",
        "הלכותחגיגה",
        3,
    ),
    (
        "Firstlings",
        "הלכות בכורות",
        "הלכותבכורות",
        8,
    ),
    (
        "Offerings for Unintentional Transgressions",
        "הלכות שגגות",
        "הלכותשגגות",
        15,
    ),
    (
        "Offerings for Those with Incomplete Atonement",
        "הלכות מחוסרי כפרה",
        "הלכותמחוסריכפרה",
        5,
    ),
    (
        "Substitution",
        "הלכות תמורה",
        "הלכותתמורה",
        4,
    ),
    (
        "Defilement by a Corpse",
        "הלכות טומאת מת",
        "הלכותטומאתמת",
        25,
    ),
    (
        "Red Heifer",
        "הלכות פרה אדומה",
        "הלכותפרהאדומה",
        15,
    ),
    (
        "Defilement by Leprosy",
        "הלכות טומאת צרעת",
        "הלכותטומאתצרעת",
        16,
    ),
    (
        "Those Who Defile Bed or Seat",
        "הלכות מטמאי משכב ומושב",
        "הלכותמטמאימשכבומושב",
        13,
    ),
    (
        "Other Sources of Defilement",
        "הלכות שאר אבות הטומאות",
        "הלכותשאראבותהטומאות",
        20,
    ),
    (
        "Defilement of Foods",
        "הלכות טומאת אוכלים",
        "הלכותטומאתאוכלים",
        16,
    ),
    ("Vessels", "הלכות כלים", "הלכותכלים", 28),
    (
        "Immersion Pools",
        "הלכות מקואות",
        "הלכותמקואות",
        11,
    ),
    (
        "Damages to Property",
        "הלכות נזקי ממון",
        "הלכותנזקיממון",
        14,
    ),
    ("Theft", "הלכות גניבה", "הלכותגניבה", 9),
    (
        "Robbery and Lost Property",
        "הלכות גזילה ואבידה",
        "הלכותגזילהואבידה",
        18,
    ),
    (
        "One Who Injures a Person or Property",
        "הלכות חובל ומזיק",
        "הלכותחובלומזיק",
        8,
    ),
    (
        "Murderer and the Preservation of Life",
        "הלכות רוצח ושמירת נפש",
        "הלכותרוצחושמירתנפש",
        13,
    ),
    ("Sales", "הלכות מכירה", "הלכותמכירה", 30),
    (
        "Ownerless Property and Gifts",
        "הלכות זכייה ומתנה",
        "הלכותזכייהומתנה",
        12,
    ),
    (
        "Neighbors",
        "הלכות שכנים",
        "הלכותשכנים",
        14,
    ),
    (
        "Agents and Partners",
        "הלכות שלוחין ושותפין",
        "הלכותשלוחיןושותפין",
        10,
    ),
    ("Slaves", "הלכות עבדים", "הלכותעבדים", 9),
    (
        "Hiring",
        "הלכות שכירות",
        "הלכותשכירות",
        13,
    ),
    (
        "Borrowing and Deposit",
        "הלכות שאלה ופיקדון",
        "הלכותשאלהופיקדון",
        8,
    ),
    (
        "Creditor and Debtor",
        "הלכות מלווה ולווה",
        "הלכותמלווהולווה",
        27,
    ),
    (
        "Plaintiff and Defendant",
        "הלכות טוען ונטען",
        "הלכותטועןונטען",
        16,
    ),
    (
        "Inheritances",
        "הלכות נחלות",
        "הלכותנחלות",
        11,
    ),
    (
        "The Sanhedrin and the Penalties within their Jurisdiction",
        "הלכות סנהדרין והעונשין המסורין להם",
        "הלכותסנהדריןוהעונשיןהמסוריןלהם",
        26,
    ),
    ("Testimony", "הלכות עדות", "הלכותעדות", 22),
    ("Rebels", "הלכות ממרים", "הלכותממרים", 7),
    ("Mourning", "הלכות אבל", "הלכותאבל", 14),
    (
        "Kings and Wars",
        "הלכות מלכים ומלחמות",
        "הלכותמלכיםומלחמות",
        12,
    ),
];

#[test]
fn ensure_right_amount_of_daf_first_cycle() {
    assert_eq!(
        GEMARAS_FIRST_CYCLE
            .iter()
            .fold(0, |old, new| { old + new.3 as u64 - 1 }),
        2702
    );
}

#[test]
fn ensure_right_amount_of_daf_second_cycle() {
    assert_eq!(
        GEMARAS_SECOND_CYCLE
            .iter()
            .fold(0, |old, new| { old + new.3 as u64 - 1 }),
        2711
    );
}

#[test]
fn ensure_right_amount_of_perakim() {
    use heca_lib::HebrewDate;
    use std::convert::TryInto;
    let day_one_of_rambam_5779 =
        HebrewDate::from_ymd(5779, HebrewMonth::Av, NonZeroI8::new(5).unwrap()).unwrap();
    let last_day_of_rambam_5780 =
        HebrewDate::from_ymd(5780, HebrewMonth::Tammuz, NonZeroI8::new(17).unwrap()).unwrap();
    let day_one_of_rambam_5779: DateTime<Utc> = day_one_of_rambam_5779.try_into().unwrap();
    let last_day_of_rambam_5780: DateTime<Utc> = last_day_of_rambam_5780.into();
    assert_eq!(
        RAMBAM
            .iter()
            .fold(0, |old, rambam| { old + rambam.3 as i64 }),
        ((last_day_of_rambam_5780 - day_one_of_rambam_5779).num_days() + 1) * 3
    );
}
