use assert_cmd::prelude::CommandCargoExt;

use serde::Deserialize;

use std::process::Command;

#[test]
fn check_chabad_holidays_doesnt_crash() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5698")
        .arg("--years")
        .arg("600")
        .arg("--show=chabad-holidays");
    let _: Vec<Res> =
        serde_json::from_str(&String::from_utf8(cmd.output().unwrap().stdout).unwrap()).unwrap();
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Res {
    day: String,
    name: String,
    r#type: String,
}
