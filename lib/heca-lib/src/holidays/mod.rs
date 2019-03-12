use smallvec::*;

use crate::convert::get_rosh_hashana;
use crate::convert::HebrewDate;
use crate::prelude::*;

#[inline]
pub(crate) fn get_yt_list(year: u64, location: Location) -> SmallVec<[TorahReadingDay; 256]> {
    let mut v1 = smallvec![
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 1),
            name: TorahReading::YomTov(YomTov::RoshHashana1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 2),
            name: TorahReading::YomTov(YomTov::RoshHashana2),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 10),
            name: TorahReading::YomTov(YomTov::YomKippur),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 15),
            name: TorahReading::YomTov(YomTov::Sukkos1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 16),
            name: TorahReading::YomTov(YomTov::Sukkos2),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 17),
            name: TorahReading::YomTov(YomTov::Sukkos3),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 18),
            name: TorahReading::YomTov(YomTov::Sukkos4),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 19),
            name: TorahReading::YomTov(YomTov::Sukkos5),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 20),
            name: TorahReading::YomTov(YomTov::Sukkos6),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 21),
            name: TorahReading::YomTov(YomTov::Sukkos7),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 22),
            name: TorahReading::YomTov(YomTov::ShminiAtzeres),
        },
    ];
    if location == Location::Chul {
        v1.push(TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 23),
            name: TorahReading::YomTov(YomTov::SimchasTorah),
        });
    }
    v1.extend_from_slice(&[
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 15),
            name: TorahReading::YomTov(YomTov::Pesach1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 16),
            name: TorahReading::YomTov(YomTov::Pesach2),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 17),
            name: TorahReading::YomTov(YomTov::Pesach3),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 18),
            name: TorahReading::YomTov(YomTov::Pesach4),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 19),
            name: TorahReading::YomTov(YomTov::Pesach5),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 20),
            name: TorahReading::YomTov(YomTov::Pesach6),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 21),
            name: TorahReading::YomTov(YomTov::Pesach7),
        },
    ]);

    if location == Location::Chul {
        v1.push(TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 22),
            name: TorahReading::YomTov(YomTov::Pesach8),
        });
    }
    v1.push(TorahReadingDay {
        day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Sivan, 6),
        name: TorahReading::YomTov(YomTov::Shavuos1),
    });

    if location == Location::Chul {
        v1.push(TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Sivan, 7),
            name: TorahReading::YomTov(YomTov::Shavuos2),
        });
    }

    v1.into()
}

