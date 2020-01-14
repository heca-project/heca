use assert_cmd::prelude::*;
use chrono::prelude::*;
use chrono::Duration;
use heca_lib::prelude::HebrewMonth;
use heca_lib::HebrewDate;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::num::NonZeroI8;
use std::process::Command;

static HEBCAL_TABLE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("RoshHashanah1", "Rosh Hashana");
    m.insert("RoshHashanah2", "Rosh Hashana II");
    m.insert("YomKippur", "Yom Kippur");
    m.insert("Sukkos1", "Sukkot I");
    m.insert("Sukkos2", "Sukkot II");
    m.insert("Sukkos3", "Sukkot III");
    m.insert("Sukkos4", "Sukkot IV");
    m.insert("Sukkos5", "Sukkot V");
    m.insert("Sukkos6", "Sukkot VI");
    m.insert("Sukkos7", "Sukkot VII");
    m.insert("ShminiAtzeres", "Shmini Atzeret");
    m.insert("SimchasTorah", "Simchat Torah");
    m.insert("ShabbosHaGadol", "Shabbat HaGadol");
    m.insert("TaanisBechoros", "Ta'anit Bechorot");
    m.insert("Pesach1", "Pesach I");
    m.insert("Pesach2", "Pesach II");
    m.insert("Pesach3", "Pesach III");
    m.insert("Pesach4", "Pesach IV");
    m.insert("Pesach5", "Pesach V");
    m.insert("Pesach6", "Pesach VI");
    m.insert("Pesach7", "Pesach VII");
    m.insert("Pesach8", "Pesach VIII");
    m.insert("Shavuos1", "Shavuot I");
    m.insert("Shavuos2", "Shavuot II");
    m.insert("Shekalim", "Shabbat Shekalim");
    m.insert("Zachor", "Shabbat Zachor");
    m.insert("Parah", "Shabbat Parah");
    m.insert("HaChodesh", "Shabbat HaChodesh");
    m.insert("TzomGedalia", "Tzom Gedaliah");
    m.insert("RoshChodeshCheshvan1", "Rosh Chodesh Cheshvan");
    m.insert("RoshChodeshCheshvan2", "Rosh Chodesh Cheshvan");
    m.insert("Chanukah1", "Chanukah: 1 Candle");
    m.insert("Chanukah2", "Chanukah: 2 Candle");
    m.insert("Chanukah3", "Chanukah: 3 Candle");
    m.insert("Chanukah4", "Chanukah: 4 Candle");
    m.insert("Chanukah5", "Chanukah: 5 Candle");
    m.insert("Chanukah6", "Chanukah: 6 Candle");
    m.insert("Chanukah7", "Chanukah: 7 Candle");
    m.insert("Chanukah8", "Chanukah: 8 Candle");
    m.insert("TenTeves", "Asara B'Tevet");
    m.insert("RoshChodeshShvat", "Rosh Chodesh Sh'vat");
    m.insert("RoshChodeshNissan", "Rosh Chodesh Nisan");
    m.insert("RoshChodeshIyar1", "Rosh Chodesh Iyyar");
    m.insert("RoshChodeshIyar2", "Rosh Chodesh Iyyar");
    m.insert("RoshChodeshSivan", "Rosh Chodesh Sivan");
    m.insert("RoshChodeshTammuz1", "Rosh Chodesh Tamuz");
    m.insert("RoshChodeshTammuz2", "Rosh Chodesh Tamuz");
    m.insert("RoshChodeshAv", "Rosh Chodesh Av");
    m.insert("RoshChodeshElul1", "Rosh Chodesh Elul");
    m.insert("RoshChodeshElul2", "Rosh Chodesh Elul");
    m.insert("RoshChodeshKislev1", "Rosh Chodesh Kislev");
    m.insert("RoshChodeshKislev2", "Rosh Chodesh Kislev");
    m.insert("RoshChodeshKislev", "Rosh Chodesh Kislev");
    m.insert("RoshChodeshTeves1", "Rosh Chodesh Tevet");
    m.insert("RoshChodeshTeves2", "Rosh Chodesh Tevet");
    m.insert("RoshChodeshTeves", "Rosh Chodesh Tevet");
    m.insert("RoshChodeshAdar1", "Rosh Chodesh Adar");
    m.insert("RoshChodeshAdar2", "Rosh Chodesh Adar");
    m.insert("TaanisEsther", "Ta'anit Esther");
    m.insert("Purim", "Purim");
    m.insert("ShushanPurim", "Shushan Purim");
    m.insert("RoshChodeshAdarRishon1", "Rosh Chodesh Adar I");
    m.insert("RoshChodeshAdarRishon2", "Rosh Chodesh Adar I");
    m.insert("RoshChodeshAdarSheni1", "Rosh Chodesh Adar I");
    m.insert("RoshChodeshAdarSheni2", "Rosh Chodesh Adar I");
    m.insert("SeventeenTammuz", "Tzom Tammuz");
    m.insert("NineAv", "Tish'a B'Av");
    m.insert("Vayelech", "Parashat Vayeilech");
    m.insert("Haazinu", "Parashat Ha'Azinu");
    m.insert("Bereishis", "Parashat Bereshit");
    m.insert("Noach", "Parashat Noach");
    m.insert("LechLecha", "Parashat Lech-Lecha");
    m.insert("Vayeira", "Parashat Vayera");
    m.insert("ChayeiSara", "Parashat Chayei Sara");
    m.insert("Toldos", "Parashat Toldot");
    m.insert("Vayetzei", "Parashat Vayetzei");
    m.insert("Vayishlach", "Parashat Vayishlach");
    m.insert("Vayeshev", "Parashat Vayeshev");
    m.insert("Miketz", "Parashat Miketz");
    m.insert("Vayigash", "Parashat Vayigash");
    m.insert("Vayechi", "Parashat Vayechi");
    m.insert("Shemos", "Parashat Shemot");
    m.insert("Vaeira", "Parashat Vaera");
    m.insert("Bo", "Parashat Bo");
    m.insert("Beshalach", "Parashat Beshalach");
    m.insert("Yisro", "Parashat Yitro");
    m.insert("Mishpatim", "Parashat Mishpatim");
    m.insert("Terumah", "Parashat Terumah");
    m.insert("Tetzaveh", "Parashat Tetzaveh");
    m.insert("KiSisa", "Parashat Ki Tisa");
    m.insert("VayakhelPikudei", "Parashat Vayakhel-Pekudei");
    m.insert("Vayakhel", "Parashat Vayakhel");
    m.insert("Pikudei", "Parashat Pekudei");
    m.insert("Vayikra", "Parashat Vayikra");
    m.insert("Tzav", "Parashat Tzav");
    m.insert("Shemini", "Parashat Shmini");
    m.insert("TazriyaMetzorah", "Parashat Tazria-Metzora");
    m.insert("Tazriya", "Parashat Tazria");
    m.insert("Metzorah", "Parashat Metzora");
    m.insert("AchareiMosKedoshim", "Parashat Achrei Mot-Kedoshim");
    m.insert("AchareiMos", "Parashat Achrei Mot");
    m.insert("Kedoshim", "Parashat Kedoshim");
    m.insert("Emor", "Parashat Emor");
    m.insert("BeharBechukosai", "Parashat Behar-Bechukotai");
    m.insert("Behar", "Parashat Behar");
    m.insert("Bechukosai", "Parashat Bechukotai");
    m.insert("Bamidbar", "Parashat Bamidbar");
    m.insert("Naso", "Parashat Nasso");
    m.insert("Behaaloscha", "Parashat Beha'alotcha");
    m.insert("Shlach", "Parashat Sh'lach");
    m.insert("Korach", "Parashat Korach");
    m.insert("ChukasBalak", "Parashat Chukat-Balak");
    m.insert("Chukas", "Parashat Chukat");
    m.insert("Balak", "Parashat Balak");
    m.insert("Pinchas", "Parashat Pinchas");
    m.insert("MatosMaasei", "Parashat Matot-Masei");
    m.insert("Matos", "Parashat Matot");
    m.insert("Maasei", "Parashat Masei");
    m.insert("Devarim", "Parashat Devarim");
    m.insert("Vaeschanan", "Parashat Vaetchanan");
    m.insert("Eikev", "Parashat Eikev");
    m.insert("Reeh", "Parashat Re'eh");
    m.insert("Shoftim", "Parashat Shoftim");
    m.insert("KiSeitzei", "Parashat Ki Teitzei");
    m.insert("KiSavoh", "Parashat Ki Tavo");
    m.insert("NitzavimVayelech", "Parashat Nitzavim-Vayeilech");
    m.insert("Nitzavim", "Parashat Nitzavim");
    m.insert("ErevRoshHashanah", "Erev Rosh Hashana");
    m.insert("ErevYomKippur", "Erev Yom Kippur");
    m.insert("ErevSukkos", "Erev Sukkot");
    m.insert("ErevPesach", "Erev Pesach");
    m.insert("ErevShavuos", "Erev Shavuot");
    m.insert("FifteenShvat", "Tu B'Shvat");
    m.insert("FifteenAv", "");
    m.insert("LagBaOmer", "Lag B'Omer");
    m.insert("Omer1", "");
    m.insert("Omer2", "");
    m.insert("Omer3", "");
    m.insert("Omer4", "");
    m.insert("Omer5", "");
    m.insert("Omer6", "");
    m.insert("Omer7", "");
    m.insert("Omer8", "");
    m.insert("Omer9", "");
    m.insert("Omer10", "");
    m.insert("Omer11", "");
    m.insert("Omer12", "");
    m.insert("Omer13", "");
    m.insert("Omer14", "");
    m.insert("Omer15", "");
    m.insert("Omer16", "");
    m.insert("Omer17", "");
    m.insert("Omer18", "");
    m.insert("Omer19", "");
    m.insert("Omer20", "");
    m.insert("Omer21", "");
    m.insert("Omer22", "");
    m.insert("Omer23", "");
    m.insert("Omer24", "");
    m.insert("Omer25", "");
    m.insert("Omer26", "");
    m.insert("Omer27", "");
    m.insert("Omer28", "");
    m.insert("Omer29", "");
    m.insert("Omer30", "");
    m.insert("Omer31", "");
    m.insert("Omer32", "");
    m.insert("Omer33", "Lag B'Omer");
    m.insert("Omer34", "");
    m.insert("Omer35", "");
    m.insert("Omer36", "");
    m.insert("Omer37", "");
    m.insert("Omer38", "");
    m.insert("Omer39", "");
    m.insert("Omer40", "");
    m.insert("Omer41", "");
    m.insert("Omer42", "");
    m.insert("Omer43", "");
    m.insert("Omer44", "");
    m.insert("Omer45", "");
    m.insert("Omer46", "");
    m.insert("Omer47", "");
    m.insert("Omer48", "");
    m.insert("Omer49", "");
    m.insert("PesachSheni", "");
    m.insert("ShabbosNachamu", "Shabbat Nachamu");
    m.insert("ShabbosChazon", "Shabbat Hazon");
    m.insert("ShabbosShuva", "Shabbat Shuva");
    m.insert("LeilSlichos", "");
    m
});

