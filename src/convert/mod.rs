use crate::args::types::{AppError, ConvertArgs, ConvertType, Language, MainArgs, OutputType};
use crate::prelude::print;
use crate::Runnable;
use chrono::prelude::*;
use chrono::Duration;
use either::Either;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use std::convert::TryInto;
use std::io::{BufWriter, StdoutLock, Write};
#[derive(Debug)]
pub struct Return {
    pub day: Either<[chrono::DateTime<Utc>; 2], [HebrewDate; 2]>,
    pub orig_day: Either<HebrewDate, chrono::DateTime<Utc>>,
}

impl Return {
    fn pretty_print(&self, args: &MainArgs) -> Result<(), AppError> {
        match args.language {
            Language::English => match self.orig_day {
                Either::Right(r) => println!(
                    "{}: From {} {} {} to {} {} {}.",
                    r.format("%A %B %-d %Y"),
                    self.day.right().unwrap()[0].day(),
                    print::hebrew_month_english(self.day.right().unwrap()[0].month()),
                    self.day.right().unwrap()[0].year(),
                    self.day.right().unwrap()[1].day(),
                    print::hebrew_month_english(self.day.right().unwrap()[1].month()),
                    self.day.right().unwrap()[1].year(),
                ),
                Either::Left(l) => println!(
                    "{} {} {} -> From sunset {} to sunset {}.",
                    l.day(),
                    print::hebrew_month_english(l.month()),
                    l.year(),
                    self.day.left().unwrap()[0].format("%A %B %-d %Y"),
                    self.day.left().unwrap()[1].format("%A %B %-d %Y"),
                ),
            },
            Language::Hebrew => match self.orig_day {
                Either::Right(r) => println!(
                    "{}: {} {} {} - {} {} {}.",
                    r.format("%A %B %-d %Y"),
                    self.day.right().unwrap()[0].day(),
                    print::hebrew_month_hebrew(self.day.right().unwrap()[0].month()),
                    self.day.right().unwrap()[0].year(),
                    self.day.right().unwrap()[1].day(),
                    print::hebrew_month_hebrew(self.day.right().unwrap()[1].month()),
                    self.day.right().unwrap()[1].year(),
                ),
                Either::Left(l) => println!(
                    "{} {} {}: {} - {}.",
                    l.day(),
                    print::hebrew_month_hebrew(l.month()),
                    l.year(),
                    self.day.left().unwrap()[0].format("%A %B %-d %Y"),
                    self.day.left().unwrap()[1].format("%A %B %-d %Y"),
                ),
            },
        };
        Ok(())
    }
    fn json_print(&self, lock: &mut BufWriter<StdoutLock<'_>>) -> Result<(), AppError> {
        match &self.day {
            Either::Right(r) => {
                lock.write(b"[").unwrap();
                lock.write(r#"{"day":"#.as_bytes()).unwrap();
                write_i64(r[0].day().get().into(), lock);
                lock.write(r#","#.as_bytes()).unwrap();
                lock.write(r#""month":""#.as_bytes()).unwrap();
                lock.write(hebrew_month_to_string(r[0].month()).as_bytes())
                    .unwrap();
                lock.write(r#"","#.as_bytes()).unwrap();
                lock.write(r#""year":"#.as_bytes()).unwrap();
                write_i64(r[0].year().try_into().unwrap(), lock);
                lock.write(r#"},{"#.as_bytes()).unwrap();
                lock.write(r#""day":"#.as_bytes()).unwrap();
                write_i64(r[1].day().get().into(), lock);
                lock.write(r#","#.as_bytes()).unwrap();
                lock.write(r#""month":""#.as_bytes()).unwrap();
                lock.write(hebrew_month_to_string(r[1].month()).as_bytes())
                    .unwrap();
                lock.write(r#"","#.as_bytes()).unwrap();
                lock.write(r#""year":"#.as_bytes()).unwrap();
                write_i64(r[1].year().try_into().unwrap(), lock);
                lock.write(r#"}"#.as_bytes()).unwrap();
                lock.write(b"]").unwrap();
            }
            Either::Left(r) => {
                lock.write(b"[\"").unwrap();
                lock.write(r[0].to_rfc3339_opts(SecondsFormat::Secs, true).as_bytes())
                    .unwrap();
                lock.write(r#"",""#.as_bytes()).unwrap();
                lock.write(r[1].to_rfc3339_opts(SecondsFormat::Secs, true).as_bytes())
                    .unwrap();
                lock.write(b"\"]").unwrap();
            }
        };
        Ok(())
    }
}

fn hebrew_month_to_string(input: HebrewMonth) -> &'static str {
    use heca_lib::prelude::HebrewMonth::*;
    match input {
        Tishrei => "Tishrei",
        Cheshvan => "Cheshvan",
        Kislev => "Kislev",
        Teves => "Teves",
        Shvat => "Shvat",
        Adar => "Adar",
        Adar1 => "Adar1",
        Adar2 => "Adar2",
        Nissan => "Nissan",
        Iyar => "Iyar",
        Sivan => "Sivan",
        Tammuz => "Tammuz",
        Av => "Av",
        Elul => "Elul",
    }
}

fn write_i64(input: i64, lock: &mut BufWriter<StdoutLock<'_>>) {
    let mut arr = [b'\0'; 20];
    let count = itoa::write(&mut arr[..], input).unwrap();
    lock.write(&arr[..count]).unwrap();
}

impl Return {
    fn print(&self, args: &MainArgs, lock: &mut BufWriter<StdoutLock<'_>>) -> Result<(), AppError> {
        match args.output_type {
            OutputType::JSON => self.json_print(lock),
            OutputType::Pretty | OutputType::Regular => self.pretty_print(args),
        }
    }
}

impl Runnable for ConvertArgs {
    fn run(&self, args: &MainArgs, lock: &mut BufWriter<StdoutLock<'_>>) -> Result<(), AppError> {
        let ret = match self.date {
            ConvertType::Gregorian(date) => Return {
                orig_day: Either::Right(date.and_hms(0, 0, 1)),
                day: Either::Right([
                    date.and_hms(0, 0, 1).try_into()?,
                    date.and_hms(23, 0, 1).try_into()?,
                ]),
            },
            ConvertType::Hebrew(date) => Return {
                orig_day: Either::Left(date),
                day: Either::Left({
                    let first_day: DateTime<Utc> = date.into();
                    [first_day, first_day + Duration::days(1)]
                }),
            },
        };

        ret.print(args, lock)?;
        Ok(())
    }
}
