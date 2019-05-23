use smallvec::*;

use crate::prelude::*;
use crate::{HebrewDate, HebrewYear};
use std::num::NonZeroI8;

#[inline]
pub(crate) fn get_yt_list(
    year: HebrewYear,
    location: Location,
) -> SmallVec<[TorahReadingDay; 256]> {
    let mut v1 = smallvec![
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::RoshHashanah1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(2).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::RoshHashanah2),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(10).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::YomKippur),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(15).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(16).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos2),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(17).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos3),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(18).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos4),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(19).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos5),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(20).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos6),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(21).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Sukkos7),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(22).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::ShminiAtzeres),
        },
    ];
    if location == Location::Chul {
        v1.push(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(23).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::SimchasTorah),
        });
    }
    v1.extend_from_slice(&[
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(15).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(16).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach2),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(17).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach3),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(18).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach4),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(19).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach5),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(20).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach6),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(21).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach7),
        },
    ]);

    if location == Location::Chul {
        v1.push(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(22).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Pesach8),
        });
    }
    v1.push(TorahReadingDay {
        day: year
            .get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(6).unwrap())
            .unwrap(),
        name: TorahReading::YomTov(YomTov::Shavuos1),
    });

    if location == Location::Chul {
        v1.push(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(7).unwrap())
                .unwrap(),
            name: TorahReading::YomTov(YomTov::Shavuos2),
        });
    }

    v1.into()
}

pub(crate) fn get_chol_list(year: HebrewYear) -> SmallVec<[TorahReadingDay; 256]> {
    let mut special_days = smallvec![
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(30).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshCheshvan1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshCheshvan2),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(25).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::Chanukah1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(26).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::Chanukah2),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(27).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::Chanukah3),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(28).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::Chanukah4),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(29).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::Chanukah5),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Shvat, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshShvat),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(10).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::TenTeves),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshNissan),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(30).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshIyar1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshIyar2),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshSivan),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroI8::new(30).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshTammuz1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tammuz, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshTammuz2),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshAv),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(30).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshElul1),
        },
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, NonZeroI8::new(1).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::RoshChodeshElul2),
        },
    ];
    let mut second_vector = {
        let mut in_vec: SmallVec<[TorahReadingDay; 256]> = SmallVec::new();
        if year.sched[1] == 30 {
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroI8::new(30).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshKislev1),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshKislev2),
            });
        } else {
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshKislev),
            });
        }

        if year.sched[2] == 30 {
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(30).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshTeves1),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshTeves2),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroI8::new(30).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Chanukah6),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Chanukah7),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(2).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Chanukah8),
            });
        } else {
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshTeves),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Chanukah6),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(2).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Chanukah7),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroI8::new(3).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Chanukah8),
            });
        }

        if year.sched[5] != 0 {
            //If this is a regular year
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Shvat, NonZeroI8::new(30).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdar1),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdar2),
            });

            // Taanis Esther is on the 13th of Adar. However, If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            in_vec.push(if year.day_of_next_rh != Day::Thursday {
                TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar, NonZeroI8::new(13).unwrap())
                        .unwrap(),
                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            } else {
                TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar, NonZeroI8::new(11).unwrap())
                        .unwrap(),
                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroI8::new(14).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Purim),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroI8::new(15).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::ShushanPurim),
            });
        } else {
            //If this is a leap year

            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Shvat, NonZeroI8::new(30).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdarRishon1),
            });
            in_vec.push(TorahReadingDay {
                name: TorahReading::Chol(Chol::RoshChodeshAdarRishon2),
                day: year
                    .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(1).unwrap())
                    .unwrap(),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar1, NonZeroI8::new(30).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdarSheni1),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroI8::new(1).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::RoshChodeshAdarSheni2),
            });
            // If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            in_vec.push(if year.day_of_next_rh != Day::Thursday {
                TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar2, NonZeroI8::new(13).unwrap())
                        .unwrap(),

                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            } else {
                TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar2, NonZeroI8::new(11).unwrap())
                        .unwrap(),
                    name: TorahReading::Chol(Chol::TaanisEsther),
                }
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroI8::new(14).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::Purim),
            });
            in_vec.push(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroI8::new(15).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::ShushanPurim),
            });
        }

        in_vec
    };

    //17th of Tammuz is on Shabbos when the next Rosh Hashana starts on Monday (For example
    //5782/5783).
    let mut third_vector = if year.day_of_next_rh == Day::Monday {
        vec![
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Tammuz, NonZeroI8::new(18).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::SeventeenTammuz),
            },
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(10).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::NineAv),
            },
        ]
    } else {
        vec![
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Tammuz, NonZeroI8::new(17).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::SeventeenTammuz),
            },
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Av, NonZeroI8::new(9).unwrap())
                    .unwrap(),
                name: TorahReading::Chol(Chol::NineAv),
            },
        ]
    };
    let tzom_gedalya = if year.day_of_rh == Day::Thursday {
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(4).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::TzomGedalia),
        }
    } else {
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroI8::new(3).unwrap())
                .unwrap(),
            name: TorahReading::Chol(Chol::TzomGedalia),
        }
    };

    special_days.extend_from_slice(&mut second_vector);
    special_days.push(tzom_gedalya);
    special_days.extend_from_slice(&mut third_vector);

    special_days
}