#[test]
fn leil_slichos_always_starts_on_shabbos() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1990")
        .arg("--years")
        .arg("5000")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    for i in res {
        if i.name == "Slichos" {
            eprintln!("{}", i.day);
            assert_eq!(
                DateTime::parse_from_rfc3339(&i.day).unwrap().weekday(),
                Weekday::Sat
            );
        }

        if i.name == "LeilSlichos" {
            eprintln!("{}", i.day);
            assert_eq!(
                DateTime::parse_from_rfc3339(&i.day).unwrap().weekday(),
                Weekday::Sat
            );
        }
    }
}

#[test]
fn shabbos_shuva_always_starts_on_shabbos() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1990")
        .arg("--years")
        .arg("5000")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    for i in res {
        if i.name == "ShabbosShuva" {
            eprintln!("{}", i.day);
            assert_eq!(
                DateTime::parse_from_rfc3339(&i.day).unwrap().weekday(),
                Weekday::Fri
            );
        }
    }
}

#[test]
fn erev_rosh_hashana_check_gregorian() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1990")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    for i in res {
        if i.name == "ErevRoshHashanah" {
            if i.day != "1990-09-18T18:00:00Z" {
                panic!("Erev Rosh Hashana is on the wrong day.")
            }
        }
    }
}

#[test]
fn erev_rosh_hashana_check_hebrew() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    for i in res {
        if i.name == "ErevRoshHashanah" {
            if i.day != "1990-09-18T18:00:00Z" {
                panic!("Erev Rosh Hashana is on the wrong day.")
            }
        }
    }
}

