use std::cmp::Ordering;

use crate::convert::HebrewDate;
use crate::types::Day;
use crate::types::HebrewMonth;
use chrono::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Eq, Clone)]
pub struct SpecialDay {
    pub(crate) day: HebrewDate,
    pub(crate) name: TorahReading,
}

impl SpecialDay {
    #[inline]
    pub fn day(&self) -> HebrewDate {
        self.day
    }

    #[inline]
    pub fn name(&self) -> TorahReading {
        self.name.clone()
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TorahReading {
    YomTov(YomTov),
    Chol(Chol),
    Shabbos(Parsha),
}
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SpecialParsha {
    Shekalim,
    Zachor,
    Parah,
    Hachodesh,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum YomTov {
    RoshHashana1,
    RoshHashana2,
    YomKippur,
    Sukkos1,
    Sukkos2,
    Sukkos3,
    Sukkos4,
    Sukkos5,
    Sukkos6,
    Sukkos7,
    ShminiAtzeres,
    SimchasTorah,
    Pesach1,
    Pesach2,
    Pesach3,
    Pesach4,
    Pesach5,
    Pesach6,
    Pesach7,
    Pesach8,
    Shavuos1,
    Shavuos2,
}

#[inline]
pub(crate) fn get_yt_list(year: u64) -> Cow<'static, [SpecialDay]> {
    vec![
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 1).unwrap(),
            name: TorahReading::YomTov(YomTov::RoshHashana1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 2).unwrap(),
            name: TorahReading::YomTov(YomTov::RoshHashana2),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 10).unwrap(),
            name: TorahReading::YomTov(YomTov::YomKippur),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 15).unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 16).unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos2),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 17).unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos3),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 18).unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos4),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 19).unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos5),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 20).unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos6),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 21).unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos7),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 22).unwrap(),
            name: TorahReading::YomTov(YomTov::ShminiAtzeres),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 23).unwrap(),
            name: TorahReading::YomTov(YomTov::SimchasTorah),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 15).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 16).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach2),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 17).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach3),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 18).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach4),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 19).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach5),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 20).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach6),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 21).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach7),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 22).unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach8),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 6).unwrap(),
            name: TorahReading::YomTov(YomTov::Shavuos1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 7).unwrap(),
            name: TorahReading::YomTov(YomTov::Shavuos2),
        },
    ]
    .into()
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Chol {
    TzomGedalya,
    RoshChodeshCheshvan1,
    RoshChodeshCheshvan2,
    Chanukah1,
    Chanukah2,
    Chanukah3,
    Chanukah4,
    Chanukah5,
    Chanukah6,
    Chanukah7,
    Chanukah8,
    RoshChodeshShvat,
    TenTeves,
    RoshChodeshNissan,
    RoshChodeshIyar1,
    RoshChodeshIyar2,
    RoshChodeshSivan,
    RoshChodeshTammuz1,
    RoshChodeshTammuz2,
    RoshChodeshAv,
    RoshChodeshElul1,
    RoshChodeshElul2,
    RoshChodeshKislev1,
    RoshChodeshKislev2,
    RoshChodeshKislev,
    RoshChodeshTeves1,
    RoshChodeshTeves2,
    RoshChodeshTeves,
    RoshChodeshAdar1,
    RoshChodeshAdar2,
    Purim,
    ShushanPurim,
    RoshChodeshAdarRishon1,
    RoshChodeshAdarRishon2,
    RoshChodeshAdarSheni1,
    RoshChodeshAdarSheni2,
    SeventeenTammuz,
    NineAv,
}
pub(crate) fn get_torah_reading_days_list(year: u64) -> Cow<'static, [SpecialDay]> {
    let mut special_days = vec![
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tishrei, 30).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshCheshvan1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshCheshvan2),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 25).unwrap(),
            name: TorahReading::Chol(Chol::Chanukah1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 26).unwrap(),
            name: TorahReading::Chol(Chol::Chanukah2),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 27).unwrap(),
            name: TorahReading::Chol(Chol::Chanukah3),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 28).unwrap(),
            name: TorahReading::Chol(Chol::Chanukah4),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 29).unwrap(),
            name: TorahReading::Chol(Chol::Chanukah5),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshShvat),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 10).unwrap(),
            name: TorahReading::Chol(Chol::TenTeves),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshNissan),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Nissan, 30).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshIyar1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Iyar, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshIyar2),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshSivan),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Sivan, 30).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshTammuz1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshTammuz2),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshAv),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Av, 30).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshElul1),
        },
        SpecialDay {
            day: HebrewDate::from_ymd(year, HebrewMonth::Elul, 1).unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshElul2),
        },
    ];
    let mut second_vector = {
        let mut in_vec: Vec<SpecialDay> = Vec::new();
        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 30) {
            in_vec.push(SpecialDay {
                day: first_day_rc,

                name: TorahReading::Chol(Chol::RoshChodeshKislev1),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 1).unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshKislev2),
            });
        } else {
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 1).unwrap(),

                name: TorahReading::Chol(Chol::RoshChodeshKislev),
            });
        }

        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
            in_vec.push(SpecialDay {
                day: first_day_rc,
                name: TorahReading::Chol(Chol::RoshChodeshTeves1),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshTeves2),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30).unwrap(),
                name: TorahReading::Chol(Chol::Chanukah6),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: TorahReading::Chol(Chol::Chanukah7),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap(),
                name: TorahReading::Chol(Chol::Chanukah8),
            });
        } else {
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshTeves),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 1).unwrap(),
                name: TorahReading::Chol(Chol::Chanukah6),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 2).unwrap(),
                name: TorahReading::Chol(Chol::Chanukah7),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Teves, 3).unwrap(),
                name: TorahReading::Chol(Chol::Chanukah8),
            });
        }

        if let Ok(second_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Adar, 1) {
            //If this is a regular year
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 30).unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdar1),
            });
            in_vec.push(SpecialDay {
                day: second_day_rc,

                name: TorahReading::Chol(Chol::RoshChodeshAdar2),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar, 14).unwrap(),
                name: TorahReading::Chol(Chol::Purim),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar, 15).unwrap(),
                name: TorahReading::Chol(Chol::ShushanPurim),
            });
        } else {
            //If this is a leap year

            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Shvat, 30).unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdarRishon1),
            });
            in_vec.push(SpecialDay {
                name: TorahReading::Chol(Chol::RoshChodeshAdarRishon2),
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar1, 1).unwrap(),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar1, 30).unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdarSheni1),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 1).unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdarSheni2),
            });

            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 14).unwrap(),
                name: TorahReading::Chol(Chol::Purim),
            });
            in_vec.push(SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Adar2, 15).unwrap(),
                name: TorahReading::Chol(Chol::ShushanPurim),
            });
        }

        in_vec
    };

    let mut third_vector = if HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 17)
        .unwrap()
        .to_gregorian()
        .weekday()
        == Weekday::Sat
    {
        vec![
            SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 18).unwrap(),
                name: TorahReading::Chol(Chol::SeventeenTammuz),
            },
            SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Av, 10).unwrap(),
                name: TorahReading::Chol(Chol::NineAv),
            },
        ]
    } else {
        vec![
            SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Tammuz, 17).unwrap(),
                name: TorahReading::Chol(Chol::SeventeenTammuz),
            },
            SpecialDay {
                day: HebrewDate::from_ymd(year, HebrewMonth::Av, 9).unwrap(),
                name: TorahReading::Chol(Chol::NineAv),
            },
        ]
    };

    special_days.append(&mut second_vector);
    special_days.append(&mut third_vector);

    special_days.into()
}

