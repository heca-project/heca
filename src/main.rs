mod args;
<<<<<<< HEAD
mod convert;
mod list;
mod prelude;
=======
>>>>>>> 5f4c91114052d99937872bb660352ea1d38bb83d

use crate::args::types;
use crate::args::types::AppError;
use crate::args::types::*;
use crate::prelude::*;

fn main() {
    let output_type = output_type();
    if let Err(err) = app(std::env::args(), output_type) {
        if output_type == OutputType::JSON {
            eprintln!("{}", serde_json::to_string(&err).unwrap());
        } else {
            eprintln!("{}", err);
        }
        std::process::exit(1);
    }
}

fn output_type() -> OutputType {
    let mut args = std::env::args();
    loop {
        let arg = args.next();
        if arg == None {
            break;
        } else if let Some(arg) = arg {
            if arg == "--print=json" {
                return OutputType::JSON;
            } else if arg == "--print" {
                if let Some(next) = args.next() {
                    if next == "json" {
                        return OutputType::JSON;
                    }
                }
            }
        }
    }

    if let Ok(json_str) = std::env::var("JSON") {
        if json_str == "YES" {
            return OutputType::JSON;
        }
    }

    OutputType::Pretty
}

fn app<I, T>(args: I, output_type: OutputType) -> Result<(), AppError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = args::build_args(args, output_type)?;
    match args.command {
        Command::List(ref sub_args) => sub_args.run(&args)?.print(args)?,
        Command::Convert(ref sub_args) => sub_args.run(&args)?.print(args)?,
    };

    Ok(())
}

<<<<<<< HEAD

=======
trait Runnable<T: Printable> {
    fn run(&self, args: &MainArgs) -> Result<T, AppError>;
}

trait Printable {
    fn print(&self, args: MainArgs) -> Result<(), AppError>;
    fn print_json(&self) -> Result<(), AppError>;
}

impl Runnable<ConvertReturn> for ConvertArgs {
    fn run(&self, _args: &MainArgs) -> Result<ConvertReturn, AppError> {
        match self.date {
            ConvertType::Gregorian(date) => Ok(ConvertReturn {
                orig_day: Either::Right(date.and_hms(0, 0, 1)),
                day: Either::Right([
                    date.and_hms(0, 0, 1).try_into()?,
                    date.and_hms(23, 0, 1).try_into()?,
                ]),
            }),
            ConvertType::Hebrew(date) => Ok(ConvertReturn {
                orig_day: Either::Left(date),
                day: Either::Left({
                    let first_day: DateTime<Utc> = date.into();
                    [first_day, first_day + Duration::days(1)]
                }),
            }),
        }
    }
}

impl Runnable<ListReturn> for ListArgs {
    fn run(&self, _args: &MainArgs) -> Result<ListReturn, AppError> {
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
        let result: Result<ListReturn, AppError> = match self.year {
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
                Ok(ListReturn { list: part2 })
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

                Ok(ListReturn { list: part2 })
            }
        };
        let mut result1 = result?;
        if !self.no_sort {
            result1.list.par_sort_unstable_by(|a, b| a.day.cmp(&b.day));
        }
        Ok(result1)
    }
}

#[derive(Debug)]
struct ConvertReturn {
    pub day: Either<[chrono::DateTime<Utc>; 2], [HebrewDate; 2]>,
    pub orig_day: Either<HebrewDate, chrono::DateTime<Utc>>,
}

