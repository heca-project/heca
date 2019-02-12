use std::cmp::Ordering;

use crate::convert::HebrewDate;
use crate::types::HebrewMonth;

#[derive(Debug, Eq)]
pub struct SpecialDay {
    day: HebrewDate,
    name: String,
}

impl SpecialDay {
    pub fn day(&self) -> HebrewDate {
        self.day
    }
    pub fn name(&self) -> &str {
        &(self.name)
    }
}

impl PartialOrd for SpecialDay {
    fn partial_cmp(&self, other: &SpecialDay) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpecialDay {
    fn cmp(&self, other: &SpecialDay) -> Ordering {
        self.day.cmp(&other.day)
    }
}
impl PartialEq for SpecialDay {
    fn eq(&self, other: &SpecialDay) -> bool {
        self.day == other.day
    }
}

pub fn get_yt_list(year: u64) -> Vec<SpecialDay> {
    vec![
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 1).unwrap(),
            name: String::from("Rosh Hashana 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 2).unwrap(),
            name: String::from("Rosh Hashana 2"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 10).unwrap(),
            name: String::from("Yom Kippur"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 15).unwrap(),
            name: String::from("Sukkos 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 16).unwrap(),
            name: String::from("Sukkos 2"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 17).unwrap(),
            name: String::from("Sukkos 3"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 18).unwrap(),
            name: String::from("Sukkos 4"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 19).unwrap(),
            name: String::from("Sukkos 5"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 20).unwrap(),
            name: String::from("Sukkos 6"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 21).unwrap(),
            name: String::from("Sukkos 7"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 22).unwrap(),
            name: String::from("Shmini Atzeres"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 23).unwrap(),
            name: String::from("Simchas Torah"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 15).unwrap(),
            name: String::from("Pesach 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 16).unwrap(),
            name: String::from("Pesach 2"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 17).unwrap(),
            name: String::from("Pesach 3"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 18).unwrap(),
            name: String::from("Pesach 4"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 19).unwrap(),
            name: String::from("Pesach 5"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 20).unwrap(),
            name: String::from("Pesach 6"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 21).unwrap(),
            name: String::from("Pesach 7"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 22).unwrap(),
            name: String::from("Pesach 8"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 6).unwrap(),
            name: String::from("Shavuos 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 7).unwrap(),
            name: String::from("Shavuos 2"),
        },
    ]
}
pub fn get_torah_reading_days_list(year: u64) -> Vec<SpecialDay> {
    let mut special_days = vec![
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 30).unwrap(),
            name: String::from("Rosh Chodesh Cheshvan 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 1).unwrap(),
            name: String::from("Rosh Chodesh Cheshvan 2"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 25).unwrap(),
            name: String::from("Chanukah 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 26).unwrap(),
            name: String::from("Chanukah 2"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 27).unwrap(),
            name: String::from("Chanukah 3"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 28).unwrap(),
            name: String::from("Chanukah 4"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 29).unwrap(),
            name: String::from("Chanukah 5"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 1).unwrap(),
            name: String::from("Rosh Chodesh Shvat"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 10).unwrap(),
            name: String::from("10th Of Teves"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 1).unwrap(),
            name: String::from("Rosh Chodesh Nissan"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 30).unwrap(),
            name: String::from("Rosh Chodesh Iyar 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Iyar, 1).unwrap(),
            name: String::from("Rosh Chodesh Iyar 2"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 1).unwrap(),
            name: String::from("Rosh Chodesh Sivan"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 30).unwrap(),
            name: String::from("Rosh Chodesh Tammuz 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 1).unwrap(),
            name: String::from("Rosh Chodesh Tammuz 2"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 17).unwrap(),
            name: String::from("17th of Tammuz"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 1).unwrap(),
            name: String::from("Rosh Chodesh Av"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 9).unwrap(),
            name: String::from("9th of Av"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 30).unwrap(),
            name: String::from("Rosh Chodesh Elul 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Elul, 1).unwrap(),
            name: String::from("Rosh Chodesh Elul 2"),
        },
    ];
    let mut second_vector = {
        let mut in_vec: Vec<SpecialDay> = Vec::new();
        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 30) {
            in_vec.push(SpecialDay {
                day: first_day_rc,
                name: String::from("Rosh Chodesh Kislev 1"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 1).unwrap(),
                name: String::from("Rosh Chodesh Kislev 2"),
            });
        } else {
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 1).unwrap(),
                name: String::from("Rosh Chodesh Kislev"),
            });
        }

        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
            in_vec.push(SpecialDay {
                day: first_day_rc,
                name: String::from("Rosh Chodesh Teves 1"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: String::from("Rosh Chodesh Teves 2"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30).unwrap(),
                name: String::from("Chanukah 6"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: String::from("Chanukah 7"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap(),
                name: String::from("Chanukah 8"),
            });
        } else {
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: String::from("Rosh Chodesh Teves"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: String::from("Chanukah 6"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap(),
                name: String::from("Chanukah 7"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 3).unwrap(),
                name: String::from("Chanukah 8"),
            });
        }

        if let Ok(second_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Adar, 1) {
            //If this is a regular year
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 30).unwrap(),
                name: String::from("Rosh Chodesh Adar 1"),
            });
            in_vec.push(SpecialDay {
                day: second_day_rc,
                name: String::from("Rosh Chodesh Adar 2"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar, 14).unwrap(),
                name: String::from("Purim"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar, 15).unwrap(),
                name: String::from("Shushan Purim"),
            });
        } else {
            //If this is a leap year

            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 30).unwrap(),
                name: String::from("Rosh Chodesh Adar I 1"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar1, 1).unwrap(),
                name: String::from("Rosh Chodesh Adar I 2"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar1, 30).unwrap(),
                name: String::from("Rosh Chodesh Adar II 1"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 1).unwrap(),
                name: String::from("Rosh Chodesh Adar II 2"),
            });

            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 14).unwrap(),
                name: String::from("Purim"),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 15).unwrap(),
                name: String::from("Shushan Purim"),
            });
        }

        in_vec
    };

    special_days.append(&mut second_vector);

    special_days
}

#[cfg(test)]
#[test]
fn check_fns_work_without_panic() {
    for i in 3764..9999 {
        println!("{}", i);
        get_yt_list(i);
        get_torah_reading_days_list(i);
    }
}
