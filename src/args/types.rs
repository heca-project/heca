use chrono::prelude::*;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;

pub struct MainArgs {
    pub config: Option<String>,
    pub output_type: OutputType,
    pub language: Language,
    pub command: Command,
}

#[derive(Eq, PartialEq)]
pub enum Language {
    English,
    Hebrew,
}

pub enum Command {
    Convert(ConvertArgs),
    List(ListArgs),
}

#[derive(Eq, PartialEq)]
pub enum OutputType {
    Regular,
    Pretty,
    JSON,
}

pub struct ConvertArgs {
    date: ConvertType,
}

pub enum ConvertType {
    Gregorian(chrono::DateTime<Utc>),
    Hebrew(HebrewDate),
}

pub struct ListArgs {
    pub year: YearType,
    pub location: Location,
    pub events: Vec<TorahReadingType>,
    pub shuffle: bool,
    pub amnt_years: u64,
}

#[derive(Eq, PartialEq)]
pub enum YearType {
    Gregorian(u64),
    Hebrew(u64),
}
