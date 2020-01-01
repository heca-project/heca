use assert_cmd::prelude::CommandCargoExt;
use chrono::{Duration, NaiveDate};

use serde::Deserialize;

use std::process::Command;

#[test]
fn hebcal_to_cmd_check() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5698")
        .arg("--years")
        .arg("600")
        .arg("--show=israeli-holidays");
    let res: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();

    let yom_haatzmauts = include_str!("yom_haatzmaut_5760_500").split('\n').collect();
    let yom_hazikarons = include_str!("yom_hazikaron_5760_500").split('\n').collect();

    find_holiday(yom_haatzmauts, "YomHaAtzmaut", &res);
    find_holiday(yom_hazikarons, "YomHaZikaron", &res);
}

fn find_holiday(yom_haatzmauts: Vec<&str>, json_match: &str, res: &[Res]) {
    for yom_haatzmaut in yom_haatzmauts {
        if yom_haatzmaut != "" {
            let hebcal_date = NaiveDate::parse_from_str(
                yom_haatzmaut
                    .split_at(
                        yom_haatzmaut
                            .find(' ')
                            .expect(&format!("{} {}", file!(), line!())),
                    )
                    .0,
                "%-m/%-d/%Y",
            )
            .unwrap();
            let found = res
                .iter()
                .filter(|x| x.name == json_match)
                .map(|x| {
                    eprintln!("{}", x.day);
                    NaiveDate::parse_from_str(&x.day, "%Y-%-m-%-dT18:00:00Z").expect(&format!(
                        "{} {}",
                        file!(),
                        line!()
                    )) + Duration::days(1)
                })
                .find(|x| x == &hebcal_date);
            assert_eq!(found, Some(hebcal_date));
        }
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Res {
    day: String,
    name: String,
    r#type: String,
}
