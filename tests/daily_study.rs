use assert_cmd::prelude::CommandCargoExt;
use chrono::{Duration, NaiveDate};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[test]
fn yerushalmi_yomi_test() {
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("2022")
        .arg("--show=yerushalmi-yomi");
    let s = &String::from_utf8(
        cmd.output()
            .expect(&format!("{} {}", file!(), line!()))
            .stdout,
    )
    .expect(&format!("{} {}", file!(), line!()));
    let res: Vec<YerushalmiYomiRes> =
        serde_json::from_str(s).expect(&format!("{} {}", file!(), line!()));
    assert_eq!(
        res.iter()
            .find(|x| x.day == "2022-11-13T18:00:00Z")
            .unwrap()
            .topic
            .masechta,
        "JerusalemTalmudBerakhot"
    );
    assert_eq!(
        res.iter()
            .find(|x| x.day == "2022-11-13T18:00:00Z")
            .unwrap()
            .topic
            .daf,
        1
    );
}

#[test]
fn daf_yomi_test() {
    let hebcal_daf_yomi = include_str!("daf_yomi.txt");
    let mut hebcal_hashset = HashMap::new();
    let mut heca_hashset = HashMap::new();
    hebcal_daf_yomi.split_terminator("\n").for_each(|x| {
        let (date, daf) = x.split_at(x.find(" Daf Yomi:").expect(&format!(
            "{} {}",
            file!(),
            line!()
        )));
        let date = date.to_string().trim().to_string();
        let date = NaiveDate::parse_from_str(&date, "%-m/%-d/%Y").expect(&format!("'{}'", date))
            - Duration::days(1);
        let daf: String = daf[daf.find(':').expect(&format!("{} {}", file!(), line!())) + 1..]
            .to_string()
            .trim()
            .to_string();
        if let Some(h) = hebcal_to_heca(&daf) {
            hebcal_hashset.insert(
                date,
                DafYomiTopic {
                    daf: h.1,
                    masechta: h.0.to_string(),
                },
            );
        }
    });

    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1975")
        .arg("--years")
        .arg("1000")
        .arg("--show=daf-yomi");
    let s = &String::from_utf8(
        cmd.output()
            .expect(&format!("{} {}", file!(), line!()))
            .stdout,
    )
    .expect(&format!("{} {}", file!(), line!()));
    let res: Vec<DafYomiRes> = serde_json::from_str(s).expect(&format!("{} {}", file!(), line!()));

    res.into_iter().for_each(|x| {
        let date = NaiveDate::parse_from_str(&x.day, "%Y-%-m-%-dT18:00:00Z").expect(&format!(
            "{} {}",
            file!(),
            line!()
        ));
        heca_hashset.insert(date, x.topic);
    });

    for (date, daf) in hebcal_hashset {
        let heca_daf = heca_hashset.get(&date);
        if let Some(heca_daf) = heca_daf {
            if &daf != heca_daf {
                panic!(
                    "hebcal daf '{:?}' != heca_daf '{:?}' at date {}",
                    daf, heca_daf, date
                );
            }
        } else {
            panic!(
                "heca_hashset not found day {} daf {:?} {:?}",
                date, daf, heca_daf
            );
        }
    }
}

fn hebcal_to_heca(s: &str) -> Option<(&'static str, u8)> {
    let (masechta, daf) =
        s.trim().split_at(
            s.rfind(" ")
                .expect(&format!("{} {} {}", s, file!(), line!())),
        );
    let masechta = masechta.trim();
    let daf = daf.trim();
    match masechta {
        "Berachot" => Some((
            "Berakhot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Shabbat" => Some((
            "Shabbat",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Eruvin" => Some((
            "Eruvin",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Pesachim" => Some((
            "Pesachim",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Shekalim" => Some((
            "Shekalim",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Yoma" => Some((
            "Yoma",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Rosh Hashana" => Some((
            "RoshHashanah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Sukkah" => Some((
            "Sukkah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Beitzah" => Some((
            "Beitzah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Taanit" => Some((
            "Taanit",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Megillah" => Some((
            "Megillah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Moed Katan" => Some((
            "MoedKatan",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Chagigah" => Some((
            "Chagigah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Yevamot" => Some((
            "Yevamot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Ketubot" => Some((
            "Ketubot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Nedarim" => Some((
            "Nedarim",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Nazir" => Some((
            "Nazir",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Sotah" => Some((
            "Sotah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Gitin" => Some((
            "Gittin",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Kiddushin" => Some((
            "Kiddushin",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Baba Kamma" => Some((
            "BavaKamma",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Baba Metzia" => Some((
            "BavaMetzia",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Baba Batra" => Some((
            "BavaBatra",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Sanhedrin" => Some((
            "Sanhedrin",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Makkot" => Some((
            "Makkot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Shevuot" => Some((
            "Shevuot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Avodah Zarah" => Some((
            "AvodahZarah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Horayot" => Some((
            "Horayot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Zevachim" => Some((
            "Zevachim",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Menachot" => Some((
            "Menachot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Chullin" => Some((
            "Chullin",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Bechorot" => Some((
            "Bekhorot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Arachin" => Some((
            "Arakhin",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Temurah" => Some((
            "Temurah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Keritot" => Some((
            "Keritot",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Niddah" => Some((
            "Niddah",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Meilah" => None,
        "Kinnim" => None,
        "Tamid" => None,
        "Midot" => None,
        _ => panic!("Unknown {} {} '{}'", masechta, daf, s),
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct DafYomiRes {
    day: String,
    topic: DafYomiTopic,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct DafYomiTopic {
    daf: u8,
    masechta: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct YerushalmiYomiRes {
    day: String,
    topic: YerushalmiYomiTopic,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct YerushalmiYomiTopic {
    daf: u8,
    masechta: String,
}

#[test]
fn rambam_test() {
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1975")
        .arg("--years")
        .arg("1000")
        .arg("--show=rambam-1-chapter");
    let s = &String::from_utf8(
        cmd.output()
            .expect(&format!("{} {}", file!(), line!()))
            .stdout,
    )
    .expect(&format!("{} {}", file!(), line!()));
    let res: Vec<RambamRes> = serde_json::from_str(s).expect(&format!("{} {}", file!(), line!()));
    for i in &res {
        assert!(i.topic.chapter > 0);
    }
    for i in &res {
        if i.day == "2020-07-8T16:00:00Z" {
            assert_eq!(i.topic.chapter, 12);
            assert_eq!(i.topic.halacha, "KingsAndWars");
        }
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct RambamRes {
    day: String,
    topic: RambamTopic,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct RambamTopic {
    chapter: u8,
    halacha: String,
}