#[test]
fn check_hebrew_command_line() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("he_IL")
        .arg("list")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,israeli-holidays,chabad-holidays,daf-yomi,yerushalmi-yomi,rambam-3-chapters,rambam-1-chapter,shabbos-mevarchim");
    let out = cmd.output().unwrap();
    if !out.status.success() {
        panic!("{}", &String::from_utf8(out.stderr).unwrap());
    }
    let res = &String::from_utf8(out.stdout).unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.env("LANG","he_IL").
        arg("list")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,israeli-holidays,chabad-holidays,daf-yomi,yerushalmi-yomi,rambam-3-chapters,rambam-1-chapter,shabbos-mevarchim");
    let out = cmd.output().unwrap();
    if !out.status.success() {
        panic!("{}", &String::from_utf8(out.stderr).unwrap());
    }
    let res1 = &String::from_utf8(out.stdout).unwrap();
    assert_eq!(res, res1);
}

static HEBCAL: Lazy<HashMap<chrono::NaiveDate, Vec<String>>> = Lazy::new(|| {
    let mut hebcal = HashMap::new();
    let holidays = include_str!("holidays_1980_9999");

    holidays.lines().for_each(|line| {
        let (str_date, name) = line.split_at(line.chars().position(|c| c == ' ').unwrap());
        let date = NaiveDate::parse_from_str(str_date, "%-m/%-d/%Y").unwrap();
        if !hebcal.contains_key(&date) {
            hebcal.insert(date, vec![String::from(name)]);
        } else {
            hebcal.get_mut(&date).unwrap().push(String::from(name));
        }
    });

    hebcal
});

