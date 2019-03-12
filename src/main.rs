use chrono::prelude::*;
use clap::App;
use heca_lib::prelude::*;
use heca_lib::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
//use cpuprofiler::PROFILER;

mod args;
use crate::args::types;
use crate::args::types::*;

fn main() {
//    PROFILER.lock().unwrap().start("./my-prof.profile");

    use args;
    let args = args::build_args();
    let res = match args.command {
        Command::List(ref sub_args) => sub_args.run(&args),
        _ => panic!("not implemented"),
    };

    match args.output_type {
        OutputType::Regular | OutputType::Pretty => res.print(args),
        OutputType::JSON => println!("{}", serde_json::to_string(&res).unwrap()),
    }

    //PROFILER.lock().unwrap().stop();
}

trait Runnable<T: Printable> {
    fn run(&self, args: &MainArgs) -> T;
}

trait Printable {
    fn print(self, args: MainArgs);
}

impl Runnable<ListReturn> for ListArgs {
    fn run(&self, args: &MainArgs) -> ListReturn {
        match self.year {
            YearType::Hebrew(year) => {
                let list = (0 as u32..(self.amnt_years as u32))
                    .into_par_iter()
                    .map(|x| {
                        let year = HebrewYear::new(x as u64 + year).unwrap();
                        let mut days = year
                            .get_holidays(self.location, &self.events)
                            .iter()
                            .map(|x| DayVal {
                                day: x.day().to_gregorian(),
                                name: Either::Left(x.name()),
                            })
                            .collect::<Vec<DayVal>>();
                        days.extend(get_minor_holidays(year));
                        days
                    })
                    .collect::<Vec<Vec<DayVal>>>();
                let mut vec_dayval: Vec<DayVal> = Vec::new();
                list.iter().for_each(|x| {
                    vec_dayval.extend(x.iter().cloned());
                });
                ListReturn { list: vec_dayval }
            }
            YearType::Gregorian(year) => {
                let that_year = (year + 3760 - 1);
                let last_year = (self.amnt_years + that_year);

                let list = (0 as u32..(self.amnt_years as u32) + 2)
                    .into_par_iter()
                    .map(|x| {
                        HebrewYear::new(x as u64 + that_year)
                            .unwrap()
                            .get_holidays(self.location, &self.events)
                            .iter()
                            .map(|x| DayVal {
                                day: x.day().to_gregorian(),
                                name: Either::Left(x.name()),
                            })
                            .filter(|x| x.clone().day > Utc.ymd(year as i32, 1, 1).and_hms(0, 0, 0))
                            .filter(|x| {
                                x.day
                                    < Utc
                                        .ymd((year + self.amnt_years) as i32, 1, 1)
                                        .and_hms(0, 0, 0)
                            })
                            .collect::<Vec<DayVal>>()
                    })
                    .collect::<Vec<Vec<DayVal>>>();
                let mut vec_dayval: Vec<DayVal> = Vec::new();
                list.iter().for_each(|x| {
                    vec_dayval.extend(x.iter().cloned());
                });
                ListReturn { list: vec_dayval }
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct ListReturn {
    list: Vec<DayVal>,
}

impl Printable for ListReturn {
    fn print(self, args: MainArgs) {
        use chrono::Datelike;
        use std::io::stdout;
        use std::io::BufWriter;
        use std::io::Write;

        let stdout = stdout();
        let mut lock = BufWriter::with_capacity(100_000, stdout.lock());
        self.list
            .iter()
            .map(|x| {
                let ret = x.day;
                (ret.year(), ret.month(), ret.day(), x.name.clone())
            })
            .for_each(|(year, month, day, name)| {
                let mut year_arr = [b'\0'; 16];
                let mut month_arr = [b'\0'; 2];
                let mut day_arr = [b'\0'; 2];
                let count_y = itoa::write(&mut year_arr[..], year).unwrap();
                let count_m = itoa::write(&mut month_arr[..], month).unwrap();
                let count_d = itoa::write(&mut day_arr[..], day).unwrap();
                lock.write(&year_arr[..count_y as usize]).unwrap();
                lock.write(b"/").unwrap();
                lock.write(&month_arr[..count_m as usize]).unwrap();
                lock.write(b"/").unwrap();
                lock.write(&day_arr[..count_d as usize]).unwrap();
                lock.write(b" ").unwrap();
                match name {
                    Either::Left(name) => {
                        lock.write(print(name, &args.language).as_bytes()).unwrap()
                    }
                    Either::Right(s) => lock.write(s.as_bytes()).unwrap(),
                };
                lock.write(b"\n").unwrap();
            });
    }
}

fn print(tr: TorahReading, language: &types::Language) -> &'static str {
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
                Parsha::Vayetzei => "Vayetzei",
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
                Parsha::Terumah => "Tetzaveh",
                Parsha::Tetzaveh => "Tetzaveh",
                Parsha::KiSisa => "Ki Sisa",
                Parsha::VayakhelPikudei => "Vayakhel/Pikudei",
                Parsha::Vayakhel => "Vayekhel",
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
        Language::Hebrew => "",
    }
}

fn get_minor_holidays(year: HebrewYear) -> Vec<DayVal> {
    let mut holidays = vec![
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, 9)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Erev Yom Kippur".into()),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, 14)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Erev Sukkos".into()),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, 14)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Erev Pesach".into()),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, 14)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Pesach Sheni".into()),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, 18)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Lag BaOmer".into()),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, 5)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Erev Shavuos".into()),
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, 29)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Erev Rosh Hashana".into()),
        },
    ];

    if year.is_leap_year() {
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, 14)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Purim Kattan".into()),
        });
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, 15)
                .unwrap()
                .to_gregorian(),
            name: Either::Right("Shushan Purim Kattan".into()),
        });
    }

    holidays
}

#[derive(Clone, Debug, Serialize)]
struct DayVal {
    day: chrono::DateTime<Utc>,
    name: Either<TorahReading, Cow<'static, str>>,
}

#[derive(Copy, Clone, Debug, Serialize)]
enum Either<L, R> {
    Right(R),
    Left(L),
}
