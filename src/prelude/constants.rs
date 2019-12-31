use crate::args::types::{DayVal, MinorDays, Name};
use heca_lib::HebrewYear;

use heca_lib::prelude::HebrewMonth;

use chrono::prelude::*;
use chrono::DateTime;
use std::num::NonZeroI8;

pub fn get_minor_holidays(year: &HebrewYear) -> Vec<DayVal> {
    let mut holidays = vec![
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(9).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevYomKippur),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevSukkos),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevPesach),
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

macro_rules! make_list {
    ($name:ident, $val:expr) => {
        pub const $name: [(&str, &str, &str, u8); 37] = [
            ("Berakhot", "ברכות", "Berakhot", 64),
            ("Shabbat", "שבת", "Shabbat", 157),
            ("Eruvin", "עירובין", "Eruvin", 105),
            ("Pesachim", "פסחים", "Pesachim", 121),
            ("Shekalim", "שקלים", "Shekalim", $val),
            ("Yoma", "יומא", "Yoma", 88),
            ("Sukkah", "סוכה", "Sukkah", 56),
            ("Beitzah", "ביצה", "Beitzah", 40),
            ("Rosh Hashanah", "ראש השנה", "RoshHashanah", 35),
            ("Taanit", "תענית", "Taanit", 31),
            ("Megillah", "מגילה", "Megillah", 32),
            ("Moed Katan", "מועד קטן", "MoedKatan", 29),
            ("Chagigah", "חגיגה", "Chagigah", 27),
            ("Yevamot", "יבמות", "Yevamot", 122),
            ("Ketubot", "כתובות", "Ketubot", 112),
            ("Nedarim", "נדרים", "Nedarim", 91),
            ("Nazir", "נזיר", "Nazir", 66),
            ("Sotah", "סוטה", "Sotah", 49),
            ("Gittin", "גיטין", "Gittin", 90),
            ("Kiddushin", "קידושין", "Kiddushin", 82),
            ("Bava Kamma", "בבא קמא", "BavaKamma", 119),
            ("Bava Metzia", "בבא מציעא", "BavaMetzia", 119),
            ("Bava Batra", "בבא בתרא", "BavaBatra", 176),
            ("Sanhedrin", "סנהדרין", "Sanhedrin", 113),
            ("Makkot", "מכות", "Makkot", 24),
            ("Shevuot", "שבועות", "Shevuot", 49),
            ("Avodah Zarah", "עבודה זרה", "AvodahZarah", 76),
            ("Horayot", "הוריות", "Horayot", 14),
            ("Zevachim", "זבחים", "Zevachim", 120),
            ("Menachot", "מנחות", "Menachot", 110),
            ("Chullin", "חולין", "Chullin", 142),
            ("Bekhorot", "בכורות", "Bekhorot", 61),
            ("Arakhin", "ערכין", "Arakhin", 34),
            ("Temurah", "תמורה", "Temurah", 34),
            ("Keritot", "כריתות", "Keritot", 28),
            ("Meilah", "מעילה", "Meilah", 37),
            ("Niddah", "נדה", "Niddah", 73),
        ];
    };
}

make_list!(GEMARAS_FIRST_CYCLE, 13);
make_list!(GEMARAS_SECOND_CYCLE, 22);

#[test]
fn check_rambam_json_title() {
    for g in RAMBAM.iter() {
        assert_eq!(
            &g.0.split(|c| c == ' ' || c == ',' || c == '\'')
                .collect::<String>()
                .to_uppercase(),
            &g.2.to_uppercase()
        );
    }
}

#[test]
fn check_yerushalmi_json_title() {
    for g in YERUSHALMI.iter() {
        assert_eq!(
            &g.0.split(|c| c == ' ' || c == ',' || c == '\'')
                .collect::<String>()
                .to_uppercase(),
            &g.2.to_uppercase()
        );
    }
}

#[test]
fn check_json_yerushalmi() {
    for g in YERUSHALMI.iter() {
        for i in g.2.chars() {
            if !i.is_alphabetic() {
                panic!("{:?} is an invalid json {}", g, i);
            }
        }
    }
}

#[test]
fn check_json_gemara() {
    for g in GEMARAS_FIRST_CYCLE.iter() {
        for i in g.2.chars() {
            if !i.is_alphabetic() {
                panic!("{:?} is an invalid json {}", g, i);
            }
        }
    }
}