impl Serialize for ConvertReturn {
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

#[derive(Debug, Serialize)]
#[serde(transparent)]
struct ListReturn {
    list: Vec<DayVal>,
}

impl Printable for ConvertReturn {
    fn print_json(&self) -> Result<(), AppError> {
        match &self.day {
            Either::Right(r) => println!("{}", serde_json::to_string(&r).unwrap()),
            Either::Left(r) => println!("{}", serde_json::to_string(&r).unwrap()),
        };
        Ok(())
    }
    fn print(&self, args: MainArgs) -> Result<(), AppError> {
        match args.language {
            Language::English => match self.orig_day {
                Either::Right(r) => println!(
                    "{}: From {} {} {} to {} {} {}.",
                    r.format("%A %B %-d %Y"),
                    self.day.right().unwrap()[0].day(),
                    print_hebrew_month_english(self.day.right().unwrap()[0].month()),
                    self.day.right().unwrap()[0].year(),
                    self.day.right().unwrap()[1].day(),
                    print_hebrew_month_english(self.day.right().unwrap()[1].month()),
                    self.day.right().unwrap()[1].year(),
                ),
                Either::Left(l) => println!(
                    "{} {} {} -> From sunset {} to sunset {}.",
                    l.day(),
                    print_hebrew_month_english(l.month()),
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
                    print_hebrew_month_hebrew(self.day.right().unwrap()[0].month()),
                    self.day.right().unwrap()[0].year(),
                    self.day.right().unwrap()[1].day(),
                    print_hebrew_month_hebrew(self.day.right().unwrap()[1].month()),
                    self.day.right().unwrap()[1].year(),
                ),
                Either::Left(l) => println!(
                    "{} {} {}: {} - {}.",
                    l.day(),
                    print_hebrew_month_hebrew(l.month()),
                    l.year(),
                    self.day.left().unwrap()[0].format("%A %B %-d %Y"),
                    self.day.left().unwrap()[1].format("%A %B %-d %Y"),
                ),
            },
        };
        Ok(())
    }
}

fn print_hebrew_month_english(h: HebrewMonth) -> &'static str {
    match h {
        HebrewMonth::Tishrei => "Tishrei",
        HebrewMonth::Cheshvan => "Cheshvan",
        HebrewMonth::Kislev => "Kislev",
        HebrewMonth::Teves => "Teves",
        HebrewMonth::Shvat => "Shvat",
        HebrewMonth::Adar => "Adar",
        HebrewMonth::Adar1 => "Adar Rishon",
        HebrewMonth::Adar2 => "Adar Sheni",
        HebrewMonth::Nissan => "Nissan",
        HebrewMonth::Iyar => "Iyar",
        HebrewMonth::Sivan => "Sivan",
        HebrewMonth::Tammuz => "Tammuz",
        HebrewMonth::Av => "Av",
        HebrewMonth::Elul => "Elul",
    }
}

fn print_hebrew_month_hebrew(h: HebrewMonth) -> &'static str {
    match h {
        HebrewMonth::Tishrei => "תשרי",
        HebrewMonth::Cheshvan => "חשוון",
        HebrewMonth::Kislev => "כסלו",
        HebrewMonth::Teves => "טבת",
        HebrewMonth::Shvat => "שבט",
        HebrewMonth::Adar => "אדר",
        HebrewMonth::Adar1 => "אדר א",
        HebrewMonth::Adar2 => "אדר ב",
        HebrewMonth::Nissan => "ניסן",
        HebrewMonth::Iyar => "אייר",
        HebrewMonth::Sivan => "סיוון",
        HebrewMonth::Tammuz => "תמוז",
        HebrewMonth::Av => "אב",
        HebrewMonth::Elul => "אלול",
    }
}

impl Printable for ListReturn {
    fn print(&self, args: MainArgs) -> Result<(), AppError> {
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
                Name::TorahReading(name) => {
                    lock.write(print_tr(name, args.language).as_bytes()).ok()
                }
                Name::MinorDays(day) => lock.write(print_md(day, args.language).as_bytes()).ok(),
                Name::CustomHoliday(custom_holiday) => {
                    lock.write(custom_holiday.printable.as_bytes()).ok()
                }
            };
            lock.write_all(b"\n").unwrap();
        });
        Ok(())
    }
    fn print_json(&self) -> Result<(), AppError> {
        println!("{}", serde_json::to_string(&self).unwrap());
        Ok(())
    }
}