/// This is based on the Biyur Halacha to Orach Chaim 428:4:3
pub(crate) fn get_torah_readings(year: u64) -> Cow<'static, [SpecialDay]> {
    use crate::convert::get_rosh_hashana;
    let (rh_day, rh_dow) = get_rosh_hashana(year);
    let (rh_day_next, rh_dow_next) = get_rosh_hashana(year + 1);

    //Tazriya/Metzorah Acharei-Mos/Kedoshim and Behar/Bechukosai are always split on a leap year
    //and connected on a regular year.
    let (split_tazriya, split_acharei, split_behar) = if rh_day_next - rh_day > 365 {
        (true, true, true)
    } else {
        (false, false, false)
    };
    //Vayakhel/Pekudei is split if the year is a leap year or if it's a full year and Rosh Hashana
    //starts on Thursday.
    let len_of_year = rh_day_next - rh_day;
    let split_vayakhel = len_of_year > 365 || (len_of_year == 355 && rh_dow == Day::Thursday);

    //Behar/Bechukosai is split only if Shavuos starts on Shabbos (Friday night).
    let split_behar = HebrewDate::from_ymd(year, HebrewMonth::Sivan, 6)
        .unwrap()
        .to_gregorian()
        .weekday()
        == Weekday::Fri
        || split_behar;
    //Chukas Balak is split when the second day of Shavuos doesn't fall on Shabbos (The first day can't fall out on Shabbos, as then the next Rosh Hashana would start on Friday, which it can't). Shavuos falls on Shabbos (5783, for example) when the first day of the next Rosh Hashana is on a Shabbos.
    let split_chukas = rh_dow_next != Day::Shabbos;
    //Mattos/Maasei is split only if it's a leap year and Rosh Hashana starts on a Thursday, and
    //the year is full, or empty.
    let split_mattos = rh_dow == Day::Thursday && (len_of_year == 383 || len_of_year == 385);
    //TODO: In Israel, It's also split in a leap year which starts on a Monday and is full, or a
    //leap year starting on a Tuesday, and the year is an ordered year.
    //See this for more information: https://he.wikipedia.org/wiki/%D7%A4%D7%A8%D7%A9%D7%AA_%D7%9E%D7%98%D7%95%D7%AA
    //
    //Nitzavim/Vayelech is split only if Rosh Hashana starts on a Monday or Tuesday
    let split_nitzavim = rh_dow == Day::Monday || rh_dow == Day::Tuesday;
    let split_nitzavim_next_year = rh_dow_next == Day::Monday || rh_dow_next == Day::Tuesday;
    let (regular_shabbosim_list, special_shabbos_list) = get_shabbosim(year, &get_yt_list(year));
    let mut parsha_list = HAAZINU_KI_SISA.to_vec();

    if split_nitzavim {
        parsha_list.push(Parsha::Vayelach);
    }
    if split_vayakhel {
        parsha_list.push(Parsha::Vayakhel);
        parsha_list.push(Parsha::Pikudei);
    } else {
        parsha_list.push(Parsha::VayakhelPikudei);
    }
    parsha_list.append(&mut VAYIKRA_SHMINI.to_vec());
    if split_tazriya {
        parsha_list.push(Parsha::Tazriya);
        parsha_list.push(Parsha::Metzorah);
    } else {
        parsha_list.push(Parsha::TazriyaMetzorah);
    }
    if split_acharei {
        parsha_list.push(Parsha::AchareiMos);
        parsha_list.push(Parsha::Kedoshim);
    } else {
        parsha_list.push(Parsha::AchareiMosKedoshim);
    }
    parsha_list.append(&mut EMOR.to_vec());
    if split_behar {
        parsha_list.push(Parsha::Behar);
        parsha_list.push(Parsha::Bechukosai);
    } else {
        parsha_list.push(Parsha::BeharBechukosai);
    }
    parsha_list.append(&mut BAMIDBAR_KORACH.to_vec());
    if split_chukas {
        parsha_list.push(Parsha::Chukas);
        parsha_list.push(Parsha::Balak);
    } else {
        parsha_list.push(Parsha::ChukasBalak);
    }

    parsha_list.append(&mut PINCHAS.to_vec());
    if split_mattos {
        parsha_list.push(Parsha::Matos);
        parsha_list.push(Parsha::Maasei);
    } else {
        parsha_list.push(Parsha::MatosMaasei);
    }

    parsha_list.append(&mut DEVARIM_KISAVO.to_vec());
    if split_nitzavim_next_year {
        parsha_list.push(Parsha::Nitzavim);
    } else {
        parsha_list.push(Parsha::NitzavimVayelech);
    }

    //Every Shabbos should have a Parsha, and every Parsha should have a Shabbos
    assert_eq!(parsha_list.len(), regular_shabbosim_list.len());
    let mut return_val = regular_shabbosim_list
        .iter()
        .enumerate()
        .map(|(i, &v)| SpecialDay {
            name: TorahReading::Shabbos(parsha_list[i].clone()),
            day: v,
        })
        .collect::<Vec<SpecialDay>>();
    return_val.append(
        &mut special_shabbos_list
            .iter()
            .map(|v| SpecialDay {
                name: TorahReading::Shabbos(Parsha::YomTov),
                day: *v,
            })
            .collect(),
    );
    return_val.into()
}