#[test]
fn hebcal_to_cmd_check() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1990")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    HEBCAL_TABLE.keys().for_each(|k| {
        assert_ne!(
            res.iter().find(|x| {
                if &x.name == k
                    || x.name.contains("Tazriya")
                    || x.name.contains("Metzorah")
                    || x.name.contains("Acharei Mos")
                    || x.name.contains("Kedoshim")
                    || x.name.contains("Behar")
                    || x.name.contains("Bechukosai")
                    || x.name.contains("Vayakhel")
                    || x.name.contains("Pikudei")
                {
                    true
                } else {
                    eprintln!("{}", k);
                    false
                }
            }),
            None
        );
    });
}
#[test]
fn cmd_to_hebcal_check() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1990")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();

    let parsha = include_str!("parsha_50_500");
    let mut hebcal = HEBCAL.clone();

    parsha.lines().for_each(|line| {
        let (str_date, name) = line.split_at(line.chars().position(|c| c == ' ').unwrap());
        let date = NaiveDate::parse_from_str(str_date, "%-m/%-d/%Y").unwrap();
        if !hebcal.contains_key(&date) {
            hebcal.insert(date, vec![String::from(name)]);
        } else {
            hebcal.get_mut(&date).unwrap().push(String::from(name));
        }
    });

    for r in res {
        let date_orig: chrono::NaiveDate = NaiveDate::parse_from_str(
            r.day
                .split_at(r.day.chars().position(|c| c == 'T').unwrap())
                .0,
            "%Y-%m-%d",
        )
        .unwrap();
        let name_in_hebcal = HEBCAL_TABLE
            .get(r.name.as_str())
            .expect(&format!("{} not found", r.name));
        let date_in_heca: NaiveDate = if !r.name.contains("Chanuka") {
            (date_orig + Duration::days(1)).into()
        } else {
            date_orig.into()
        };

        if *name_in_hebcal != "" {
            if hebcal.contains_key(&date_in_heca) {
                let mut found = false;
                for e in &hebcal[&date_in_heca] {
                    if e.contains(name_in_hebcal) {
                        found = true;
                    }
                }

                if !found {
                    let res = format!(
                        "Day error -> {:?} {} {} {}",
                        hebcal[&date_in_heca], name_in_hebcal, date_in_heca, date_orig
                    );
                    panic!(res);
                }
            } else {
                panic!(format!("{} not found -> {}", date_in_heca, r.name));
            }
        }
    }
}