pub(crate) fn get_chol_list(year: u64) -> SmallVec<[TorahReadingDay; 256]> {
    use crate::convert::get_rosh_hashana;
    let mut special_days = smallvec![
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 30),
            name: TorahReading::Chol(Chol::RoshChodeshCheshvan1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Cheshvan, 1),
            name: TorahReading::Chol(Chol::RoshChodeshCheshvan2),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 25),
            name: TorahReading::Chol(Chol::Chanukah1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 26),
            name: TorahReading::Chol(Chol::Chanukah2),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 27),
            name: TorahReading::Chol(Chol::Chanukah3),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 28),
            name: TorahReading::Chol(Chol::Chanukah4),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 29),
            name: TorahReading::Chol(Chol::Chanukah5),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Shvat, 1),
            name: TorahReading::Chol(Chol::RoshChodeshShvat),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 10),
            name: TorahReading::Chol(Chol::TenTeves),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 1),
            name: TorahReading::Chol(Chol::RoshChodeshNissan),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 30),
            name: TorahReading::Chol(Chol::RoshChodeshIyar1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Iyar, 1),
            name: TorahReading::Chol(Chol::RoshChodeshIyar2),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Sivan, 1),
            name: TorahReading::Chol(Chol::RoshChodeshSivan),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Sivan, 30),
            name: TorahReading::Chol(Chol::RoshChodeshTammuz1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tammuz, 1),
            name: TorahReading::Chol(Chol::RoshChodeshTammuz2),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Av, 1),
            name: TorahReading::Chol(Chol::RoshChodeshAv),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Av, 30),
            name: TorahReading::Chol(Chol::RoshChodeshElul1),
        },
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Elul, 1),
            name: TorahReading::Chol(Chol::RoshChodeshElul2),
        },
    ];
    let mut second_vector = {
        let mut in_vec: SmallVec<[TorahReadingDay; 256]> = SmallVec::new();
        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Cheshvan, 30) {
            in_vec.push(TorahReadingDay {
                day: first_day_rc,

                name: TorahReading::Chol(Chol::RoshChodeshKislev1),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 1),
                name: TorahReading::Chol(Chol::RoshChodeshKislev2),
            });
        } else {
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 1),
                name: TorahReading::Chol(Chol::RoshChodeshKislev),
            });
        }

        if let Ok(first_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Kislev, 30) {
            in_vec.push(TorahReadingDay {
                day: first_day_rc,
                name: TorahReading::Chol(Chol::RoshChodeshTeves1),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 1),
                name: TorahReading::Chol(Chol::RoshChodeshTeves2),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Kislev, 30),
                name: TorahReading::Chol(Chol::Chanukah6),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 1),
                name: TorahReading::Chol(Chol::Chanukah7),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 2),
                name: TorahReading::Chol(Chol::Chanukah8),
            });
        } else {
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 1),
                name: TorahReading::Chol(Chol::RoshChodeshTeves),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 1),
                name: TorahReading::Chol(Chol::Chanukah6),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 2),
                name: TorahReading::Chol(Chol::Chanukah7),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Teves, 3),
                name: TorahReading::Chol(Chol::Chanukah8),
            });
        }

        if let Ok(second_day_rc) = HebrewDate::from_ymd(year, HebrewMonth::Adar, 1) {
            //If this is a regular year
            in_vec.push(TorahReadingDay {
                day: second_day_rc,
                name: TorahReading::Chol(Chol::RoshChodeshAdar2),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar, 1),

                name: TorahReading::Chol(Chol::RoshChodeshAdar1),
            });

            // If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            in_vec.push(if get_rosh_hashana(year + 1).1 != Day::Thursday {
                TorahReadingDay {
                    day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar, 13),
                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            } else {
                TorahReadingDay {
                    day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar, 11),
                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar, 14),
                name: TorahReading::Chol(Chol::Purim),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar, 15),
                name: TorahReading::Chol(Chol::ShushanPurim),
            });
        } else {
            //If this is a leap year

            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Shvat, 30),
                name: TorahReading::Chol(Chol::RoshChodeshAdarRishon1),
            });
            in_vec.push(TorahReadingDay {
                name: TorahReading::Chol(Chol::RoshChodeshAdarRishon2),
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar1, 1),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar1, 30),
                name: TorahReading::Chol(Chol::RoshChodeshAdarSheni1),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar2, 1),
                name: TorahReading::Chol(Chol::RoshChodeshAdarSheni2),
            });
            // If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            in_vec.push(if get_rosh_hashana(year + 1).1 != Day::Thursday {
                TorahReadingDay {
                    day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar2, 13),
                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            } else {
                TorahReadingDay {
                    day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar2, 11),
                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar2, 14),
                name: TorahReading::Chol(Chol::Purim),
            });
            in_vec.push(TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar2, 15),
                name: TorahReading::Chol(Chol::ShushanPurim),
            });
        }

        in_vec
    };

    //17th of Tammuz is on Shabbos when the next Rosh Hashana starts on Monday (For example
    //5782/5783).
    let mut third_vector = if get_rosh_hashana(year + 1).1 == Day::Monday {
        vec![
            TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tammuz, 18),
                name: TorahReading::Chol(Chol::SeventeenTammuz),
            },
            TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Av, 10),
                name: TorahReading::Chol(Chol::NineAv),
            },
        ]
    } else {
        vec![
            TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tammuz, 17),
                name: TorahReading::Chol(Chol::SeventeenTammuz),
            },
            TorahReadingDay {
                day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Av, 9),
                name: TorahReading::Chol(Chol::NineAv),
            },
        ]
    };
    let tzom_gedalya = if get_rosh_hashana(year).1 == Day::Thursday {
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 4),
            name: TorahReading::Chol(Chol::TzomGedalya),
        }
    } else {
        TorahReadingDay {
            day: HebrewDate::from_ymd_unsafe(year, HebrewMonth::Tishrei, 3),
            name: TorahReading::Chol(Chol::TzomGedalya),
        }
    };

    special_days.extend_from_slice(&mut second_vector);
    special_days.push(tzom_gedalya);
    special_days.extend_from_slice(&mut third_vector);

    special_days
}

