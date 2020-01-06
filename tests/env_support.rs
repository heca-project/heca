use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn env_heca_year_type_yomi_test() {
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5700")
        .arg("--years=2")
        .arg("--type=gregorian")
        .arg("--show=yom-tov");
    let o = cmd.output().expect(&format!("{} {}", file!(), line!()));
    let s1 = &String::from_utf8(o.stdout).expect(&format!("{} {}", file!(), line!()));

    if !o.status.success() {
        panic!(
            "{}",
            &String::from_utf8(o.stderr).expect(&format!("{} {}", file!(), line!()))
        )
    }

    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.env_clear();
    cmd.env("HECA_YEAR_TYPE", "GREGORIAN");
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5700")
        .arg("--years=2")
        .arg("--show=yom-tov");
    let o = cmd.output().expect(&format!("{} {}", file!(), line!()));
    let s2 = &String::from_utf8(o.stdout).expect(&format!("{} {}", file!(), line!()));
    if !o.status.success() {
        panic!(
            "{}",
            &String::from_utf8(o.stderr).expect(&format!("{} {}", file!(), line!()))
        )
    }

    assert_eq!(s1, s2);
}

#[test]
fn env_location_type_yomi_test() {
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5700")
        .arg("--years=2")
        .arg("--location=Israel")
        .arg("--show=yom-tov");
    let o = cmd.output().expect(&format!("{} {}", file!(), line!()));
    let s1 = &String::from_utf8(o.stdout).expect(&format!("{} {}", file!(), line!()));

    if !o.status.success() {
        panic!(
            "{}",
            &String::from_utf8(o.stderr).expect(&format!("{} {}", file!(), line!()))
        )
    }

    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect(&format!("{} {}", file!(), line!()));
    cmd.env_clear();
    cmd.env("HECA_LOCATION", "ISRAEL");
    cmd.arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("list")
        .arg("5700")
        .arg("--years=2")
        .arg("--show=yom-tov");
    let o = cmd.output().expect(&format!("{} {}", file!(), line!()));
    let s2 = &String::from_utf8(o.stdout).expect(&format!("{} {}", file!(), line!()));
    if !o.status.success() {
        panic!(
            "{}",
            &String::from_utf8(o.stderr).expect(&format!("{} {}", file!(), line!()))
        )
    }

    assert_eq!(s1, s2);
}
