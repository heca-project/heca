use std::cmp::Ordering;

use crate::convert::HebrewDate;
use crate::types::HebrewMonth;
use std::borrow::Cow;

#[derive(Debug, Eq)]
pub struct SpecialDay {
    day: HebrewDate,
    name: Cow<'static, str>,
}

impl SpecialDay {
    #[inline]
    pub fn day(&self) -> HebrewDate {
        self.day
    }

    #[inline]
    pub fn name(&self) -> &str {
        &(self.name)
    }
}

impl PartialOrd for SpecialDay {
    #[inline]
    fn partial_cmp(&self, other: &SpecialDay) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpecialDay {
    #[inline]
    fn cmp(&self, other: &SpecialDay) -> Ordering {
        self.day.cmp(&other.day)
    }
}
impl PartialEq for SpecialDay {
    #[inline]
    fn eq(&self, other: &SpecialDay) -> bool {
        self.day == other.day
    }
}

#[inline]
pub fn get_yt_list(year: u64) -> Vec<SpecialDay> {
    vec![
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 1).unwrap(),
            name: "Rosh Hashana 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 2).unwrap(),
            name: "Rosh Hashana 2".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 10).unwrap(),
            name: "Yom Kippur".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 15).unwrap(),
            name: "Sukkos 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 16).unwrap(),
            name: "Sukkos 2".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 17).unwrap(),
            name: "Sukkos 3".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 18).unwrap(),
            name: "Sukkos 4".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 19).unwrap(),
            name: "Sukkos 5".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 20).unwrap(),
            name: "Sukkos 6".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 21).unwrap(),
            name: "Sukkos 7".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 22).unwrap(),
            name: "Shmini Atzeres".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 23).unwrap(),
            name: "Simchas Torah".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 15).unwrap(),
            name: "Pesach 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 16).unwrap(),
            name: "Pesach 2".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 17).unwrap(),
            name: "Pesach 3".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 18).unwrap(),
            name: "Pesach 4".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 19).unwrap(),
            name: "Pesach 5".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 20).unwrap(),
            name: "Pesach 6".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 21).unwrap(),
            name: "Pesach 7".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 22).unwrap(),
            name: "Pesach 8".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 6).unwrap(),
            name: "Shavuos 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 7).unwrap(),
            name: "Shavuos 2".into(),
        },
    ]
}
pub fn get_torah_reading_days_list(year: u64) -> Vec<SpecialDay> {
    let mut special_days = vec![
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 30).unwrap(),
            name: "Rosh Chodesh Cheshvan 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 1).unwrap(),
            name: "Rosh Chodesh Cheshvan 2".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 25).unwrap(),
            name: "Chanukah 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 26).unwrap(),
            name: "Chanukah 2".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 27).unwrap(),
            name: "Chanukah 3".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 28).unwrap(),
            name: "Chanukah 4".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 29).unwrap(),
            name: "Chanukah 5".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 1).unwrap(),
            name: "Rosh Chodesh Shvat".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 10).unwrap(),
            name: "10th Of Teves".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 1).unwrap(),
            name: "Rosh Chodesh Nissan".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 30).unwrap(),
            name: "Rosh Chodesh Iyar 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Iyar, 1).unwrap(),
            name: "Rosh Chodesh Iyar 2".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 1).unwrap(),
            name: "Rosh Chodesh Sivan".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 30).unwrap(),
            name: "Rosh Chodesh Tammuz 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 1).unwrap(),
            name: "Rosh Chodesh Tammuz 2".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 17).unwrap(),
            name: "17th of Tammuz".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 1).unwrap(),
            name: "Rosh Chodesh Av".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 9).unwrap(),
            name: "9th of Av".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 30).unwrap(),
            name: "Rosh Chodesh Elul 1".into(),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Elul, 1).unwrap(),
            name: "Rosh Chodesh Elul 2".into(),
        },
    ];
    let mut second_vector = {
        let mut in_vec: Vec<SpecialDay> = Vec::new();
        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 30) {
            in_vec.push(SpecialDay {
                day: first_day_rc,
                name: "Rosh Chodesh Kislev 1".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 1).unwrap(),
                name: "Rosh Chodesh Kislev 2".into(),
            });
        } else {
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 1).unwrap(),
                name: "Rosh Chodesh Kislev".into(),
            });
        }

        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
            in_vec.push(SpecialDay {
                day: first_day_rc,
                name: "Rosh Chodesh Teves 1".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: "Rosh Chodesh Teves 2".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30).unwrap(),
                name: "Chanukah 6".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: "Chanukah 7".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap(),
                name: "Chanukah 8".into(),
            });
        } else {
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: "Rosh Chodesh Teves".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: "Chanukah 6".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap(),
                name: "Chanukah 7".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 3).unwrap(),
                name: "Chanukah 8".into(),
            });
        }

        if let Ok(second_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Adar, 1) {
            //If this is a regular year
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 30).unwrap(),
                name: "Rosh Chodesh Adar 1".into(),
            });
            in_vec.push(SpecialDay {
                day: second_day_rc,
                name: "Rosh Chodesh Adar 2".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar, 14).unwrap(),
                name: "Purim".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar, 15).unwrap(),
                name: "Shushan Purim".into(),
            });
        } else {
            //If this is a leap year

            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 30).unwrap(),
                name: "Rosh Chodesh Adar I 1".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar1, 1).unwrap(),
                name: "Rosh Chodesh Adar I 2".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar1, 30).unwrap(),
                name: "Rosh Chodesh Adar II 1".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 1).unwrap(),
                name: "Rosh Chodesh Adar II 2".into(),
            });

            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 14).unwrap(),
                name: "Purim".into(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 15).unwrap(),
                name: "Shushan Purim".into(),
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