/// This is based on the Biyur Halacha to Orach Chaim 428:4:3
pub(crate) fn get_shabbos_list(year: u64, location: Location) -> SmallVec<[TorahReadingDay; 256]> {
    use crate::convert::get_rosh_hashana;
    let (rh_day, rh_dow) = get_rosh_hashana(year);
    let (rh_day_next, rh_dow_next) = get_rosh_hashana(year + 1);
    let year_len = rh_day_next - rh_day;

    //Tazriya/Metzorah Acharei-Mos/Kedoshim and Behar/Bechukosai are always split on a leap year
    //and connected on a regular year. The only exception is (in Israel) that Behar is split when the year is a non-leap year, is regular ordered and Rosh Hashana is on Thursday
    let (split_tazriya, split_acharei, mut split_behar) = if rh_day_next - rh_day > 365 {
        (true, true, true)
    } else {
        (false, false, false)
    };
    if location == Location::Israel {
        split_behar = split_behar || rh_day_next - rh_day == 354 && rh_dow == Day::Thursday;
    }

    //Vayakhel/Pekudei is split if the year is a leap year or if it's a full year and Rosh Hashana
    //starts on Thursday.
    let len_of_year = rh_day_next - rh_day;
    let split_vayakhel = len_of_year > 365 || (len_of_year == 355 && rh_dow == Day::Thursday);

    //Chukas Balak is split when the second day of Shavuos doesn't fall on Shabbos (The first day can't fall out on Shabbos, as then the next Rosh Hashana would start on Friday, which it can't). Shavuos falls on Shabbos (5783, for example) when the first day of the next Rosh Hashana is on a Shabbos.
    //Obviously, in Israel it's never split (as they don't have the second day of Shavuos).
    let split_chukas = location == Location::Israel || rh_dow_next != Day::Shabbos;
    //Mattos/Maasei is split only if it's a leap year and Rosh Hashana starts on a Thursday, and
    //the year is full, or empty.
    //In Israel, It's also split in a leap year which starts on a Monday and is full, or a
    //leap year starting on a Tuesday, and the year is an ordered year.
    //See this for more information: https://he.wikipedia.org/wiki/%D7%A4%D7%A8%D7%A9%D7%AA_%D7%9E%D7%98%D7%95%D7%AA

    let split_mattos = rh_dow == Day::Thursday && (len_of_year == 383 || len_of_year == 385)
        || (location == Location::Israel
            && (rh_dow == Day::Monday && year_len == 385
                || rh_dow == Day::Tuesday && year_len == 384));
    //
    //Nitzavim/Vayelech is split only if Rosh Hashana starts on a Monday or Tuesday
    let split_nitzavim = rh_dow == Day::Monday || rh_dow == Day::Tuesday;
    let split_nitzavim_next_year = rh_dow_next == Day::Monday || rh_dow_next == Day::Tuesday;
    let regular_shabbosim_list = get_shabbosim(year, &get_yt_list(year, location)).0;
    let mut parsha_list = if split_nitzavim {
        let mut v: SmallVec<[Parsha; 256]> = SmallVec::new();
        v.push(Parsha::Vayelach);
        v
    } else {
        SmallVec::new()
    };
    parsha_list.extend_from_slice(&HAAZINU_KI_SISA);

    if split_vayakhel {
        parsha_list.push(Parsha::Vayakhel);
        parsha_list.push(Parsha::Pikudei);
    } else {
        parsha_list.push(Parsha::VayakhelPikudei);
    }
    parsha_list.extend_from_slice(&VAYIKRA_SHMINI);
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
    parsha_list.extend_from_slice(&EMOR);
    if split_behar {
        parsha_list.push(Parsha::Behar);
        parsha_list.push(Parsha::Bechukosai);
    } else {
        parsha_list.push(Parsha::BeharBechukosai);
    }
    parsha_list.extend_from_slice(&BAMIDBAR_KORACH);
    if split_chukas {
        parsha_list.push(Parsha::Chukas);
        parsha_list.push(Parsha::Balak);
    } else {
        parsha_list.push(Parsha::ChukasBalak);
    }

    parsha_list.extend_from_slice(&PINCHAS);
    if split_mattos {
        parsha_list.push(Parsha::Matos);
        parsha_list.push(Parsha::Maasei);
    } else {
        parsha_list.push(Parsha::MatosMaasei);
    }

    parsha_list.extend_from_slice(&DEVARIM_KISAVO);
    if split_nitzavim_next_year {
        parsha_list.push(Parsha::Nitzavim);
    } else {
        parsha_list.push(Parsha::NitzavimVayelech);
    }

    //Every Shabbos should have a Parsha, and every Parsha should have a Shabbos
    assert_eq!(parsha_list.len(), regular_shabbosim_list.len());
    let return_val = regular_shabbosim_list
        .iter()
        .enumerate()
        .map(|(i, &v)| TorahReadingDay {
            name: TorahReading::Shabbos(parsha_list[i].clone()),
            day: v,
        })
        .collect::<SmallVec<[TorahReadingDay; 256]>>();
    return_val
}

