use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn check_list1_broken() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("list1").arg("9999");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: The subcommand 'list1' wasn't recognized",
    ));

    Ok(())
}
#[test]
fn base_convert_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .env("JSON", "YES")
        .arg("en_US")
        .arg("convert")
        .arg("--datefmt")
        .arg("ISO")
        .arg("1990/1/1");
    cmd.assert().success().stdout(predicate::str::contains(
        r#"[{"day":4,"month":"Teves","year":5750},{"day":5,"month":"Teves","year":5750}]"#,
    ));

    Ok(())
}

#[test]
fn verify_that_json_equals_yes_works() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .env("JSON", "YES")
        .arg("en_US")
        .arg("convert")
        .arg("--datefmt")
        .arg("ISO")
        .arg("1990/1/1");
    cmd.assert().success().stdout(predicate::str::contains(
        r#"[{"day":4,"month":"Teves","year":5750},{"day":5,"month":"Teves","year":5750}]"#,
    ));

    Ok(())
}

#[test]
fn base_convert_hebrew() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .env("JSON", "YES")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("convert")
        .arg("4-teves-5750");
    cmd.assert().success().stdout(predicate::str::contains(
        r#"["1989-12-31T18:00:00Z","1990-01-01T18:00:00Z"]"#,
    ));

    Ok(())
}

#[test]
fn convert_adar2_in_regular_year_no_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .arg("en_US")
        .arg("convert")
        .arg("4-adar2-5750");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"Can't convert an Adar 1 or Adar 2 of a year which isn't a leap year"#,
    ));

    Ok(())
}

#[test]
fn convert_adar2_in_regular_year_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .env("JSON", "YES")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("convert")
        .arg("4-adar2-5750");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"{"type":"ConversionError","error":"IsNotLeapYear"}"#,
    ));

    Ok(())
}

#[test]
fn convert_regular_in_leap_year_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.env("JSON", "YES")
        .arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("convert")
        .arg("4-adar-5752");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"{"type":"ConversionError","error":"IsLeapYear"}"#,
    ));

    Ok(())
}

#[test]
fn convert_regular_in_leap_year_no_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .arg("en_US")
        .arg("convert")
        .arg("4-adar-5752");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"Can't convert an Adar of a year which is a leap year. Specify Adar1 or Adar2"#,
    ));

    Ok(())
}

#[test]
fn convert_year_too_small_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.env("JSON", "YES")
        .arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("convert")
        .arg("0/1/2");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"{"type":"ConversionError","error":"YearTooSmall"}"#,
    ));

    Ok(())
}

#[test]
fn convert_year_too_small_no_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .arg("en_US")
        .arg("convert")
        .arg("0/1/2");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"Cannot build calendar for years below 3764 (After Creation)"#,
    ));

    Ok(())
}

#[test]
fn convert_month_too_large_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.env("JSON", "YES")
        .arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("convert")
        .arg("5/13/3");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"{"type":"InvalidGregorianDay","error":""#,
    ));

    Ok(())
}

#[test]
fn convert_month_too_large_no_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .arg("en_US")
        .arg("convert")
        .arg("5/13/3");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(r#"is not a valid Gregorian date"#));

    Ok(())
}

#[test]
fn convert_day_too_large_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.env("JSON", "YES")
        .arg("--language")
        .arg("en_US")
        .arg("--print")
        .arg("json")
        .arg("convert")
        .arg("5/1/33");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"{"type":"InvalidGregorianDay","error":"5/1/33"}"#,
    ));

    Ok(())
}

#[test]
fn convert_day_too_large_no_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--language")
        .arg("en_US")
        .arg("convert")
        .arg("5/1/33");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(r#"is not a valid Gregorian date"#));

    Ok(())
}