#[test]
fn check_json_rambam() {
    for g in RAMBAM.iter() {
        for i in g.2.chars() {
            if !i.is_alphabetic() {
                panic!("{:?} is an invalid json {}", g, i);
            }
        }
    }
}

pub const RAMBAM: [(&str, &str, &str, u8); 88] = [
    (
        "Transmission of the Oral Law",
        "מסירת תורה שבעל פה",
        "TransmissionOfTheOralLaw",
        3,
    ),
    ("Positive Mitzvot", "מצוות עשה", "PositiveMitzvot", 3),
    ("Negative Mitzvot", "מצוות לא תעשה", "NegativeMitzvot", 3),
    (
        "Overview of Mishneh Torah Contents",
        "תוכן החיבור",
        "OverviewOfMishnehTorahContents",
        3,
    ),
    (
        "Foundations of the Torah",
        "הלכות יסודי התורה",
        "FoundationsOfTheTorah",
        10,
    ),
    ("Human Dispositions", "הלכות דעות", "HumanDispositions", 7),
    ("Torah Study", "הלכות תלמוד תורה", "TorahStudy", 7),
    (
        "Foreign Worship and Customs of the Nations",
        "הלכות עבודה זרה וחוקות הגויים",
        "ForeignWorshipAndCustomsOfTheNations",
        12,
    ),
    ("Repentance", "הלכות תשובה", "Repentance", 10),
    ("Reading the Shema", "הלכות קריאת שמע", "ReadingTheShema", 4),
    (
        "Prayer and the Priestly Blessing",
        "הלכות תפילה וברכת כהנים",
        "PrayerAndThePriestlyBlessing",
        15,
    ),
    (
        "Tefillin, Mezuzah and the Torah Scroll",
        "הלכות תפילין ומזוזה וספר תורה",
        "TefillinMezuzahAndTheTorahScroll",
        10,
    ),
    ("Fringes", "הלכות ציצית", "Fringes", 3),
    ("Blessings", "הלכות ברכות", "Blessings", 11),
    ("Circumcision", "הלכות מילה", "Circumcision", 3),
    ("The Order of Prayer", "סדר התפילה", "TheOrderOfPrayer", 4),
    ("Sabbath", "הלכות שבת", "Sabbath", 30),
    ("Eruvin", "הלכות עירובין", "Eruvin", 8),
    (
        "Rest on the Tenth of Tishrei",
        "הלכות שביתת עשור",
        "RestOnTheTenthOfTishrei",
        3,
    ),
    (
        "Rest on a Holiday",
        "הלכות שביתת יום טוב",
        "RestOnAHoliday",
        8,
    ),
    (
        "Leavened and Unleavened Bread",
        "הלכות חמץ ומצה",
        "LeavenedAndUnleavenedBread",
        9,
    ),
    (
        "Shofar, Sukkah and Lulav",
        "הלכות שופר וסוכה ולולב",
        "ShofarSukkahAndLulav",
        8,
    ),
    ("Sheqel Dues", "הלכות שקלים", "SheqelDues", 4),
    (
        "Sanctification of the New Month",
        "הלכות קידוש החודש",
        "SanctificationOfTheNewMonth",
        19,
    ),
    ("Fasts", "הלכות תעניות", "Fasts", 5),
    (
        "Scroll of Esther and Hanukkah",
        "הלכות מגילה וחנוכה",
        "ScrollOfEstherAndHanukkah",
        4,
    ),
    ("Marriage", "הלכות אישות", "Marriage", 25),
    ("Divorce", "הלכות גירושין", "Divorce", 13),
    (
        "Levirate Marriage and Release",
        "הלכות יבום וחליצה",
        "LevirateMarriageAndRelease",
        8,
    ),
    ("Virgin Maiden", "הלכות נערה בתולה", "VirginMaiden", 3),
    (
        "Woman Suspected of Infidelity",
        "הלכות סוטה",
        "WomanSuspectedOfInfidelity",
        4,
    ),
    (
        "Forbidden Intercourse",
        "הלכות איסורי ביאה",
        "ForbiddenIntercourse",
        22,
    ),
    (
        "Forbidden Foods",
        "הלכות מאכלות אסורות",
        "ForbiddenFoods",
        17,
    ),
    ("Ritual Slaughter", "הלכות שחיטה", "RitualSlaughter", 14),
    ("Oaths", "הלכות שבועות", "Oaths", 12),
    ("Vows", "הלכות נדרים", "Vows", 13),
    ("Nazariteship", "הלכות נזירות", "Nazariteship", 10),
    (
        "Appraisals and Devoted Property",
        "הלכות ערכים וחרמין",
        "AppraisalsAndDevotedProperty",
        8,
    ),
    ("Diverse Species", "הלכות כלאים", "DiverseSpecies", 10),
    (
        "Gifts to the Poor",
        "הלכות מתנות עניים",
        "GiftsToThePoor",
        10,
    ),
    ("Heave Offerings", "הלכות תרומות", "HeaveOfferings", 15),
    ("Tithes", "הלכות מעשרות", "Tithes", 14),
    (
        "Second Tithes and Fourth Year's Fruit",
        "הלכות מעשר שני ונטע רבעי",
        "SecondTithesAndFourthYearsFruit",
        11,
    ),
    (
        "First Fruits and other Gifts to Priests Outside the Sanctuary",
        "הלכות ביכורים ושאר מתנות כהונה שבגבולין",
        "FirstFruitsAndOtherGiftsToPriestsOutsideTheSanctuary",
        12,
    ),
    (
        "Sabbatical Year and the Jubilee",
        "הלכות שמיטה ויובל",
        "SabbaticalYearAndTheJubilee",
        13,
    ),
    (
        "The Chosen Temple",
        "הלכות בית הבחירה",
        "TheChosenTemple",
        8,
    ),
    (
        "Vessels of the Sanctuary and Those who Serve Therein",
        "הלכות כלי המקדש והעובדין בו",
        "VesselsOfTheSanctuaryAndThoseWhoServeTherein",
        10,
    ),
    (
        "Admission into the Sanctuary",
        "הלכות ביאת מקדש",
        "AdmissionIntoTheSanctuary",
        9,
    ),
    (
        "Things Forbidden on the Altar",
        "הלכות איסורי המזבח",
        "ThingsForbiddenOnTheAltar",
        7,
    ),
    (
        "Sacrificial Procedure",
        "הלכות מעשה הקרבנות",
        "SacrificialProcedure",
        19,
    ),
    (
        "Daily Offerings and Additional Offerings",
        "הלכות תמידים ומוספין",
        "DailyOfferingsAndAdditionalOfferings",
        10,
    ),
    (
        "Sacrifices Rendered Unfit",
        "הלכות פסולי המוקדשין",
        "SacrificesRenderedUnfit",
        19,
    ),
    (
        "Service on the Day of Atonement",
        "הלכות עבודת יום הכפורים",
        "ServiceOnTheDayOfAtonement",
        5,
    ),
    ("Trespass", "הלכות מעילה", "Trespass", 8),
    ("Paschal Offering", "הלכות קרבן פסח", "PaschalOffering", 10),
    ("Festival Offering", "הלכות חגיגה", "FestivalOffering", 3),
    ("Firstlings", "הלכות בכורות", "Firstlings", 8),
    (
        "Offerings for Unintentional Transgressions",
        "הלכות שגגות",
        "OfferingsForUnintentionalTransgressions",
        15,
    ),
    (
        "Offerings for Those with Incomplete Atonement",
        "הלכות מחוסרי כפרה",
        "OfferingsForThoseWithIncompleteAtonement",
        5,
    ),
    ("Substitution", "הלכות תמורה", "Substitution", 4),
    (
        "Defilement by a Corpse",
        "הלכות טומאת מת",
        "DefilementByACorpse",
        25,
    ),
    ("Red Heifer", "הלכות פרה אדומה", "RedHeifer", 15),
    (
        "Defilement by Leprosy",
        "הלכות טומאת צרעת",
        "DefilementByLeprosy",
        16,
    ),
    (
        "Those Who Defile Bed or Seat",
        "הלכות מטמאי משכב ומושב",
        "ThoseWhoDefileBedOrSeat",
        13,
    ),
    (
        "Other Sources of Defilement",
        "הלכות שאר אבות הטומאות",
        "OtherSourcesOfDefilement",
        20,
    ),
    (
        "Defilement of Foods",
        "הלכות טומאת אוכלים",
        "DefilementOfFoods",
        16,
    ),
    ("Vessels", "הלכות כלים", "Vessels", 28),
    ("Immersion Pools", "הלכות מקואות", "ImmersionPools", 11),
    (
        "Damages to Property",
        "הלכות נזקי ממון",
        "DamagesToProperty",
        14,
    ),
    ("Theft", "הלכות גניבה", "Theft", 9),
    (
        "Robbery and Lost Property",
        "הלכות גזילה ואבידה",
        "RobberyAndLostProperty",
        18,
    ),
    (
        "One Who Injures a Person or Property",
        "הלכות חובל ומזיק",
        "OneWhoInjuresAPersonOrProperty",
        8,
    ),
    (
        "Murderer and the Preservation of Life",
        "הלכות רוצח ושמירת נפש",
        "MurdererAndThePreservationOfLife",
        13,
    ),
    ("Sales", "הלכות מכירה", "Sales", 30),
    (
        "Ownerless Property and Gifts",
        "הלכות זכייה ומתנה",
        "OwnerlessPropertyAndGifts",
        12,
    ),
    ("Neighbors", "הלכות שכנים", "Neighbors", 14),
    (
        "Agents and Partners",
        "הלכות שלוחין ושותפין",
        "AgentsAndPartners",
        10,
    ),
    ("Slaves", "הלכות עבדים", "Slaves", 9),
    ("Hiring", "הלכות שכירות", "Hiring", 13),
    (
        "Borrowing and Deposit",
        "הלכות שאלה ופיקדון",
        "BorrowingAndDeposit",
        8,
    ),
    (
        "Creditor and Debtor",
        "הלכות מלווה ולווה",
        "CreditorAndDebtor",
        27,
    ),
    (
        "Plaintiff and Defendant",
        "הלכות טוען ונטען",
        "PlaintiffAndDefendant",
        16,
    ),
    ("Inheritances", "הלכות נחלות", "Inheritances", 11),
    (
        "The Sanhedrin and the Penalties within their Jurisdiction",
        "הלכות סנהדרין והעונשין המסורין להם",
        "TheSanhedrinAndThePenaltiesWithinTheirJurisdiction",
        26,
    ),
    ("Testimony", "הלכות עדות", "Testimony", 22),
    ("Rebels", "הלכות ממרים", "Rebels", 7),
    ("Mourning", "הלכות אבל", "Mourning", 14),
    ("Kings and Wars", "הלכות מלכים ומלחמות", "KingsAndWars", 12),
];