pub(crate) fn get_special_parsha_list(year: u64) -> SmallVec<[TorahReadingDay; 256]> {
    let rh_day = get_rosh_hashana(year).0;
    let (rh_day_next, rh_dow_next) = get_rosh_hashana(year + 1);
    let len_of_year = rh_day_next - rh_day;

    let shekalim = TorahReadingDay {
        //Parshas Shekalim is the Shabbos before, or the Shabbos of the second day of Rosh Chodesh Adar (or the second day of Rosh Chodesh Adar Beis).
        // The first day of Rosh Chodesh Adar II can never be on Shabbos, as Purim would then
        // be on Sunday, and then the next Rosh Hashana would start on a Wednesday, breaking Lo
        // Adu Rosh.
        //
        //If Rosh Chodesh Adar (Beis) is on Shabbos (like in the year 5805), the next Rosh Hashana starts on a Tuesday
        day: if rh_dow_next == Day::Tuesday {
            if len_of_year < 360 {
                // This is a regular year
                HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar, 1)
            } else {
                // This is a leap year
                HebrewDate::from_ymd_unsafe(year, HebrewMonth::Adar2, 1)
            }
        } else {
            let month = if len_of_year < 360 {
                HebrewMonth::Shvat
            } else {
                HebrewMonth::Adar1
            };
            // This is a regular year
            HebrewDate::from_ymd_unsafe(
                year,
                month,
                match rh_dow_next {
                    Day::Monday => 25,
                    Day::Thursday => 29,
                    Day::Shabbos => 27,
                    _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
                },
            )
        },
        name: TorahReading::SpecialParsha(SpecialParsha::Shekalim),
    };
    let zachor = TorahReadingDay {
        //Parshas Zachor is read on the Shabbos before Purim.
        day: {
            let month = if len_of_year < 360 {
                HebrewMonth::Adar
            } else {
                HebrewMonth::Adar2
            };
            let day = match rh_dow_next {
                Day::Monday => 9,    //For example, 5782
                Day::Tuesday => 8,   //For example, 5781
                Day::Thursday => 13, // For example, 5784
                Day::Shabbos => 11,  //For example, 5780
                _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
            };
            HebrewDate::from_ymd_unsafe(year, month, day)
        },
        name: TorahReading::SpecialParsha(SpecialParsha::Zachor),
    };
    let parah = TorahReadingDay {
        //Parshas Parah is read on the Shabbos before Hachodesh.
        day: {
            let month = if len_of_year < 360 {
                HebrewMonth::Adar
            } else {
                HebrewMonth::Adar2
            };
            let day = match rh_dow_next {
                Day::Monday => 23,   //For example, 5782
                Day::Tuesday => 22,  //For example, 5781
                Day::Thursday => 20, // For example, 5784
                Day::Shabbos => 18,  //For example, 5780
                _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
            };
            HebrewDate::from_ymd_unsafe(year, month, day)
        },
        name: TorahReading::SpecialParsha(SpecialParsha::Parah),
    };
    let hachodesh = TorahReadingDay {
        //Parshas Hachodesh is read on the Shabbos before Rosh Chodesh Nissan, or on Rosh Chodesh
        //Nissan itself.
        day: {
            if rh_dow_next == Day::Monday {
                //Hachodesh is read on Rosh Chodesh Nissan itself
                HebrewDate::from_ymd_unsafe(year, HebrewMonth::Nissan, 1)
            } else {
                let month = if len_of_year < 360 {
                    HebrewMonth::Adar
                } else {
                    HebrewMonth::Adar2
                };
                let day = match rh_dow_next {
                    Day::Tuesday => 29,  //For example, 5781
                    Day::Thursday => 27, // For example, 5784
                    Day::Shabbos => 25,  //For example, 5780
                    _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
                };
                HebrewDate::from_ymd_unsafe(year, month, day)
            }
        },
        name: TorahReading::SpecialParsha(SpecialParsha::Hachodesh),
    };

    smallvec![shekalim, zachor, parah, hachodesh]
}