fn get_shabbosim(year: u64, ignore_dates: &[SpecialDay]) -> (Vec<HebrewDate>, Vec<HebrewDate>) {
    use crate::convert::get_rosh_hashana;
    let (day_of_rh, rosh_hashana_dow) = get_rosh_hashana(year);
    let (day_of_next_rh, _) = get_rosh_hashana(year + 1);
    let amnt_days_to_shabbos = Day::Shabbos as u64 - (rosh_hashana_dow as u64);
    let mut cur_day = day_of_rh + amnt_days_to_shabbos;
    let mut return_regular_shabbosim: Vec<HebrewDate> = Vec::new();
    let mut return_special_shabbosim: Vec<HebrewDate> = Vec::new();
    while cur_day < day_of_next_rh {
        let day = HebrewDate::get_hebrewdate_from_days_after_rh(year, cur_day, day_of_rh);
        if ignore_dates.iter().filter(|x| x.day == day).count() == 0 {
            return_regular_shabbosim.push(day);
        } else {
            return_special_shabbosim.push(day);
        }

        cur_day += 7;
    }
    (return_regular_shabbosim, return_special_shabbosim)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Parsha {
    Vayelach,
    Haazinu,
    Bereishis,
    Noach,
    LechLecha,
    Vayeira,
    ChayeiSara,
    Toldos,
    Vayetzei,
    Vayishlach,
    Veayeshev,
    Miketz,
    Vayigash,
    Vayechi,
    Shemos,
    Vaeira,
    Bo,
    Beshalach,
    Yisro,
    Mishpatim,
    Terumah,
    Tetzaveh,
    KiSisa,
    VayakhelPikudei,
    Vayakhel,
    Pikudei,
    Vayikra,
    Tzav,
    Shemini,
    TazriyaMetzorah,
    Tazriya,
    Metzorah,
    AchareiMosKedoshim,
    AchareiMos,
    Kedoshim,
    Emor,
    BeharBechukosai,
    Behar,
    Bechukosai,
    Bamidbar,
    Naso,
    Behaaloscha,
    Shlach,
    Korach,
    ChukasBalak,
    Chukas,
    Balak,
    Pinchas,
    MatosMaasei,
    Matos,
    Maasei,
    Devarim,
    Vaeschanan,
    Eikev,
    Reeh,
    Shoftim,
    KiSeitzei,
    KiSavoh,
    NitzavimVayelech,
    Nitzavim,

    YomTov,
}

const HAAZINU_KI_SISA: [Parsha; 22] = [
    Parsha::Haazinu,
    Parsha::Bereishis,
    Parsha::Noach,
    Parsha::LechLecha,
    Parsha::Vayeira,
    Parsha::ChayeiSara,
    Parsha::Toldos,
    Parsha::Vayetzei,
    Parsha::Vayishlach,
    Parsha::Veayeshev,
    Parsha::Miketz,
    Parsha::Vayigash,
    Parsha::Vayechi,
    Parsha::Shemos,
    Parsha::Vaeira,
    Parsha::Bo,
    Parsha::Beshalach,
    Parsha::Yisro,
    Parsha::Mishpatim,
    Parsha::Terumah,
    Parsha::Tetzaveh,
    Parsha::KiSisa,
];
const VAYIKRA_SHMINI: [Parsha; 3] = [Parsha::Vayikra, Parsha::Tzav, Parsha::Shemini];
const EMOR: [Parsha; 1] = [Parsha::Emor];
const BAMIDBAR_KORACH: [Parsha; 5] = [
    Parsha::Bamidbar,
    Parsha::Naso,
    Parsha::Behaaloscha,
    Parsha::Shlach,
    Parsha::Korach,
];
const PINCHAS: [Parsha; 1] = [Parsha::Pinchas];
const DEVARIM_KISAVO: [Parsha; 7] = [
    Parsha::Devarim,
    Parsha::Vaeschanan,
    Parsha::Eikev,
    Parsha::Reeh,
    Parsha::Shoftim,
    Parsha::KiSeitzei,
    Parsha::KiSavoh,
];

#[cfg(test)]
#[test]
fn check_fns_work_without_panic() {
    for i in 3764..9999 {
        println!("{}", i);
        get_yt_list(i);
        get_torah_reading_days_list(i);
        get_torah_readings(i);
    }
}

#[test]
fn get_shabbosim_fall_on_shabbos() {
    for i in 3764..9999 {
        get_shabbosim(i, &vec![])
            .0
            .iter()
            //Shabbos starts on _Friday_ night
            .for_each(|x| assert_eq!(x.to_gregorian().weekday(), Weekday::Fri));
        get_shabbosim(i, &vec![])
            .1
            .iter()
            //Shabbos starts on _Friday_ night
            .for_each(|x| assert_eq!(x.to_gregorian().weekday(), Weekday::Fri));
    }
}
