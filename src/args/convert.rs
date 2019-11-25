use crate::args::prelude::{parse_hebrew, ConfigDateFmt, ConfigDateType};
use crate::args::types::{AppError, Command, ConvertArgs, ConvertType, Language};
use crate::args::DATE_TOKEN;
use chrono::prelude::*;
use heca_lib::HebrewDate;

pub fn parse_options(
    date: &str,
    language: Language,
    datefmt: ConfigDateFmt,
    date_type: ConfigDateType,
) -> Result<Command, AppError> {
    let sp = date.split(&DATE_TOKEN[..]).collect::<Vec<&str>>();
    if sp.len() != 3 {
        return Err(AppError::SplitDateError);
    }

    Ok(match date_type {
        ConfigDateType::Hebrew => parse_convert_hebrew(&sp, language)?,
        ConfigDateType::Gregorian => parse_convert_gregorian(&sp, datefmt, language)?,
        ConfigDateType::Fuzzy => {
            if sp[1].parse::<u8>().is_ok() {
                parse_convert_gregorian(&sp, datefmt, language)?
            } else {
                parse_convert_hebrew(&sp, language)?
            }
        }
    })
}

pub fn parse_convert_hebrew(sp: &[&str], language: Language) -> Result<Command, AppError> {
    let (day, month, year) = parse_hebrew(sp)?;
    Ok(Command::Convert(ConvertArgs {
        language,
        date: ConvertType::Hebrew(HebrewDate::from_ymd(year.unwrap(), month, day)?),
    }))
}

pub fn parse_convert_gregorian(
    sp: &[&str],
    format: ConfigDateFmt,
    language: Language,
) -> Result<Command, AppError> {
    let (day, month, year) = match format {
        ConfigDateFmt::ISO | ConfigDateFmt::B => {
            let year = sp[0]
                .parse()
                .map_err(|_| AppError::CannotParseYear(sp[0].into()))?;
            let month = sp[1]
                .parse()
                .map_err(|_| AppError::CannotParseMonth(sp[1].into()))?;
            let day = sp[2]
                .parse()
                .map_err(|_| AppError::CannotParseDay(sp[2].into()))?;
            (day, month, year)
        }
        ConfigDateFmt::US | ConfigDateFmt::M => {
            let year = sp[2]
                .parse()
                .map_err(|_| AppError::CannotParseYear(sp[2].into()))?;
            let month = sp[0]
                .parse()
                .map_err(|_| AppError::CannotParseMonth(sp[0].into()))?;
            let day = sp[1]
                .parse()
                .map_err(|_| AppError::CannotParseDay(sp[1].into()))?;

            (day, month, year)
        }
        ConfigDateFmt::UK | ConfigDateFmt::L => {
            let year = sp[2]
                .parse()
                .map_err(|_| AppError::CannotParseYear(sp[2].into()))?;
            let month = sp[1]
                .parse()
                .map_err(|_| AppError::CannotParseMonth(sp[1].into()))?;
            let day = sp[0]
                .parse()
                .map_err(|_| AppError::CannotParseDay(sp[0].into()))?;

            (day, month, year)
        }
    };
    Ok(Command::Convert(ConvertArgs {
        language,
        date: ConvertType::Gregorian(
            Utc.ymd_opt(year, month, day)
                .single()
                .ok_or_else(|| AppError::InvalidGregorianDate(year, month, day))?,
        ),
    }))
}