pub(crate) fn get_shabbosim(
    year: u64,
    ignore_dates: &[TorahReadingDay],
) -> (Vec<HebrewDate>, Vec<HebrewDate>) {
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
    Parsha::Vayeshev,
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
mod test {
    use crate::holidays::*;
    use chrono::prelude::*;
    #[test]
    fn fasts_should_never_start_on_friday_night() {
        for i in 3764..9999 {
            for day in get_chol_list(i).iter() {
                if day.name == TorahReading::Chol(Chol::TzomGedalya)
                    || day.name == TorahReading::Chol(Chol::TenTeves)
                    || day.name == TorahReading::Chol(Chol::SeventeenTammuz)
                    || day.name == TorahReading::Chol(Chol::NineAv)
                {
                    assert_ne!(day.day.to_gregorian().weekday(), Weekday::Fri);
                }
                //Taanis Esther can never be on a Friday night or on a Thursday night
                if day.name == TorahReading::Chol(Chol::TaanisEsther) {
                    assert_ne!(day.day.to_gregorian().weekday(), Weekday::Fri);
                    assert_ne!(day.day.to_gregorian().weekday(), Weekday::Thu);
                }
            }
        }
    }
    #[test]
    fn check_shekalim_on_shabbos_mevorchim_or_rosh_chodesh() {
        use chrono::Duration;
        for loc in [Location::Chul, Location::Israel].into_iter() {
            for i in 5764..9999 {
                let date = if let Ok(date) = HebrewDate::from_ymd(i, HebrewMonth::Adar, 1) {
                    date
                } else {
                    HebrewDate::from_ymd(i, HebrewMonth::Adar2, 1).unwrap()
                }
                .to_gregorian();
                assert_eq!(
                    get_shabbos_list(i, *loc)
                        .iter()
                        .filter(|x| x.name == TorahReading::SpecialParsha(SpecialParsha::Shekalim))
                        .filter(|x| x.day.to_gregorian() - date > Duration::days(7))
                        .count(),
                    0
                );
            }
        }
    }
    #[test]
    fn check_hachodesh_on_shabbos_mevorchim_or_rosh_chodesh() {
        use chrono::Duration;
        for loc in [Location::Chul, Location::Israel].into_iter() {
            for i in 5764..9999 {
                let date = HebrewDate::from_ymd(i, HebrewMonth::Nissan, 1)
                    .unwrap()
                    .to_gregorian();
                assert_eq!(
                    get_shabbos_list(i, *loc)
                        .iter()
                        .filter(|x| x.name == TorahReading::SpecialParsha(SpecialParsha::Hachodesh))
                        .filter(|x| x.day.to_gregorian() - date > Duration::days(7))
                        .count(),
                    0
                );
            }
        }
    }

    #[test]
    fn check_zachor_on_shabbos_before_purim() {
        use chrono::Duration;
        for loc in [Location::Chul, Location::Israel].into_iter() {
            for i in 5764..9999 {
                let date = if let Ok(date) = HebrewDate::from_ymd(i, HebrewMonth::Adar, 14) {
                    date
                } else {
                    HebrewDate::from_ymd(i, HebrewMonth::Adar2, 14).unwrap()
                }
                .to_gregorian();
                assert_eq!(
                    get_shabbos_list(i, *loc)
                        .iter()
                        .filter(|x| x.name == TorahReading::SpecialParsha(SpecialParsha::Zachor))
                        .filter(|x| x.day.to_gregorian() - date > Duration::days(7))
                        .count(),
                    0
                );
            }
        }
    }
    #[test]
    fn check_all_shabbosim_and_torah_readings_are_on_shabbos() {
        for i in 5764..9999 {
            assert_eq!(
                get_shabbos_list(i, Location::Chul)
                    .iter()
                    .filter(|&x| (*x).day.to_gregorian().weekday() != Weekday::Fri)
                    .count(),
                0
            );
            assert_eq!(
                get_shabbos_list(i, Location::Israel)
                    .iter()
                    .filter(|&x| (*x).day.to_gregorian().weekday() != Weekday::Fri)
                    .count(),
                0
            );
        }
    }

    #[test]
    fn check_fns_work_without_panic() {
        for i in 5764..9999 {
            println!("{}", i);
            get_yt_list(i, Location::Chul);
            get_yt_list(i, Location::Israel);
            get_chol_list(i);
        }
    }

    #[test]
    fn get_shabbosim_fall_on_shabbos() {
        for i in 3764..9999 {
            get_shabbosim(i, &[])
                .0
                .iter()
                //Shabbos starts on _Friday_ night
                .for_each(|x| assert_eq!(x.to_gregorian().weekday(), Weekday::Fri));
            get_shabbosim(i, &[])
                .1
                .iter()
                //Shabbos starts on _Friday_ night
                .for_each(|x| assert_eq!(x.to_gregorian().weekday(), Weekday::Fri));
        }
    }
}