/// This is based on the Biyur Halacha to Orach Chaim 428:4:3
pub(crate) fn get_shabbos_list(
    year: HebrewYear,
    location: Location,
) -> SmallVec<[TorahReadingDay; 256]> {
    let rh_dow = year.day_of_rh;
    let rh_dow_next = year.day_of_next_rh;
    let year_len = year.year_len;

    //Tazriya/Metzorah Acharei-Mos/Kedoshim and Behar/Bechukosai are always split on a leap year
    //and connected on a regular year. The only exception is (in Israel) that Behar is split when the year is a non-leap year, is regular ordered and Rosh Hashana is on Thursday
    let (split_tazriya, split_acharei, mut split_behar) = if year_len > 365 {
        (true, true, true)
    } else {
        (false, false, false)
    };
    if location == Location::Israel {
        split_behar = split_behar || year_len == 354 && rh_dow == Day::Thursday;
    }

    //Vayakhel/Pekudei is split if the year is a leap year or if it's a full year and Rosh Hashana
    //starts on Thursday.
    let split_vayakhel = year_len > 365 || (year_len == 355 && rh_dow == Day::Thursday);

    //Chukas Balak is split when the second day of Shavuos doesn't fall on Shabbos (The first day can't fall out on Shabbos, as then the next Rosh Hashana would start on Friday, which it can't). Shavuos falls on Shabbos (5783, for example) when the first day of the next Rosh Hashana is on a Shabbos.
    //Obviously, in Israel it's never split (as they don't have the second day of Shavuos).
    let split_chukas = location == Location::Israel || rh_dow_next != Day::Shabbos;
    //Mattos/Maasei is split only if it's a leap year and Rosh Hashana starts on a Thursday, and
    //the year is full, or empty.
    //In Israel, It's also split in a leap year which starts on a Monday and is full, or a
    //leap year starting on a Tuesday, and the year is an ordered year.
    //See this for more information: https://he.wikipedia.org/wiki/%D7%A4%D7%A8%D7%A9%D7%AA_%D7%9E%D7%98%D7%95%D7%AA

    let split_mattos = rh_dow == Day::Thursday && (year_len == 383 || year_len == 385)
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
        v.push(Parsha::Vayelech);
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

    let return_val = regular_shabbosim_list
        .iter()
        .enumerate()
        .map(|(i, &v)| TorahReadingDay {
            name: TorahReading::Shabbos(parsha_list[i]),
            day: v,
        })
        .collect::<SmallVec<[TorahReadingDay; 256]>>();
    return_val
}

pub(crate) fn get_special_parsha_list(year: HebrewYear) -> SmallVec<[TorahReadingDay; 256]> {
    let rh_dow_next = year.day_of_next_rh;
    let len_of_year = year.year_len;

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
                year.get_hebrew_date(HebrewMonth::Adar, NonZeroI8::new(1).unwrap())
                    .unwrap()
            } else {
                // This is a leap year
                year.get_hebrew_date(HebrewMonth::Adar2, NonZeroI8::new(1).unwrap())
                    .unwrap()
            }
        } else {
            let month = if len_of_year < 360 {
                HebrewMonth::Shvat
            } else {
                HebrewMonth::Adar1
            };
            // This is a regular year
            year.get_hebrew_date(
                month,
                NonZeroI8::new(match rh_dow_next {
                    Day::Monday => 25,
                    Day::Thursday => 29,
                    Day::Shabbos => 27,
                    _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
                })
                .unwrap(),
            )
            .unwrap()
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
            year.get_hebrew_date(month, NonZeroI8::new(day).unwrap())
                .unwrap()
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
            year.get_hebrew_date(month, NonZeroI8::new(day).unwrap())
                .unwrap()
        },
        name: TorahReading::SpecialParsha(SpecialParsha::Parah),
    };
    let hachodesh = TorahReadingDay {
        //Parshas Hachodesh is read on the Shabbos before Rosh Chodesh Nissan, or on Rosh Chodesh
        //Nissan itself.
        day: {
            if rh_dow_next == Day::Monday {
                //Hachodesh is read on Rosh Chodesh Nissan itself
                year.get_hebrew_date(HebrewMonth::Nissan, NonZeroI8::new(1).unwrap())
                    .unwrap()
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
                year.get_hebrew_date(month, NonZeroI8::new(day).unwrap())
                    .unwrap()
            }
        },
        name: TorahReading::SpecialParsha(SpecialParsha::HaChodesh),
    };

    smallvec![shekalim, zachor, parah, hachodesh]
}

