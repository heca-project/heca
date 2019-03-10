use std::cmp::Ordering;

use crate::convert::HebrewDate;

#[derive(Debug, Eq, Clone, Copy)]
pub struct TorahReadingDay {
    pub(crate) day: HebrewDate,
    pub(crate) name: TorahReading,
}

impl TorahReadingDay {
    #[inline]
    pub fn day(&self) -> HebrewDate {
        self.day
    }

    #[inline]
    pub fn name(&self) -> TorahReading {
        self.name.clone()
    }
}

impl PartialOrd for TorahReadingDay {
    #[inline]
    fn partial_cmp(&self, other: &TorahReadingDay) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TorahReadingDay {
    #[inline]
    fn cmp(&self, other: &TorahReadingDay) -> Ordering {
        self.day.cmp(&other.day)
    }
}

impl PartialEq for TorahReadingDay {
    #[inline]
    fn eq(&self, other: &TorahReadingDay) -> bool {
        self.day == other.day
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Location {
    Israel,
    Chul,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
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

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TorahReading {
    YomTov(YomTov),
    Chol(Chol),
    Shabbos(Parsha),
    SpecialParsha(SpecialParsha),
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum SpecialParsha {
    Shekalim,
    Zachor,
    Parah,
    Hachodesh,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
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
    TenTeves,
    RoshChodeshShvat,
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
    TaanisEsther,
    Purim,
    ShushanPurim,
    RoshChodeshAdarRishon1,
    RoshChodeshAdarRishon2,
    RoshChodeshAdarSheni1,
    RoshChodeshAdarSheni2,
    SeventeenTammuz,
    NineAv,
}



#[derive(Clone, Debug, Eq, PartialEq, Copy)]
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
