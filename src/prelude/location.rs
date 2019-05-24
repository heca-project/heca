use serde::*;
use std::cmp::Ordering;

use crate::convert::HebrewDate;

#[derive(Debug, Eq, Copy, Clone, Serialize)]
/// This struct holds a day on which the Torah is read.
///
/// You can get the Hebrew Date and the Torah reading.
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
        self.name
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

#[derive(Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Location {
    Israel,
    Chul,
}

/// Yom Tov, including Rosh Hashana, Yom Kippur and Chol HaMoed
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum YomTov {
    RoshHashanah1,
    RoshHashanah2,
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

#[derive(Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TorahReading {
    YomTov(YomTov),
    Chol(Chol),
    Shabbos(Parsha),
    SpecialParsha(SpecialParsha),
}

/// Special Parshas read every winter
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpecialParsha {
    Shekalim,
    Zachor,
    Parah,
    HaChodesh,
}
/// Possible weekday Torah readings
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Chol {
    TzomGedalia,
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

/// Weekly Torah Portion
#[derive(Clone, Debug, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub enum Parsha {
    Vayelech,
    Haazinu,
    Bereishis,
    Noach,
    LechLecha,
    Vayeira,
    ChayeiSara,
    Toldos,
    Vayetzei,
    Vayishlach,
    Vayeshev,
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
}