#[test]
fn hebcal_il_check() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5750")
        .arg("--location")
        .arg("Israel")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();

    let parsha = include_str!("parsha_il_50_500");
    let mut hebcal = HEBCAL.clone();

    parsha.lines().for_each(|line| {
        let (str_date, name) = line.split_at(line.chars().position(|c| c == ' ').unwrap());
        let date = NaiveDate::parse_from_str(str_date, "%-m/%-d/%Y").unwrap();
        if !hebcal.contains_key(&date) {
            hebcal.insert(date, vec![String::from(name)]);
        } else {
            hebcal.get_mut(&date).unwrap().push(String::from(name));
        }
    });

    for r in res {
        let date_orig: chrono::NaiveDate = NaiveDate::parse_from_str(
            r.day
                .split_at(r.day.chars().position(|c| c == 'T').unwrap())
                .0,
            "%Y-%m-%d",
        )
        .unwrap();
        let name_in_hebcal = HEBCAL_TABLE
            .get(r.name.as_str())
            .expect(&format!("{} not found", r.name));
        let date_in_heca: NaiveDate = if !r.name.contains("Chanuka") {
            (date_orig + Duration::days(1)).into()
        } else {
            date_orig.into()
        };
        if *name_in_hebcal != "" {
            if hebcal.contains_key(&date_in_heca) {
                let mut found = false;
                for e in &hebcal[&date_in_heca] {
                    if e.contains(name_in_hebcal) {
                        found = true;
                    }
                }
                if !found {
                    let res = format!(
                        "Day error -> {:?} {} {} {}",
                        hebcal[&date_in_heca], name_in_hebcal, date_in_heca, date_orig
                    );
                    panic!(res);
                }
            } else {
                panic!(format!("{} not found -> {}", date_in_heca, r.name));
            }
        }
    }
}

#[test]
fn custom_day_check_file_does_not_exist() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("/sample_config_does_not_exist.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    let out = cmd.output().unwrap();
    assert_eq!(out.status.code().unwrap(), 1);
    let res: Err = serde_json::from_str(&String::from_utf8(out.stderr).unwrap()).unwrap();

    assert_eq!(res.r#type, "ReadError");
}

#[test]
fn custom_day_check_gregorian() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/sample_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1990")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();

    assert_eq!(
        res.into_iter().find(|x| x.name == "YudShvat"),
        Some(Res {
            day: "1990-02-04T18:00:00Z".into(),
            name: "YudShvat".into(),
            r#type: "CustomHoliday".into(),
        })
    );
}

#[test]
fn custom_day_check_hebrew() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/sample_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();

    assert_eq!(
        res.into_iter().find(|x| x.name == "YudShvat"),
        Some(Res {
            day: "1990-02-04T18:00:00Z".into(),
            name: "YudShvat".into(),
            r#type: "CustomHoliday".into(),
        })
    );
}

