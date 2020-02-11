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

make_list!(GEMARAS_FIRST_CYCLE, 13);
make_list!(GEMARAS_SECOND_CYCLE, 22);