fn print_md(tr: MinorDays, language: types::Language) -> &'static str {
    match language {
        Language::English => match tr {
            //Generated from https://play.golang.org/p/HtWEMOgflMt
            MinorDays::Omer1 => "1st day of the Omer",
            MinorDays::Omer2 => "2nd day of the Omer",
            MinorDays::Omer3 => "3rd day of the Omer",
            MinorDays::Omer4 => "4th day of the Omer",
            MinorDays::Omer5 => "5th day of the Omer",
            MinorDays::Omer6 => "6th day of the Omer",
            MinorDays::Omer7 => "7th day of the Omer",
            MinorDays::Omer8 => "8th day of the Omer",
            MinorDays::Omer9 => "9th day of the Omer",
            MinorDays::Omer10 => "10th day of the Omer",
            MinorDays::Omer11 => "11th day of the Omer",
            MinorDays::Omer12 => "12th day of the Omer",
            MinorDays::Omer13 => "13th day of the Omer",
            MinorDays::Omer14 => "14th day of the Omer",
            MinorDays::Omer15 => "15th day of the Omer",
            MinorDays::Omer16 => "16th day of the Omer",
            MinorDays::Omer17 => "17th day of the Omer",
            MinorDays::Omer18 => "18th day of the Omer",
            MinorDays::Omer19 => "19th day of the Omer",
            MinorDays::Omer20 => "20th day of the Omer",
            MinorDays::Omer21 => "21st day of the Omer",
            MinorDays::Omer22 => "22nd day of the Omer",
            MinorDays::Omer23 => "23rd day of the Omer",
            MinorDays::Omer24 => "24th day of the Omer",
            MinorDays::Omer25 => "25th day of the Omer",
            MinorDays::Omer26 => "26th day of the Omer",
            MinorDays::Omer27 => "27th day of the Omer",
            MinorDays::Omer28 => "28th day of the Omer",
            MinorDays::Omer29 => "29th day of the Omer",
            MinorDays::Omer30 => "30th day of the Omer",
            MinorDays::Omer31 => "31st day of the Omer",
            MinorDays::Omer32 => "32nd day of the Omer",
            MinorDays::Omer33 => "33rd day of the Omer",
            MinorDays::Omer34 => "34th day of the Omer",
            MinorDays::Omer35 => "35th day of the Omer",
            MinorDays::Omer36 => "36th day of the Omer",
            MinorDays::Omer37 => "37th day of the Omer",
            MinorDays::Omer38 => "38th day of the Omer",
            MinorDays::Omer39 => "39th day of the Omer",
            MinorDays::Omer40 => "40th day of the Omer",
            MinorDays::Omer41 => "41st day of the Omer",
            MinorDays::Omer42 => "42nd day of the Omer",
            MinorDays::Omer43 => "43rd day of the Omer",
            MinorDays::Omer44 => "44th day of the Omer",
            MinorDays::Omer45 => "45th day of the Omer",
            MinorDays::Omer46 => "46th day of the Omer",
            MinorDays::Omer47 => "47th day of the Omer",
            MinorDays::Omer48 => "48th day of the Omer",
            MinorDays::Omer49 => "49th day of the Omer",
            MinorDays::ErevPesach => "Erev Pesach",
            MinorDays::ErevSukkos => "Erev Sukkos",
            MinorDays::ErevShavuos => "Erev Shavuos",
            MinorDays::ErevYomKippur => "Erev Yom Kippur",
            MinorDays::ErevRoshHashanah => "Erev Rosh Hashana",
            MinorDays::PesachSheni => "Pesach Sheni",
            MinorDays::LagBaOmer => "Lag BaOmer",
            MinorDays::FifteenAv => "15th of Av",
            MinorDays::FifteenShvat => "15th of Shevat",
            MinorDays::PurimKattan => "Purim Kattan",
            MinorDays::ShushanPurimKattan => "Shushan Purim Kattan",
        },
        Language::Hebrew => match tr {
            //generated from https://play.golang.org/p/LH0qQmYxZsP
            MinorDays::Omer1 => "היום יום 1 לעומר",
            MinorDays::Omer2 => "היום יום 2 לעומר",
            MinorDays::Omer3 => "היום יום 3 לעומר",
            MinorDays::Omer4 => "היום יום 4 לעומר",
            MinorDays::Omer5 => "היום יום 5 לעומר",
            MinorDays::Omer6 => "היום יום 6 לעומר",
            MinorDays::Omer7 => "היום יום 7 לעומר",
            MinorDays::Omer8 => "היום יום 8 לעומר",
            MinorDays::Omer9 => "היום יום 9 לעומר",
            MinorDays::Omer10 => "היום יום 10 לעומר",
            MinorDays::Omer11 => "היום יום 11 לעומר",
            MinorDays::Omer12 => "היום יום 12 לעומר",
            MinorDays::Omer13 => "היום יום 13 לעומר",
            MinorDays::Omer14 => "היום יום 14 לעומר",
            MinorDays::Omer15 => "היום יום 15 לעומר",
            MinorDays::Omer16 => "היום יום 16 לעומר",
            MinorDays::Omer17 => "היום יום 17 לעומר",
            MinorDays::Omer18 => "היום יום 18 לעומר",
            MinorDays::Omer19 => "היום יום 19 לעומר",
            MinorDays::Omer20 => "היום יום 20 לעומר",
            MinorDays::Omer21 => "היום יום 21 לעומר",
            MinorDays::Omer22 => "היום יום 22 לעומר",
            MinorDays::Omer23 => "היום יום 23 לעומר",
            MinorDays::Omer24 => "היום יום 24 לעומר",
            MinorDays::Omer25 => "היום יום 25 לעומר",
            MinorDays::Omer26 => "היום יום 26 לעומר",
            MinorDays::Omer27 => "היום יום 27 לעומר",
            MinorDays::Omer28 => "היום יום 28 לעומר",
            MinorDays::Omer29 => "היום יום 29 לעומר",
            MinorDays::Omer30 => "היום יום 30 לעומר",
            MinorDays::Omer31 => "היום יום 31 לעומר",
            MinorDays::Omer32 => "היום יום 32 לעומר",
            MinorDays::Omer33 => "היום יום 33 לעומר",
            MinorDays::Omer34 => "היום יום 34 לעומר",
            MinorDays::Omer35 => "היום יום 35 לעומר",
            MinorDays::Omer36 => "היום יום 36 לעומר",
            MinorDays::Omer37 => "היום יום 37 לעומר",
            MinorDays::Omer38 => "היום יום 38 לעומר",
            MinorDays::Omer39 => "היום יום 39 לעומר",
            MinorDays::Omer40 => "היום יום 40 לעומר",
            MinorDays::Omer41 => "היום יום 41 לעומר",
            MinorDays::Omer42 => "היום יום 42 לעומר",
            MinorDays::Omer43 => "היום יום 43 לעומר",
            MinorDays::Omer44 => "היום יום 44 לעומר",
            MinorDays::Omer45 => "היום יום 45 לעומר",
            MinorDays::Omer46 => "היום יום 46 לעומר",
            MinorDays::Omer47 => "היום יום 47 לעומר",
            MinorDays::Omer48 => "היום יום 48 לעומר",
            MinorDays::Omer49 => "היום יום 49 לעומר",
            MinorDays::ErevPesach => "ערב פסח",
            MinorDays::ErevSukkos => "ערב סוכות",
            MinorDays::ErevShavuos => "ערב שבועות",
            MinorDays::ErevYomKippur => "ערב יום כיפור",
            MinorDays::ErevRoshHashanah => "ערב ראש השנה",
            MinorDays::PesachSheni => "ערב פסח שני",
            MinorDays::LagBaOmer => "ל\"ג בעומר",
            MinorDays::FifteenAv => "ט\"ו באב",
            MinorDays::FifteenShvat => "ט\"ו בשבט",
            MinorDays::PurimKattan => "פורים קטן",
            MinorDays::ShushanPurimKattan => "שושן פורים קטן",
        },
    }
}