#[test]
fn check_month_not_parsed_hebrew() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/bad_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5749")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    let err: Err =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stderr).unwrap()).unwrap();

    assert_eq!(
        err,
        Err {
            r#type: "MonthNotParsed".into(),
            error: "AdarII".into(),
        }
    );
}

#[test]
fn check_month_not_parsed_gregorian_1() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/bad_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1988")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    let err: Err =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stderr).unwrap()).unwrap();

    assert_eq!(
        err,
        Err {
            r#type: "MonthNotParsed".into(),
            error: "AdarII".into(),
        }
    );
}

#[test]
fn check_month_not_parsed_gregorian_2() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/bad_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1989")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    let err: Err =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stderr).unwrap()).unwrap();

    assert_eq!(
        err,
        Err {
            r#type: "MonthNotParsed".into(),
            error: "AdarII".into(),
        }
    );
}

#[test]
fn ensure_days_not_found_in_every_year_gregorian() {
    for x in 5..1000 {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("--language")
            .arg("en_US")
            .arg("--config")
            .arg("./tests/sample_config.toml")
            .arg("--print")
            .arg("json")
            .arg("list")
            .arg(x.to_string())
            .arg("--type=gregorian")
            .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
        let out = cmd.output().unwrap();
        if !out.status.success() {
            eprintln!("{}", &String::from_utf8(out.stderr).unwrap());
        }
        let ret = String::from_utf8(out.stdout).unwrap();

        let res: Vec<Res> = serde_json::from_str(&ret).expect(&ret);

        res.iter()
            .filter(|x| x.name == "AnnoyingDay")
            .for_each(|x| {
                let e_day = DateTime::parse_from_rfc3339(&x.day).unwrap();
                let e_day = e_day.with_timezone(&Utc);
                let h_day = HebrewDate::try_from(e_day).unwrap();
                if !(h_day.month() == HebrewMonth::Kislev || h_day.month() == HebrewMonth::Teves) {
                    panic!("Wrong time of AnnoyingDay");
                }
            });
        res.iter().filter(|x| x.name == "YudShvat").for_each(|x| {
            let e_day = DateTime::parse_from_rfc3339(&x.day).unwrap();
            let e_day = e_day.with_timezone(&Utc);
            let h_day = HebrewDate::try_from(e_day).unwrap();
            assert_eq!(
                h_day,
                HebrewDate::from_ymd(
                    h_day.year(),
                    HebrewMonth::Shvat,
                    NonZeroI8::new(10).unwrap()
                )
                .unwrap()
            );
        });
        res.iter()
            .filter(|x| x.name == "YahrtzeitRebMoshe")
            .for_each(|x| {
                let e_day = DateTime::parse_from_rfc3339(&x.day).unwrap();
                let e_day = e_day.with_timezone(&Utc);
                let h_day = HebrewDate::try_from(e_day).unwrap();
                if !(h_day.month() == HebrewMonth::Adar || h_day.month() == HebrewMonth::Adar2) {
                    panic!("Wrong time of YahrtzeitRebMoshe");
                }
            });

        assert_eq!(res.iter().filter(|x| x.name == "HuhDay").count(), 0);
    }
}

#[test]
fn ensure_days_not_found_in_every_year_hebrew() {
    for x in 5750..10000 {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("--language")
            .arg("en_US")
            .arg("--config")
            .arg("./tests/sample_config.toml")
            .arg("--print")
            .arg("json")
            .arg("list")
            .arg(x.to_string())
            .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
        let ret = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
        let res: Vec<Res> = serde_json::from_str(&ret).unwrap();
        eprintln!("{}", x);
        assert_ne!(res.iter().filter(|x| x.name == "AnnoyingDay").count(), 0);
        assert_eq!(res.iter().filter(|x| x.name == "YudShvat").count(), 1);
        assert_eq!(
            res.iter().filter(|x| x.name == "YahrtzeitRebMoshe").count(),
            1
        );
        assert_eq!(res.iter().filter(|x| x.name == "HuhDay").count(), 0);
    }
}

