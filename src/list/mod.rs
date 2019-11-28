use crate::args::types::{
    AppError, CustomHoliday, DayVal, Event, ListArgs, MainArgs, MinorHoliday, Name, OutputType,
    YearType,
};
use crate::prelude::constants::get_minor_holidays;
use crate::prelude::get_omer::get_omer;
use crate::prelude::print;
use crate::{Printable, Runnable};
use chrono::prelude::*;
use heca_lib::prelude::TorahReadingType;
use heca_lib::HebrewYear;
use rayon::prelude::*;
use serde::Serialize;
use std::convert::TryInto;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Return {
    list: Vec<DayVal>,
}

impl Return {
    fn pretty_print(&self, args: MainArgs) -> Result<(), AppError> {
        use std::io::stdout;
        use std::io::BufWriter;
        use std::io::Write;
        let stdout = stdout();
        let mut lock = BufWriter::with_capacity(100, stdout.lock());
        self.list.iter().for_each(|d| {
            let ret = d.day;
            let year = ret.year();
            let month = ret.month();
            let day = ret.day();
            let name = d.name.clone();

            let mut year_arr = [b'\0'; 16];
            let mut month_arr = [b'\0'; 2];
            let mut day_arr = [b'\0'; 2];
            let count_y = itoa::write(&mut year_arr[..], year).unwrap();
            let count_m = itoa::write(&mut month_arr[..], month).unwrap();
            let count_d = itoa::write(&mut day_arr[..], day).unwrap();
            lock.write_all(&year_arr[..count_y as usize]).ok();
            lock.write_all(b"/").ok();
            lock.write_all(&month_arr[..count_m as usize]).ok();
            lock.write_all(b"/").ok();
            lock.write_all(&day_arr[..count_d as usize]).ok();
            lock.write_all(b" ").ok();
            match name {
                Name::TorahReading(name) => lock
                    .write(print::torah_reading(name, args.language).as_bytes())
                    .ok(),
                Name::MinorDays(day) => lock
                    .write(print::minor_holidays(day, args.language).as_bytes())
                    .ok(),
                Name::CustomHoliday(custom_holiday) => {
                    lock.write(custom_holiday.printable.as_bytes()).ok()
                }
            };
            lock.write_all(b"\n").unwrap();
        });
        Ok(())
    }
    fn json_print(&self) -> Result<(), AppError> {
        println!("{}", serde_json::to_string(&self).unwrap());
        Ok(())
    }
}
impl Printable for Return {
    fn print(&self, args: MainArgs) -> Result<(), AppError> {
        match args.output_type {
            OutputType::JSON => self.json_print(),
            OutputType::Pretty | OutputType::Regular => self.pretty_print(args),
        }
    }
}