pub const YERUSHALMI: [(&str, &str, &str, u8); 39] = [
    (
        "Jerusalem Talmud Berakhot",
        "תלמוד ירושלמי ברכות",
        "JerusalemTalmudBerakhot",
        68,
    ),
    (
        "Jerusalem Talmud Peah",
        "תלמוד ירושלמי פאה",
        "JerusalemTalmudPeah",
        37,
    ),
    (
        "Jerusalem Talmud Demai",
        "תלמוד ירושלמי דמאי",
        "JerusalemTalmudDemai",
        34,
    ),
    (
        "Jerusalem Talmud Kilayim",
        "תלמוד ירושלמי כלאים",
        "JerusalemTalmudKilayim",
        44,
    ),
    (
        "Jerusalem Talmud Shevi'it",
        "תלמוד ירושלמי שביעית",
        "JerusalemTalmudSheviit",
        31,
    ),
    (
        "Jerusalem Talmud Terumot",
        "תלמוד ירושלמי תרומות",
        "JerusalemTalmudTerumot",
        59,
    ),
    (
        "Jerusalem Talmud Ma'asrot",
        "תלמוד ירושלמי מעשרות",
        "JerusalemTalmudMaasrot",
        26,
    ),
    (
        "Jerusalem Talmud Ma'aser Sheni",
        "תלמוד ירושלמי מעשר שני",
        "JerusalemTalmudMaaserSheni",
        33,
    ),
    (
        "Jerusalem Talmud Hallah",
        "תלמוד ירושלמי חלה",
        "JerusalemTalmudHallah",
        28,
    ),
    (
        "Jerusalem Talmud Orlah",
        "תלמוד ירושלמי ערלה",
        "JerusalemTalmudOrlah",
        20,
    ),
    (
        "Jerusalem Talmud Bikkurim",
        "תלמוד ירושלמי בכורים",
        "JerusalemTalmudBikkurim",
        13,
    ),
    (
        "Jerusalem Talmud Shabbat",
        "תלמוד ירושלמי שבת",
        "JerusalemTalmudShabbat",
        92,
    ),
    (
        "Jerusalem Talmud Eruvin",
        "תלמוד ירושלמי עירובין",
        "JerusalemTalmudEruvin",
        65,
    ),
    (
        "Jerusalem Talmud Pesachim",
        "תלמוד ירושלמי פסחים",
        "JerusalemTalmudPesachim",
        71,
    ),
    (
        "Jerusalem Talmud Beitzah",
        "תלמוד ירושלמי ביצה",
        "JerusalemTalmudBeitzah",
        22,
    ),
    (
        "Jerusalem Talmud Rosh Hashanah",
        "תלמוד ירושלמי ראש השנה",
        "JerusalemTalmudRoshHashanah",
        22,
    ),
    (
        "Jerusalem Talmud Yoma",
        "תלמוד ירושלמי יומא",
        "JerusalemTalmudYoma",
        42,
    ),
    (
        "Jerusalem Talmud Sukkah",
        "תלמוד ירושלמי סוכה",
        "JerusalemTalmudSukkah",
        26,
    ),
    (
        "Jerusalem Talmud Ta'anit",
        "תלמוד ירושלמי תענית",
        "JerusalemTalmudTaanit",
        26,
    ),
    (
        "Jerusalem Talmud Shekalim",
        "תלמוד ירושלמי שקלים",
        "JerusalemTalmudShekalim",
        33,
    ),
    (
        "Jerusalem Talmud Megillah",
        "תלמוד ירושלמי מגילה",
        "JerusalemTalmudMegillah",
        34,
    ),
    (
        "Jerusalem Talmud Chagigah",
        "תלמוד ירושלמי חגיגה",
        "JerusalemTalmudChagigah",
        22,
    ),
    (
        "Jerusalem Talmud Moed Kattan",
        "תלמוד ירושלמי מועד קטן",
        "JerusalemTalmudMoedKattan",
        19,
    ),
    (
        "Jerusalem Talmud Yevamot",
        "תלמוד ירושלמי יבמות",
        "JerusalemTalmudYevamot",
        85,
    ),
    (
        "Jerusalem Talmud Ketubot",
        "תלמוד ירושלמי כתובות",
        "JerusalemTalmudKetubot",
        72,
    ),
    (
        "Jerusalem Talmud Sotah",
        "תלמוד ירושלמי סוטה",
        "JerusalemTalmudSotah",
        47,
    ),
    (
        "Jerusalem Talmud Nedarim",
        "תלמוד ירושלמי נדרים",
        "JerusalemTalmudNedarim",
        40,
    ),
    (
        "Jerusalem Talmud Nazir",
        "תלמוד ירושלמי נזיר",
        "JerusalemTalmudNazir",
        47,
    ),
    (
        "Jerusalem Talmud Gittin",
        "תלמוד ירושלמי גיטין",
        "JerusalemTalmudGittin",
        54,
    ),
    (
        "Jerusalem Talmud Kiddushin",
        "תלמוד ירושלמי קידושין",
        "JerusalemTalmudKiddushin",
        48,
    ),
    (
        "Jerusalem Talmud Bava Kamma",
        "תלמוד ירושלמי בבא קמא",
        "JerusalemTalmudBavaKamma",
        44,
    ),
    (
        "Jerusalem Talmud Bava Metsia",
        "תלמוד ירושלמי בבא מציעא",
        "JerusalemTalmudBavaMetsia",
        37,
    ),
    (
        "Jerusalem Talmud Bava Batra",
        "תלמוד ירושלמי בבא בתרא",
        "JerusalemTalmudBavaBatra",
        34,
    ),
    (
        "Jerusalem Talmud Shevuot",
        "תלמוד ירושלמי שבועות",
        "JerusalemTalmudShevuot",
        44,
    ),
    (
        "Jerusalem Talmud Makkot",
        "תלמוד ירושלמי מכות",
        "JerusalemTalmudMakkot",
        9,
    ),
    (
        "Jerusalem Talmud Sanhedrin",
        "תלמוד ירושלמי סנהדרין",
        "JerusalemTalmudSanhedrin",
        57,
    ),
    (
        "Jerusalem Talmud Avodah Zarah",
        "תלמוד ירושלמי עבודה זרה",
        "JerusalemTalmudAvodahZarah",
        37,
    ),
    (
        "Jerusalem Talmud Horayot",
        "תלמוד ירושלמי הוריות",
        "JerusalemTalmudHorayot",
        19,
    ),
    (
        "Jerusalem Talmud Niddah",
        "תלמוד ירושלמי נדה",
        "JerusalemTalmudNiddah",
        13,
    ),
];

#[test]
fn ensure_right_amount_of_yerushalmi_daf() {
    assert_eq!(
        YERUSHALMI.iter().map(|x| x.3 as u64).sum::<u64>(),
        1563 /*Days Between 23 Av 5778 and 19th Cheshvan 5783*/ - 5 /* Yom Kippur*/ - 4 /* Tisha BeAv*/
    );
}

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
