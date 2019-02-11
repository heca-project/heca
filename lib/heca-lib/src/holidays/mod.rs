use crate::convert::HebrewDate;
use crate::types::HebrewMonth;

pub struct SpecialDay {
    day: HebrewDate,
    name: String,
}

fn get_special_days_list(year: u64) -> Vec<SpecialDay> {
    let mut special_days = vec![
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
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 30).unwrap(),
            name: String::from("Rosh Chodesh Cheshvan 1"),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 1).unwrap(),
            name: String::from("Rosh Chodesh Cheshvan 2"),
        },
    ];

    let mut second_vector = {
        let mut in_vec: Vec<SpecialDay> = Vec::new();
        let mut two_day_rc_kislev = false;
        in_vec.push(
            if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 30) {
                two_day_rc_kislev = true;
                SpecialDay {
                    day: first_day_rc,
                    name: String::from("Rosh Chodesh Kislev 1"),
                }
            } else {
                SpecialDay {
                    day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 1).unwrap(),
                    name: String::from("Rosh Chodesh Kislev"),
                }
            },
        );
        if two_day_rc_kislev {
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 2).unwrap(),
                name: String::from("Rosh Chodesh Kislev 2"),
            });
        }
        in_vec
    };

    special_days.append(&mut second_vector);
    special_days

    /*
         SpecialDay::Chanukah1 => Some(HebrewDate::from_ymd(year, HebrewMonth::Kislev, 25).unwrap()),
        SpecialDay::Chanukah2 => Some(HebrewDate::from_ymd(year, HebrewMonth::Kislev, 26).unwrap()),
        SpecialDay::Chanukah3 => Some(HebrewDate::from_ymd(year, HebrewMonth::Kislev, 27).unwrap()),
        SpecialDay::Chanukah4 => Some(HebrewDate::from_ymd(year, HebrewMonth::Kislev, 28).unwrap()),
        SpecialDay::Chanukah5 => Some(HebrewDate::from_ymd(year, HebrewMonth::Kislev, 29).unwrap()),

        SpecialDay::Chanukah6 => if let Ok(chan_7) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
            Some(HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30).unwrap())
        } else {
            Some(HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap())
        }
        SpecialDay::Chanukah7 => if let Ok(chan_7) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
            Some(HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap())
        } else {
            Some(HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap())
        }
        SpecialDay::Chanukah8 => if let Ok(chan_8) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
            Some(HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap())
        } else {
            Some(HebrewDate::from_ymd(year, HebrewMonth::Teves, 3).unwrap())
        }

        SpecialDay::RoshChodeshTeves1 => {
            if let Ok(thirty_kislev) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
                Some(thirty_kislev)
            } else {
                Some(HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap())
            }
        }
        SpecialDay::RoshChodeshKislev2 => {
            if let Ok(thirty_cheshvan) = HebrewDate::from_ymd(year, HebrewMonth::Teves, 1) {
                Some(thirty_cheshvan)
            } else {
                None
            }
        }



    }*/
}