pub(crate) fn get_shabbosim(
    year: HebrewYear,
    ignore_dates: &[TorahReadingDay],
) -> (Vec<HebrewDate>, Vec<HebrewDate>) {
    let amnt_days_to_shabbos = Day::Shabbos as u64 - (year.day_of_rh as u64);
    let mut cur_day = year.days_since_epoch + amnt_days_to_shabbos;
    let mut return_regular_shabbosim: Vec<HebrewDate> = Vec::new();
    let mut return_special_shabbosim: Vec<HebrewDate> = Vec::new();
    while cur_day < year.days_since_epoch + year.year_len {
        let day = year.get_hebrewdate_from_days_after_rh(cur_day);
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
    fn purim_should_never_start_on_a_friday_night() {
        for i in 3764..9999 {
            for day in get_chol_list(HebrewYear::new(i).unwrap()).iter() {
                if day.name == TorahReading::Chol(Chol::Purim) {
                    assert_ne!(day.day.to_gregorian().weekday(), Weekday::Fri);
                }
            }
        }
    }
    #[test]
    fn fasts_should_never_start_on_friday_night() {
        for i in 3764..9999 {
            for day in get_chol_list(HebrewYear::new(i).unwrap()).iter() {
                if day.name == TorahReading::Chol(Chol::TzomGedalia)
                    || day.name == TorahReading::Chol(Chol::TenTeves)
                    || day.name == TorahReading::Chol(Chol::SeventeenTammuz)
                    || day.name == TorahReading::Chol(Chol::NineAv)
                {
                    println!(
                        "{:?}  {:?} {:?}",
                        day.name,
                        day.day,
                        day.day.to_gregorian().weekday()
                    );
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
                let y = HebrewYear::new(i).unwrap();
                let date = if let Ok(date) =
                    HebrewDate::from_ymd(i, HebrewMonth::Adar, NonZeroI8::new(1).unwrap())
                {
                    date
                } else {
                    HebrewDate::from_ymd(i, HebrewMonth::Adar2, NonZeroI8::new(1).unwrap()).unwrap()
                }
                .to_gregorian();
                assert_eq!(
                    get_shabbos_list(y, *loc)
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
                let date = HebrewDate::from_ymd(i, HebrewMonth::Nissan, NonZeroI8::new(1).unwrap())
                    .unwrap()
                    .to_gregorian();
                assert_eq!(
                    get_shabbos_list(HebrewYear::new(i).unwrap(), *loc)
                        .iter()
                        .filter(|x| x.name == TorahReading::SpecialParsha(SpecialParsha::HaChodesh))
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
                let date = if let Ok(date) =
                    HebrewDate::from_ymd(i, HebrewMonth::Adar, NonZeroI8::new(14).unwrap())
                {
                    date
                } else {
                    HebrewDate::from_ymd(i, HebrewMonth::Adar2, NonZeroI8::new(14).unwrap())
                        .unwrap()
                }
                .to_gregorian();
                assert_eq!(
                    get_shabbos_list(HebrewYear::new(i).unwrap(), *loc)
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
                get_shabbos_list(HebrewYear::new(i).unwrap(), Location::Chul)
                    .iter()
                    .filter(|&x| (*x).day.to_gregorian().weekday() != Weekday::Fri)
                    .count(),
                0
            );
            assert_eq!(
                get_shabbos_list(HebrewYear::new(i).unwrap(), Location::Israel)
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
            println!("Getting chul yt list");
            get_yt_list(HebrewYear::new(i).unwrap(), Location::Chul);
            println!("Getting eretz yt list");
            get_yt_list(HebrewYear::new(i).unwrap(), Location::Israel);
            println!("Getting chol list");
            get_chol_list(HebrewYear::new(i).unwrap());
        }
    }

    #[test]
    fn get_shabbosim_fall_on_shabbos() {
        for i in 3764..9999 {
            get_shabbosim(HebrewYear::new(i).unwrap(), &[])
                .0
                .iter()
                //Shabbos starts on _Friday_ night
                .for_each(|x| assert_eq!(x.to_gregorian().weekday(), Weekday::Fri));
            get_shabbosim(HebrewYear::new(i).unwrap(), &[])
                .1
                .iter()
                //Shabbos starts on _Friday_ night
                .for_each(|x| assert_eq!(x.to_gregorian().weekday(), Weekday::Fri));
        }
    }
}