#[test]
fn custom_day_check_and_avoid_crash() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/sample_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("--years")
        .arg("500")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    assert_eq!(
        &String::from_utf8(cmd.output().unwrap().stderr).unwrap(),
        ""
    );
}

#[test]
fn custom_day_check_and_avoid_crash_v1() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/edge_cases_config_v1.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("--years")
        .arg("500")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    assert_eq!(
        &String::from_utf8(cmd.output().unwrap().stderr).unwrap(),
        ""
    );
}

#[test]
fn custom_day_check_of_edge_cases_and_avoid_crash() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/edge_cases_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("--years")
        .arg("500")
        .arg("5750")
        .arg("--show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer,custom-holidays");
    assert_eq!(
        &String::from_utf8(cmd.output().unwrap().stderr).unwrap(),
        ""
    );
}

#[test]
fn check_shabbos_hagadol_always_is_shabbos() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/edge_cases_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5778")
        .arg("--years")
        .arg("7000")
        .arg("--show=minor-holidays");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    for i in res {
        if i.name == "ShabbosHaGadol" {
            let day = DateTime::parse_from_rfc3339(&i.day).expect(&i.day);
            assert_eq!(day.weekday(), Weekday::Fri);
        }
    }
}

#[test]
fn check_taanis_bechoros_is_either_erev_pesach_or_thursday() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/edge_cases_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5778")
        .arg("--years")
        .arg("7000")
        .arg("--show=minor-holidays");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    for i in res {
        if i.name == "TaanisBechoros" {
            let day = DateTime::parse_from_rfc3339(&i.day).expect(&i.day);
            let h_day: HebrewDate = day.with_timezone(&Utc).try_into().unwrap();
            let h_day: DateTime<Utc> =
                HebrewDate::from_ymd(h_day.year(), h_day.month(), NonZeroI8::new(15).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap();
            if h_day.weekday() == Weekday::Sat {
                assert_eq!(day.weekday(), Weekday::Wed);
            }
            assert_ne!(day.weekday(), Weekday::Fri);
        }
    }
}

#[test]
fn check_double_days_hebrew() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/edge_cases_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5778")
        .arg("--show=custom-holidays");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    let thirty_cheshvan: Vec<&Res> = res.iter().filter(|x| x.name == "30Cheshvan").collect();
    assert_eq!(thirty_cheshvan.iter().count(), 2);
    assert_eq!(thirty_cheshvan[0].day, "2017-11-17T18:00:00Z");
    assert_eq!(thirty_cheshvan[1].day, "2017-11-18T18:00:00Z");
}

#[test]
fn check_double_days_gregorian() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--config")
        .arg("./tests/edge_cases_config.toml")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("2017")
        .arg("--show=custom-holidays");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
    let thirty_cheshvan: Vec<&Res> = res.iter().filter(|x| x.name == "30Cheshvan").collect();
    assert_eq!(thirty_cheshvan.iter().count(), 2);
    assert_eq!(thirty_cheshvan[0].day, "2017-11-17T18:00:00Z");
    assert_eq!(thirty_cheshvan[1].day, "2017-11-18T18:00:00Z");
}

#[test]
fn just_shabbos_works_hebrew() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("list").arg("5750").arg("--show=shabbos");
    assert_eq!(
        &String::from_utf8(cmd.output().unwrap().stderr).unwrap(),
        ""
    );
}

#[test]
fn just_shabbos_works_gregorian() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("list").arg("1990").arg("--show=shabbos");
    assert_eq!(
        &String::from_utf8(cmd.output().unwrap().stderr).unwrap(),
        ""
    );
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Res {
    day: String,
    name: String,
    r#type: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
struct Err {
    error: String,
    r#type: String,
}