impl Runnable<Return> for ListArgs {
    fn run(&self, _args: &MainArgs) -> Result<Return, AppError> {
        let mut part1: Vec<Vec<DayVal>> = Vec::with_capacity(self.amnt_years as usize);

        let main_events = self
            .events
            .iter()
            .map(|x| {
                if let Event::TorahReadingType(trr) = x {
                    Some(trr)
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .map(|x| *x.unwrap())
            .collect::<Vec<TorahReadingType>>();

        let custom_events = self
            .events
            .iter()
            .filter_map(|x| {
                if let Event::CustomHoliday(custom_holiday) = x {
                    Some(custom_holiday.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<CustomHoliday>>();
        let result: Result<Return, AppError> = match self.year {
            YearType::Hebrew(year) => {
                HebrewYear::new(year)?;
                HebrewYear::new(year + self.amnt_years)?;

                (0 as u32..(self.amnt_years as u32))
                    .into_par_iter()
                    .map(|x| {
                        let mut ret: Vec<DayVal> = Vec::with_capacity(200);
                        let year = HebrewYear::new(x as u64 + year).unwrap();

                        ret.extend(
                            year.get_holidays(self.location, &main_events)
                                .into_iter()
                                .map(|x| DayVal {
                                    day: x.day().into(),
                                    name: Name::TorahReading(x.name()),
                                }),
                        );

                        if self
                            .events
                            .contains(&Event::MinorHoliday(MinorHoliday::Omer))
                        {
                            ret.extend_from_slice(&get_omer(&year));
                        }
                        if self
                            .events
                            .contains(&Event::MinorHoliday(MinorHoliday::Minor))
                        {
                            ret.extend(get_minor_holidays(&year));
                        }
                        custom_events.iter().for_each(|x| {
                            if let Ok(day) = year.get_hebrew_date(x.date.month, x.date.day) {
                                let d = DayVal {
                                    name: Name::CustomHoliday(x.clone()),
                                    day: day.try_into().unwrap(),
                                };
                                ret.push(d);
                            } else if let Some(not_exists) = &x.if_not_exists {
                                not_exists.iter().for_each(|day_month| {
                                    if let Ok(day) =
                                        year.get_hebrew_date(day_month.month, day_month.day)
                                    {
                                        let d = DayVal {
                                            name: Name::CustomHoliday(x.clone()),
                                            day: day.into(),
                                        };
                                        ret.push(d);
                                    }
                                });
                            }
                        });

                        ret
                    })
                    .collect_into_vec(&mut part1);
                let mut part2: Vec<DayVal> = Vec::with_capacity((self.amnt_years as usize) * 100);
                part1.into_iter().flatten().for_each(|x| part2.push(x));
                Ok(Return { list: part2 })
            }

            YearType::Gregorian(year) => {
                let that_year = year + 3760 - 1;
                HebrewYear::new(that_year)?;
                HebrewYear::new(that_year + self.amnt_years)?;
                (0 as u32..(self.amnt_years as u32) + 2)
                    .into_par_iter()
                    .map(|x| {
                        let mut ret = Vec::with_capacity(200);
                        let heb_year = HebrewYear::new(x as u64 + that_year).unwrap();

                        ret.extend(
                            heb_year
                                .get_holidays(self.location, &main_events)
                                .into_iter()
                                .map(|x| DayVal {
                                    day: x.day().into(),
                                    name: Name::TorahReading(x.name()),
                                }),
                        );
                        if self
                            .events
                            .contains(&Event::MinorHoliday(MinorHoliday::Omer))
                        {
                            ret.extend_from_slice(&get_omer(&heb_year));
                        }
                        if self
                            .events
                            .contains(&Event::MinorHoliday(MinorHoliday::Minor))
                        {
                            ret.extend(get_minor_holidays(&heb_year));
                        }
                        custom_events.iter().for_each(|x| {
                            if let Ok(day) = heb_year.get_hebrew_date(x.date.month, x.date.day) {
                                let d = DayVal {
                                    name: Name::CustomHoliday(x.clone()),
                                    day: day.try_into().unwrap(),
                                };
                                ret.push(d);
                            } else if let Some(not_exists) = &x.if_not_exists {
                                not_exists.iter().for_each(|day_month| {
                                    if let Ok(day) =
                                        heb_year.get_hebrew_date(day_month.month, day_month.day)
                                    {
                                        let d = DayVal {
                                            name: Name::CustomHoliday(x.clone()),
                                            day: day.into(),
                                        };
                                        ret.push(d);
                                    }
                                });
                            }
                        });
                        ret
                    })
                    .collect_into_vec(&mut part1);
                let mut part2: Vec<DayVal> = Vec::with_capacity((self.amnt_years as usize) * 100);
                part1
                    .into_iter()
                    .flatten()
                    .filter(|x| x.day > Utc.ymd(year as i32, 1, 1).and_hms(0, 0, 0))
                    .filter(|x| {
                        x.day
                            < Utc
                                .ymd((year + self.amnt_years) as i32, 1, 1)
                                .and_hms(0, 0, 0)
                    })
                    .for_each(|x| part2.push(x));

                Ok(Return { list: part2 })
            }
        };
        let mut result1 = result?;
        if !self.no_sort {
            result1.list.par_sort_unstable_by(|a, b| a.day.cmp(&b.day));
        }
        Ok(result1)
    }
}
