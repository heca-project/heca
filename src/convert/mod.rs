use crate::args::types::{AppError, ConvertArgs, ConvertType, Language, MainArgs, OutputType};
use crate::prelude::print;
use crate::Runnable;
use chrono::prelude::*;
use chrono::Duration;
use either::Either;
use heca_lib::HebrewDate;
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Return {
    pub day: Either<[chrono::DateTime<Utc>; 2], [HebrewDate; 2]>,
    pub orig_day: Either<HebrewDate, chrono::DateTime<Utc>>,
}

impl Serialize for Return {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.day {
            Either::Left(val) => serialize_array(val, serializer),
            Either::Right(val) => serialize_array(val, serializer),
        }
    }
}

fn serialize_array<S, A>(cv: [A; 2], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    A: Serialize,
{
    let mut seq = serializer.serialize_seq(Some(2))?;
    for e in &cv {
        seq.serialize_element(e)?;
    }
    seq.end()
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
    fn json_print(&self) -> Result<(), AppError> {
        match &self.day {
            Either::Right(r) => println!("{}", serde_json::to_string(&r).unwrap()),
            Either::Left(r) => println!("{}", serde_json::to_string(&r).unwrap()),
        };
        Ok(())
    }
}

impl Return {
    fn print(&self, args: &MainArgs) -> Result<(), AppError> {
        match args.output_type {
            OutputType::JSON => self.json_print(),
            OutputType::Pretty | OutputType::Regular => self.pretty_print(args),
        }
    }
}

impl Runnable for ConvertArgs {
    fn run(&self, args: &MainArgs) -> Result<(), AppError> {
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

        ret.print(args)?;
        Ok(())
    }
}
