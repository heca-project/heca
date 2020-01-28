use assert_cmd::prelude::CommandCargoExt;
use chrono::{DateTime, Datelike, Duration, NaiveDate};
use serde::Deserialize;
use std::convert::TryInto;

use regex::Regex;
use std::process::Command;

#[test]
fn test_candle_lighting_new_york() {
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("1945")
        .arg("--years")
        .arg("10")
        .arg("--show=shabbos,yom-tov")
        .arg("--city")
        .arg("NewYorkCity");
    let out = cmd.output().expect(&format!("{} {}", file!(), line!()));
    if !out.status.success() {
        panic!("{}", String::from_utf8(out.stderr).unwrap());
    }
    let re = Regex::new("(.+)/(.+)/(.+) Candle lighting:  (.+):(.+)").expect(&format!(
        "{} {}",
        file!(),
        line!()
    ));
    let orig: Vec<ResSemantic> = include_str!("new_york_city_1945_100")
        .lines()
        .filter(|x| x != &"")
        .map(|x| {
            let caps = re.captures(x).expect(&format!("{} {}", file!(), line!()));
            let month: u8 = caps
                .get(1)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let day: u8 = caps
                .get(2)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let year: u64 = caps
                .get(3)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));

            let hour: u8 = caps
                .get(4)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));
            let minute: u8 = caps
                .get(5)
                .expect(&format!("{} {}", file!(), line!()))
                .as_str()
                .parse()
                .expect(&format!("{} {}", file!(), line!()));

            ResSemantic {
                month,
                day,
                year,
                candle_lighting_hour: hour + 12,
                candle_lighting_minute: minute,
            }
        })
        .collect();
    let out = String::from_utf8(out.stdout).expect(&format!("{} {}", file!(), line!()));
    let res: Vec<ResHeca> = serde_json::from_str(&out).expect(&format!("{} {}", file!(), line!()));
    let mut count = 0;
    let i: Vec<ResHeca> = res
        .into_iter()
        .filter(|x| x.candle_lighting.is_some())
        .filter(|x| x.candle_lighting.as_ref().unwrap() != "undefined")
        .collect();
    let len = i.len();
    for r in i {
        eprintln!("count={}, {}", count, len);
        count += 1;
        orig.iter()
            .find(|x| {
                let heca_candle_lighting_day = DateTime::parse_from_rfc3339(&r.day)
                    .expect(&format!("{} {}", file!(), line!()));
                dbg!(&r.candle_lighting);
                let heca_candle_lighting_time =
                    DateTime::parse_from_rfc3339(r.candle_lighting.as_ref().unwrap())
                        .expect(&format!("{} {}", file!(), line!()))
                        .naive_local();
                let res_semantic =
                    NaiveDate::from_ymd(x.year.try_into().unwrap(), x.month.into(), x.day as u32)
                        .and_hms(
                            x.candle_lighting_hour.into(),
                            x.candle_lighting_minute.into(),
                            0,
                        );
                (x.year as i32) == heca_candle_lighting_day.year()
                    && (x.day as u32) == heca_candle_lighting_day.day()
                    && (x.month as u32) == heca_candle_lighting_day.month()
                    && heca_candle_lighting_time > res_semantic - Duration::minutes(5)
                    && heca_candle_lighting_time < res_semantic + Duration::minutes(5)
            })
            .expect(&format!("Not found {:?} {} {}", r, file!(), line!()));
    }
}

#[derive(Deserialize, Clone, Debug)]
struct ResHeca {
    day: String,
    #[serde(rename = "candleLighting")]
    candle_lighting: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct ResSemantic {
    day: u8,
    month: u8,
    year: u64,
    candle_lighting_hour: u8,
    candle_lighting_minute: u8,
}