fn print_tr(tr: TorahReading, language: types::Language) -> &'static str {
    match language {
        Language::English => match tr {
            TorahReading::YomTov(yt) => match yt {
                YomTov::RoshHashanah1 => "1st day of Rosh Hashanah",
                YomTov::RoshHashanah2 => "2nd day of Rosh Hashanah",
                YomTov::YomKippur => "Yom Kippur",
                YomTov::Sukkos1 => "1st day of Sukkos",
                YomTov::Sukkos2 => "2nd day of Sukkos",
                YomTov::Sukkos3 => "3rd day of Sukkos",
                YomTov::Sukkos4 => "4th day of Sukkos",
                YomTov::Sukkos5 => "5th day of Sukkos",
                YomTov::Sukkos6 => "6th day of Sukkos",
                YomTov::Sukkos7 => "7th day of Sukkos",
                YomTov::ShminiAtzeres => "Shmini Atzeres",
                YomTov::SimchasTorah => "Simchas Torah",
                YomTov::Pesach1 => "1st day of Pesach",
                YomTov::Pesach2 => "2nd day of Pesach",
                YomTov::Pesach3 => "3rd day of Pesach",
                YomTov::Pesach4 => "4th day of Pesach",
                YomTov::Pesach5 => "5th day of Pesach",
                YomTov::Pesach6 => "6th day of Pesach",
                YomTov::Pesach7 => "7th day of Pesach",
                YomTov::Pesach8 => "8th day of Pesach",
                YomTov::Shavuos1 => "1st day of Shavuos",
                YomTov::Shavuos2 => "2nd day of Shavuos",
            },
            TorahReading::Chol(tr) => match tr {
                Chol::RoshChodeshCheshvan1 => "1st day of Rosh Chodesh Cheshvan",
                Chol::RoshChodeshCheshvan2 => "2nd day of Rosh Chodesh Cheshvan",
                Chol::RoshChodeshKislev => "Rosh Chodesh Kislev",
                Chol::RoshChodeshKislev1 => "1st day of Rosh Chodesh Kislev",
                Chol::RoshChodeshKislev2 => "2nd day of Rosh Chodesh Kislev",
                Chol::RoshChodeshTeves => "Rosh Chodesh Teves",
                Chol::RoshChodeshTeves1 => "1st day of Rosh Chodesh Teves",
                Chol::RoshChodeshTeves2 => "2nd day of Rosh Chodesh Teves",
                Chol::RoshChodeshShvat => "Rosh Chodesh Shvat",
                Chol::RoshChodeshAdar1 => "1st day of Rosh Chodesh Adar",
                Chol::RoshChodeshAdar2 => "2nd day of Rosh Chodesh Adar",
                Chol::RoshChodeshAdarRishon1 => "1st day of Rosh Chodesh Adar Rishon",
                Chol::RoshChodeshAdarRishon2 => "2nd day of Rosh Chodesh Adar Rishon",
                Chol::RoshChodeshAdarSheni1 => "1st day of Rosh Chodesh Adar Sheni",
                Chol::RoshChodeshAdarSheni2 => "2nd day of Rosh Chodesh Adar Sheni",
                Chol::RoshChodeshNissan => "Rosh Chodesh Nissan",
                Chol::RoshChodeshIyar1 => "1st day of Rosh Chodesh Iyar",
                Chol::RoshChodeshIyar2 => "2nd day of Rosh Chodesh Iyar",
                Chol::RoshChodeshSivan => "Rosh Chodesh Sivan",
                Chol::RoshChodeshTammuz1 => "1st day of Rosh Chodesh Tammuz",
                Chol::RoshChodeshTammuz2 => "2nd day of Rosh Chodesh Tammuz",
                Chol::RoshChodeshAv => "Rosh Chodesh Av",
                Chol::RoshChodeshElul1 => "1st day of Rosh Chodesh Elul",
                Chol::RoshChodeshElul2 => "2nd day of Rosh Chodesh Elul",
                Chol::Chanukah1 => "1st day of Chanukah",
                Chol::Chanukah2 => "2nd day of Chanukah",
                Chol::Chanukah3 => "3rd day of Chanukah",
                Chol::Chanukah4 => "4rd day of Chanukah",
                Chol::Chanukah5 => "5rd day of Chanukah",
                Chol::Chanukah6 => "6rd day of Chanukah",
                Chol::Chanukah7 => "7rd day of Chanukah",
                Chol::Chanukah8 => "8rd day of Chanukah",
                Chol::TzomGedalia => "Tzom Gedalia",
                Chol::TaanisEsther => "Taanis Esther",
                Chol::TenTeves => "Tenth of Teves",
                Chol::Purim => "Purim",
                Chol::ShushanPurim => "Shushan Purim",
                Chol::SeventeenTammuz => "Seventeenth of Tammuz",
                Chol::NineAv => "Ninth of Av",
            },
            TorahReading::Shabbos(tr) => match tr {
                Parsha::Haazinu => "Haazina",
                Parsha::Vayelech => "Vayelech",
                Parsha::Bereishis => "Bereishis",
                Parsha::Noach => "Noach",
                Parsha::LechLecha => "Lech Lecha",
                Parsha::Vayeira => "Vayeira",
                Parsha::ChayeiSara => "Chayei Sarah",
                Parsha::Toldos => "Toldos",
                Parsha::Vayetzei => "Vayetzei",
                Parsha::Vayishlach => "Vayishlach",
                Parsha::Vayeshev => "Vayeshev",
                Parsha::Miketz => "Miketz",
                Parsha::Vayigash => "Vayigash",
                Parsha::Vayechi => "Vayechi",
                Parsha::Shemos => "Shemos",
                Parsha::Vaeira => "Vaeira",
                Parsha::Bo => "Bo",
                Parsha::Beshalach => "Beshalach",
                Parsha::Yisro => "Yisro",
                Parsha::Mishpatim => "Mishpatim",
                Parsha::Terumah => "Terumah",
                Parsha::Tetzaveh => "Tetzaveh",
                Parsha::KiSisa => "Ki Sisa",
                Parsha::VayakhelPikudei => "Vayakhel/Pikudei",
                Parsha::Vayakhel => "Vayakhel",
                Parsha::Pikudei => "Pikudei",
                Parsha::Vayikra => "Vayikra",
                Parsha::Tzav => "Tzav",
                Parsha::Shemini => "Shemini",
                Parsha::TazriyaMetzorah => "Tazriya/Metzorah",
                Parsha::Tazriya => "Tazriya",
                Parsha::Metzorah => "Metzorah",
                Parsha::AchareiMosKedoshim => "Acharei Mos/Kedoshim",
                Parsha::AchareiMos => "Acharei Mos",
                Parsha::Kedoshim => "Kedoshim",
                Parsha::Emor => "Emor",
                Parsha::BeharBechukosai => "Behar/Bechukosai",
                Parsha::Behar => "Behar",
                Parsha::Bechukosai => "Bechukosai",
                Parsha::Bamidbar => "Bamidbar",
                Parsha::Naso => "Naso",
                Parsha::Behaaloscha => "Behaaloscha",
                Parsha::Shlach => "Shlach",
                Parsha::Korach => "Korach",
                Parsha::ChukasBalak => "Chukas/Balak",
                Parsha::Chukas => "Chukas",
                Parsha::Balak => "Balak",
                Parsha::Pinchas => "Pinchas",
                Parsha::MatosMaasei => "Matos/Maasei",
                Parsha::Matos => "Matos",
                Parsha::Maasei => "Maasei",
                Parsha::Devarim => "Devarim",
                Parsha::Vaeschanan => "Vaeschanan",
                Parsha::Eikev => "Eikev",
                Parsha::Reeh => "Re'eh",
                Parsha::Shoftim => "Shoftim",
                Parsha::KiSeitzei => "Ki Seitzei",
                Parsha::KiSavoh => "Ki Savo",
                Parsha::NitzavimVayelech => "Nitzavim/Vayelech",
                Parsha::Nitzavim => "Nitzavim",
            },
            TorahReading::SpecialParsha(tr) => match tr {
                SpecialParsha::Zachor => "Parshas Zachor",
                SpecialParsha::HaChodesh => "Parshas HaChodesh",
                SpecialParsha::Parah => "Parshas Parah",
                SpecialParsha::Shekalim => "Parshas Shekalim",
            },
        },
        Language::Hebrew => match tr {
            TorahReading::YomTov(yt) => match yt {
                YomTov::RoshHashanah1 => "יןם א של ראש השנה",
                YomTov::RoshHashanah2 => "יןם ב של ראש השנה",
                YomTov::YomKippur => "יום כיפור",
                YomTov::Sukkos1 => "יום א של חג הסוכות",
                YomTov::Sukkos2 => "יום ב של חג הסוכות",
                YomTov::Sukkos3 => "יום ג  של חג הסוכות",
                YomTov::Sukkos4 => "יום ד של חג הסוכות",
                YomTov::Sukkos5 => "יום ה של חג הסוכות",
                YomTov::Sukkos6 => "יום ו של חג הסוכות",
                YomTov::Sukkos7 => "יום ז של חג הסוכות",
                YomTov::ShminiAtzeres => "שמיני עצרת",
                YomTov::SimchasTorah => "שמחת תורה",
                YomTov::Pesach1 => "יום א של חג הפסח",
                YomTov::Pesach2 => "יום ב של חג הפסח",
                YomTov::Pesach3 => "יום ג של חג הפסח",
                YomTov::Pesach4 => "יום ד של חג הפסח",
                YomTov::Pesach5 => "יום ה של חג הפסח",
                YomTov::Pesach6 => "יום ו של חג הפסח",
                YomTov::Pesach7 => "יום ז של חג הפסח",
                YomTov::Pesach8 => "יום ח של חג הפסח",
                YomTov::Shavuos1 => "יום א של חג השבועות",
                YomTov::Shavuos2 => "יום ב של חג השבועות",
            },
            TorahReading::Chol(tr) => match tr {
                Chol::RoshChodeshCheshvan1 => "יום א של ראש חודש חשון",
                Chol::RoshChodeshCheshvan2 => "יום ב של ראש חודש חשון",
                Chol::RoshChodeshKislev => "ראש חודש כסלו",
                Chol::RoshChodeshKislev1 => "יום א של ראש חודש כסלו",
                Chol::RoshChodeshKislev2 => "יום ב של ראש חודש כסלו",
                Chol::RoshChodeshTeves => "ראש חודש טבת",
                Chol::RoshChodeshTeves1 => "יום א של ראש חודש טבת",
                Chol::RoshChodeshTeves2 => "יום ב של ראש חודש טבת",
                Chol::RoshChodeshShvat => "ראש חודש שבט",
                Chol::RoshChodeshAdar1 => "יום א של ראש חודש אדר",
                Chol::RoshChodeshAdar2 => "יום ב של ראש חודש אדר",
                Chol::RoshChodeshAdarRishon1 => "יום א של ראש חודש אדר ראשון",
                Chol::RoshChodeshAdarRishon2 => "יום ב של ראש חודש אדר ראשון",
                Chol::RoshChodeshAdarSheni1 => "יום א של ראש חודש אדר שני",
                Chol::RoshChodeshAdarSheni2 => "יום ב של ראש חודש אדר שני",
                Chol::RoshChodeshNissan => "ראש חדש ניסן",
                Chol::RoshChodeshIyar1 => "יום א של ראש חודש אייר",
                Chol::RoshChodeshIyar2 => "יום ב של ראש חודש אייר",
                Chol::RoshChodeshSivan => "ראש חדש סיון",
                Chol::RoshChodeshTammuz1 => "יום א של ראש חודש תמוז",
                Chol::RoshChodeshTammuz2 => "יום ב של ראש חודש תמוז",
                Chol::RoshChodeshAv => "ראש חודש אב",
                Chol::RoshChodeshElul1 => "יום א של ראש חודש אלול",
                Chol::RoshChodeshElul2 => "יום ב של ראש חודש אלול",
                Chol::Chanukah1 => "יום א של חנוכה",
                Chol::Chanukah2 => "יום ב של חנוכה",
                Chol::Chanukah3 => "יום ג של חנוכה",
                Chol::Chanukah4 => "יום ד של חנוכה",
                Chol::Chanukah5 => "יום ה של חנוכה",
                Chol::Chanukah6 => "יום ו של חנוכה",
                Chol::Chanukah7 => "יום ז של חנוכה",
                Chol::Chanukah8 => "יום ח של חנוכה",
                Chol::TzomGedalia => "צום גדליה",
                Chol::TaanisEsther => "תענית אסתר",
                Chol::TenTeves => "י' טבת",
                Chol::Purim => "פורים",
                Chol::ShushanPurim => "שושן פורים",
                Chol::SeventeenTammuz => "שבעה עשר בתמוז",
                Chol::NineAv => "תשעה באב",
            },
            TorahReading::Shabbos(tr) => match tr {
                Parsha::Haazinu => "האזינו",
                Parsha::Vayelech => "וילך",
                Parsha::Bereishis => "בראשית",
                Parsha::Noach => "נח",
                Parsha::LechLecha => "לך לך",
                Parsha::Vayeira => "וירא",
                Parsha::ChayeiSara => "חיי שרה",
                Parsha::Toldos => "תולדות",
                Parsha::Vayetzei => "ויצא",
                Parsha::Vayishlach => "וישלח",
                Parsha::Vayeshev => "וישב",
                Parsha::Miketz => "מיקץ",
                Parsha::Vayigash => "ויגש",
                Parsha::Vayechi => "ויחי",
                Parsha::Shemos => "שמות",
                Parsha::Vaeira => "וארא",
                Parsha::Bo => "בא",
                Parsha::Beshalach => "בשלח",
                Parsha::Yisro => "יתרו",
                Parsha::Mishpatim => "משפטים",
                Parsha::Terumah => "תרומה",
                Parsha::Tetzaveh => "תצוה",
                Parsha::KiSisa => "כי תשא",
                Parsha::VayakhelPikudei => "ויקהל/פקודי",
                Parsha::Vayakhel => "ויקהל",
                Parsha::Pikudei => "פקודי",
                Parsha::Vayikra => "ויקרא",
                Parsha::Tzav => "צו",
                Parsha::Shemini => "שמיני",
                Parsha::TazriyaMetzorah => "תזריע/מצורע",
                Parsha::Tazriya => "תזריע",
                Parsha::Metzorah => "מצורע",
                Parsha::AchareiMosKedoshim => "אחרי מות/קדושים",
                Parsha::AchareiMos => "אחרי מות",
                Parsha::Kedoshim => "קדושים",
                Parsha::Emor => "אמור",
                Parsha::BeharBechukosai => "בהר/בחוקותי",
                Parsha::Behar => "בהר",
                Parsha::Bechukosai => "בחוקותי",
                Parsha::Bamidbar => "במדבר",
                Parsha::Naso => "נשא",
                Parsha::Behaaloscha => "בהעלותך",
                Parsha::Shlach => "שלח",
                Parsha::Korach => "קרח",
                Parsha::ChukasBalak => "חקת/בלק",
                Parsha::Chukas => "חקת",
                Parsha::Balak => "בלק",
                Parsha::Pinchas => "פינחס",
                Parsha::MatosMaasei => "מטות/מסעי",
                Parsha::Matos => "מטות",
                Parsha::Maasei => "מסעי",
                Parsha::Devarim => "דברים",
                Parsha::Vaeschanan => "ואתחנן",
                Parsha::Eikev => "עקב",
                Parsha::Reeh => "ראה",
                Parsha::Shoftim => "שופטים",
                Parsha::KiSeitzei => "כי תצא",
                Parsha::KiSavoh => "כי תבוא",
                Parsha::NitzavimVayelech => "ניצבים/וילך",
                Parsha::Nitzavim => "ניצבים",
            },
            TorahReading::SpecialParsha(tr) => match tr {
                SpecialParsha::Zachor => "פרשת זכור",
                SpecialParsha::HaChodesh => "פרשת החודש",
                SpecialParsha::Parah => "פרשת פרה",
                SpecialParsha::Shekalim => "פרשת שקלים",
            },
        },
    }
}

