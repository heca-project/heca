use assert_cmd::prelude::CommandCargoExt;
use chrono::{DateTime, Datelike, Duration, NaiveDate};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

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
            hebcal_hashset.insert(date, format!("{}{}", h.0, h.1));
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
        .arg("8000")
        .arg("--show=daf-yomi");
    let s = &String::from_utf8(
        cmd.output()
            .expect(&format!("{} {}", file!(), line!()))
            .stdout,
    )
    .expect(&format!("{} {}", file!(), line!()));
    let res: Vec<Res> = serde_json::from_str(s).expect(&format!("{} {}", file!(), line!()));

    res.into_iter().for_each(|x| {
        let date = DateTime::parse_from_rfc3339(&x.day).expect(&format!("{} {}", file!(), line!()));
        let date = NaiveDate::from_ymd(date.year(), date.month(), date.day());
        let daf = x.name;
        heca_hashset.insert(date, daf);
    });

    for (date, daf) in hebcal_hashset {
        let heca_daf = heca_hashset.get(&date);
        if let Some(heca_daf) = heca_daf {
            if &daf != heca_daf {
                panic!(
                    "hebcal daf '{}' != heca_daf '{}' at date {}",
                    daf, heca_daf, date
                );
            }
        } else {
            panic!("heca_hashset not found day {} daf {}", date, daf);
        }
    }
}

fn hebcal_to_heca(s: &str) -> Option<(&'static str, u8)> {
    let (masechta, daf) =
        s.trim()
            .split_at(s.rfind(" ").expect(&format!("{} {}", file!(), line!())));
    let masechta = masechta.trim();
    let daf = daf.trim();
    match masechta {
        "Berachot" => Some((
            "ברכות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Shabbat" => Some((
            "שבת",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Eruvin" => Some((
            "עירובין",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Pesachim" => Some((
            "פסחים",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Shekalim" => Some((
            "שקלים",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Yoma" => Some((
            "יומא",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Rosh Hashana" => Some((
            "ראשהשנה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Sukkah" => Some((
            "סוכה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Beitzah" => Some((
            "ביצה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Taanit" => Some((
            "תענית",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Megillah" => Some((
            "מגילה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Moed Katan" => Some((
            "מועדקטן",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Chagigah" => Some((
            "חגיגה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Yevamot" => Some((
            "יבמות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Ketubot" => Some((
            "כתובות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Nedarim" => Some((
            "נדרים",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Nazir" => Some((
            "נזיר",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Sotah" => Some((
            "סוטה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Gitin" => Some((
            "גיטין",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Kiddushin" => Some((
            "קידושין",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Baba Kamma" => Some((
            "בבאקמא",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Baba Metzia" => Some((
            "בבאמציעא",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Baba Batra" => Some((
            "בבאבתרא",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Sanhedrin" => Some((
            "סנהדרין",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Makkot" => Some((
            "מכות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Shevuot" => Some((
            "שבועות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Avodah Zarah" => Some((
            "עבודהזרה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Horayot" => Some((
            "הוריות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Zevachim" => Some((
            "זבחים",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Menachot" => Some((
            "מנחות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Chullin" => Some((
            "חולין",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Bechorot" => Some((
            "בכורות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Arachin" => Some((
            "ערכין",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Temurah" => Some((
            "תמורה",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Keritot" => Some((
            "כריתות",
            daf.parse().expect(&format!("{} {}", file!(), line!())),
        )),
        "Niddah" => Some((
            "נדה",
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
pub struct Res {
    day: String,
    name: String,
    #[serde(rename = "type")]
    res_type: String,
}
