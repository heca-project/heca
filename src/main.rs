use chrono::prelude::*;
use chrono::Duration;
use either::*;
use heca_lib::prelude::*;
use heca_lib::*;
use rayon::prelude::*;
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;

use cpuprofiler::PROFILER;

mod args;
use crate::args::types;
use crate::args::types::*;

fn main() {
    //PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();

    use args;
    let args = args::build_args();
    let res: Box<Printable> = match args.command {
        Command::List(ref sub_args) => Box::new(sub_args.run(&args)),
        Command::Convert(ref sub_args) => Box::new(sub_args.run(&args)),
    };

    match args.output_type {
        OutputType::Regular | OutputType::Pretty => (&res).print(args),
        OutputType::JSON => (&res).print_json(),
    };

    //PROFILER.lock().unwrap().stop().unwrap();
}

trait Runnable<T: Printable> {
    fn run(&self, args: &MainArgs) -> T;
}

trait Printable {
    fn print(&self, args: MainArgs);
    fn print_json(&self);
}

impl Runnable<ConvertReturn> for ConvertArgs {
    fn run(&self, _args: &MainArgs) -> ConvertReturn {
        match self.date {
            ConvertType::Gregorian(date) => ConvertReturn {
                day: Either::Right([
                    HebrewDate::from_gregorian(date.and_hms(0, 0, 1)).unwrap(),
                    HebrewDate::from_gregorian(date.and_hms(23, 0, 1)).unwrap(),
                ]),
            },
            ConvertType::Hebrew(date) => ConvertReturn {
                day: Either::Left([
                    date.to_gregorian().into(),
                    (date.to_gregorian() + Duration::days(1)).into(),
                ]),
            },
        }
    }
}
impl Runnable<ListReturn> for ListArgs {
    fn run(&self, _args: &MainArgs) -> ListReturn {
        let mut main_events: Vec<TorahReadingType> = Vec::new();
        let mut custom_events: Vec<CustomHoliday> = Vec::new();
        for event in &self.events {
            match event {
                Left(event) => main_events.push(*event),
                Right(event) => custom_events.push(*event),
            };
        }
        let mut result = match self.year {
            YearType::Hebrew(year) => {
                let list = (0 as u32..(self.amnt_years as u32))
                    .into_par_iter()
                    .flat_map(|x| {
                        let mut ret = Vec::with_capacity(200);
                        let year = HebrewYear::new(x as u64 + year).unwrap();
                        ret.extend(
                            year.get_holidays(self.location, &main_events)
                                .into_iter()
                                .map(|x| DayVal {
                                    day: x.day().to_gregorian(),
                                    name: Name::TorahReading(x.name()),
                                }),
                        );
                        if custom_events.contains(&CustomHoliday::Omer) {
                            ret.extend(get_omer(&year));
                        }
                        if custom_events.contains(&CustomHoliday::Minor) {
                            ret.extend(get_minor_holidays(&year));
                        }
                        ret
                    })
                    .collect::<Vec<DayVal>>();
                ListReturn { list }
            }
            YearType::Gregorian(year) => {
                let that_year = year + 3760 - 1;
                let list = (0 as u32..(self.amnt_years as u32) + 2)
                    .into_par_iter()
                    .flat_map(|x| {
                        let mut ret = Vec::with_capacity(200);
                        let heb_year = HebrewYear::new(x as u64 + that_year).unwrap();
                        ret.extend(
                            heb_year
                                .get_holidays(self.location, &main_events)
                                .into_iter()
                                .map(|x| DayVal {
                                    day: x.day().to_gregorian(),
                                    name: Name::TorahReading(x.name()),
                                })
                                .into_iter(),
                        );
;
                        if custom_events.contains(&CustomHoliday::Omer) {
                            ret.extend(get_omer(&heb_year).into_iter());
                        }
                        if custom_events.contains(&CustomHoliday::Minor) {
                            ret.extend(get_minor_holidays(&heb_year).into_iter());
                        }
                        ret
                    })
                    .filter(|x| x.day > Utc.ymd(year as i32, 1, 1).and_hms(0, 0, 0))
                    .filter(|x| {
                        x.day
                            < Utc
                                .ymd((year + self.amnt_years) as i32, 1, 1)
                                .and_hms(0, 0, 0)
                    })
                    .collect::<Vec<DayVal>>();
                ListReturn { list }
            }
        };
        if !self.no_sort {
            result.list.par_sort_unstable_by(|a, b| a.day.cmp(&b.day));
        }
        result
    }
}
#[derive(Debug)]
struct ConvertReturn {
    pub day: Either<[chrono::DateTime<Utc>; 2], [HebrewDate; 2]>,
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
    fn print_json(&self) {
        match &self.day {
            Either::Right(r) => println!("{}", serde_json::to_string(&r).unwrap()),
            Either::Left(r) => println!("{}", serde_json::to_string(&r).unwrap()),
        };
    }
    fn print(&self, _args: MainArgs) {}
}
impl Printable for ListReturn {
    fn print_json(&self) {
        println!("{}", serde_json::to_string(&self).unwrap());
    }
    fn print(&self, args: MainArgs) {
        use chrono::Datelike;
        use std::io::stdout;
        use std::io::BufWriter;
        use std::io::Write;
        let stdout = stdout();
        let mut lock = BufWriter::with_capacity(100_000, stdout.lock());
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
            lock.write(&year_arr[..count_y as usize]).unwrap();
            lock.write(b"/").unwrap();
            lock.write(&month_arr[..count_m as usize]).unwrap();
            lock.write(b"/").unwrap();
            lock.write(&day_arr[..count_d as usize]).unwrap();
            lock.write(b" ").unwrap();
            match name {
                Name::TorahReading(name) => {
                    lock.write(print(name, &args.language).as_bytes()).unwrap()
                }
                Name::CustomName { json: _, printable } => {
                    lock.write(printable.as_bytes()).unwrap()
                }
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
pub fn get_omer(year: &HebrewYear) -> Vec<DayVal> {
    let mut dv: Vec<DayVal> = Vec::new();
    let first_day_of_pesach = year
        .get_hebrew_date(HebrewMonth::Nissan, 15)
        .unwrap()
        .to_gregorian();
    for i in 1..=49 {
        dv.push(DayVal {
            day: first_day_of_pesach + Duration::days(i),
            name: Name::CustomName {
                printable: format!(
                    "{}{} day of the Omer",
                    i,
                    if (i % 10) == 1 && i != 11 {
                        "st"
                    } else if (i % 10) == 2 && i != 12 {
                        "nd"
                    } else if (i % 10) == 3 && i != 13 {
                        "rd"
                    } else {
                        "th"
                    }
                )
                .into(),
                json: format!("Omer{}", i,).into(),
            },
        })
    }
    dv
}
fn get_minor_holidays(year: &HebrewYear) -> Vec<DayVal> {
    let mut holidays = vec![
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, 9)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Erev Yom Kippur".into(),
                json: "ErevYomKippur".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, 14)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Erev Sukkos".into(),
                json: "ErevSukkos".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, 14)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Erev Pesach".into(),
                json: "ErevPesach".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, 14)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Pesach Sheni".into(),
                json: "PesachSheni".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, 18)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Lag Baomer".into(),
                json: "LagBaomer".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, 5)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Erev Shavuos".into(),
                json: "ErevShavuos".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, 29)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Erev Rosh Hashana".into(),
                json: "ErevRoshHashanah".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Shvat, 15)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "15th of Shvat".into(),
                json: "Shvat15".into(),
            },
        },
        DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Av, 15)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "15th of Av".into(),
                json: "Av15".into(),
            },
        },
    ];

    if year.is_leap_year() {
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, 14)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Purim Kattan".into(),
                json: "PurimKattan".into(),
            },
        });
        holidays.push(DayVal {
            day: year
                .get_hebrew_date(HebrewMonth::Adar1, 15)
                .unwrap()
                .to_gregorian(),
            name: Name::CustomName {
                printable: "Shushan Purim Kattan".into(),
                json: "ShushanPurimKattan".into(),
            },
        });
    }

    holidays
}
