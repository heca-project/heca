use assert_cmd::prelude::CommandCargoExt;
use chrono::{Duration, SecondsFormat, TimeZone, Utc};

use serde::Deserialize;

use regex::Regex;
use std::process::Command;

#[test]
fn test_molad() {
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1990")
        .arg("--years")
        .arg("100")
        .arg("--show=shabbos-mevarchim");
    let out = cmd.output().expect(&format!("{} {}", file!(), line!()));
    if !out.status.success() {
        panic!("{}", String::from_utf8(out.stderr).unwrap());
    }
    let re = Regex::new(
        r#"(.+)/(.+)/(.+) Molad (.+): (.+), (.+) minutes and (.+) chalakim after (.+) (.+)"#,
    )
    .expect(&format!("{} {}", file!(), line!()));
    let orig: Vec<Res> = include_str!("molad_1990_100")
        .split('\n')
        .filter(|x| x != &"")
        .map(|x| {
            eprintln!("{}", x);
            let caps = re.captures(x).expect(&format!("{} {}", file!(), line!()));
            let month: u32 = caps
                .get(1)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let day: u32 = caps
                .get(2)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let year: i32 = caps
                .get(3)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let month_hebrew = caps
                .get(4)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str();
            let minute: u8 = caps
                .get(6)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let chalakim: u8 = caps
                .get(7)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let am: bool = caps
                .get(9)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                == "AM";
            let hour: u8 = caps
                .get(8)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let hour = if am { hour } else { hour + 12 };

            let day = Utc.ymd(year, month, day).and_hms(18, 0, 0) - Duration::days(1);
            Res {
                day: day.to_rfc3339_opts(SecondsFormat::Secs, true),
                month: month_hebrew.to_string(),
                molad: Molad {
                    hour,
                    minute,
                    chalakim,
                },
            }
        })
        .collect();
    let res: Vec<Res> = serde_json::from_str(&String::from_utf8(out.stdout).expect(&format!(
        "{} {}",
        file!(),
        line!()
    )))
    .expect(&format!("{} {}", file!(), line!()));
    eprintln!("res = {:?}", res);

    for r in res.iter().filter(|x| x.month != "Tishrei") {
        orig.iter()
            .find(|x| {
                x.day == r.day
                    && x.molad.hour == r.molad.hour
                    && x.molad.minute == r.molad.minute
                    && x.molad.chalakim == r.molad.chalakim
            })
            .expect(&format!("Not found {:?} {} {}", r, file!(), line!()));
    }
}

#[derive(Deserialize, Clone, Debug)]
struct Res {
    day: String,
    month: String,
    molad: Molad,
}

#[derive(Deserialize, Clone, Debug)]
struct Molad {
    hour: u8,
    minute: u8,
    chalakim: u8,
}
