use chrono::prelude::*;
use clap::App;
use heca_lib::prelude::*;
use heca_lib::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

mod args;
use crate::args::types;
use crate::args::types::*;

fn main() {
    use args;
    let args = args::build_args();
    let res = match args.command {
        Command::List(ref args) => args.run(),
        _ => panic!("not implemented"),
    };

    match args.output_type {
        OutputType::Regular | OutputType::Pretty => res.print(args),
        OutputType::JSON => println!("{:}", serde_json::to_string(&res).unwrap()),
    }
}

trait Runnable<T: Printable> {
    fn run(&self) -> T;
}

trait Printable {
    fn print(self, args: MainArgs);
}

impl Runnable<ListReturn> for ListArgs {
    fn run(&self) -> ListReturn {
        match self.year {
            YearType::Hebrew(year) => {
                let list = (0 as u32..(self.amnt_years as u32))
                    .into_par_iter()
                    .map(|x| {
                        HebrewYear::new(x as u64 + year)
                            .unwrap()
                            .get_holidays(self.location, &self.events)
                            .iter()
                            .map(|x| DayVal {
                                day: x.day().to_gregorian(),
                                name: x.name(),
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
                                name: x.name(),
                            })
                            .filter(|&x| x.day > Utc.ymd(year as i32, 1, 1).and_hms(0, 0, 0))
                            .filter(|&x| {
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

#[derive(Debug, Serialize, Deserialize)]
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
                (ret.year(), ret.month(), ret.day(), x.name)
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
                //lock.write(print(TorahReading,self.Language)).unwrap();
                lock.write(b"\n").unwrap();
            });
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct DayVal {
    day: chrono::DateTime<Utc>,
    name: TorahReading,
}