fn get_minor_holidays(year: &HebrewYear) -> SmallVec<[DayVal; 16]> {
    let mut holidays = smallvec![
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(9).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevYomKippur)
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevSukkos)
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevPesach)
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::PesachSheni),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(18).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::LagBaOmer),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(5).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevShavuos),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, NonZeroI8::new(29).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ErevRoshHashanah),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Shvat, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::FifteenShvat),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::FifteenAv),
        },
    ];

    if year.is_leap_year() {
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(14).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::PurimKattan),
        });
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(15).unwrap())
                .unwrap()
                .into(),
            name: Name::MinorDays(MinorDays::ShushanPurimKattan),
        });
    }

    holidays
}

//generated from https://play.golang.com/p/fCtYz6kNCBw
pub fn get_omer(year: &HebrewYear) -> [DayVal; 49] {
    let first_day_of_pesach: DateTime<Utc> = year
        .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(15).unwrap())
        .unwrap()
        .into();

    //generated by https://play.golang.org/p/G78vf0EJnCN
    [
        DayVal {
            day: first_day_of_pesach + Duration::days(1),
            name: Name::MinorDays(MinorDays::Omer1),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(2),
            name: Name::MinorDays(MinorDays::Omer2),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(3),
            name: Name::MinorDays(MinorDays::Omer3),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(4),
            name: Name::MinorDays(MinorDays::Omer4),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(5),
            name: Name::MinorDays(MinorDays::Omer5),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(6),
            name: Name::MinorDays(MinorDays::Omer6),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(7),
            name: Name::MinorDays(MinorDays::Omer7),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(8),
            name: Name::MinorDays(MinorDays::Omer8),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(9),
            name: Name::MinorDays(MinorDays::Omer9),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(10),
            name: Name::MinorDays(MinorDays::Omer10),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(11),
            name: Name::MinorDays(MinorDays::Omer11),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(12),
            name: Name::MinorDays(MinorDays::Omer12),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(13),
            name: Name::MinorDays(MinorDays::Omer13),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(14),
            name: Name::MinorDays(MinorDays::Omer14),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(15),
            name: Name::MinorDays(MinorDays::Omer15),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(16),
            name: Name::MinorDays(MinorDays::Omer16),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(17),
            name: Name::MinorDays(MinorDays::Omer17),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(18),
            name: Name::MinorDays(MinorDays::Omer18),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(19),
            name: Name::MinorDays(MinorDays::Omer19),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(20),
            name: Name::MinorDays(MinorDays::Omer20),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(21),
            name: Name::MinorDays(MinorDays::Omer21),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(22),
            name: Name::MinorDays(MinorDays::Omer22),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(23),
            name: Name::MinorDays(MinorDays::Omer23),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(24),
            name: Name::MinorDays(MinorDays::Omer24),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(25),
            name: Name::MinorDays(MinorDays::Omer25),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(26),
            name: Name::MinorDays(MinorDays::Omer26),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(27),
            name: Name::MinorDays(MinorDays::Omer27),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(28),
            name: Name::MinorDays(MinorDays::Omer28),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(29),
            name: Name::MinorDays(MinorDays::Omer29),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(30),
            name: Name::MinorDays(MinorDays::Omer30),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(31),
            name: Name::MinorDays(MinorDays::Omer31),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(32),
            name: Name::MinorDays(MinorDays::Omer32),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(33),
            name: Name::MinorDays(MinorDays::Omer33),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(34),
            name: Name::MinorDays(MinorDays::Omer34),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(35),
            name: Name::MinorDays(MinorDays::Omer35),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(36),
            name: Name::MinorDays(MinorDays::Omer36),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(37),
            name: Name::MinorDays(MinorDays::Omer37),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(38),
            name: Name::MinorDays(MinorDays::Omer38),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(39),
            name: Name::MinorDays(MinorDays::Omer39),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(40),
            name: Name::MinorDays(MinorDays::Omer40),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(41),
            name: Name::MinorDays(MinorDays::Omer41),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(42),
            name: Name::MinorDays(MinorDays::Omer42),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(43),
            name: Name::MinorDays(MinorDays::Omer43),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(44),
            name: Name::MinorDays(MinorDays::Omer44),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(45),
            name: Name::MinorDays(MinorDays::Omer45),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(46),
            name: Name::MinorDays(MinorDays::Omer46),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(47),
            name: Name::MinorDays(MinorDays::Omer47),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(48),
            name: Name::MinorDays(MinorDays::Omer48),
        },
        DayVal {
            day: first_day_of_pesach + Duration::days(49),
            name: Name::MinorDays(MinorDays::Omer49),
        },
    ]
}
>>>>>>> 5f4c91114052d99937872bb660352ea1d38bb83d
