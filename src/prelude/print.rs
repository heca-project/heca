use heca_lib::prelude::HebrewMonth;

pub fn hebrew_month_hebrew(h: HebrewMonth) -> &'static str {
    match h {
        HebrewMonth::Tishrei => "תשרי",
        HebrewMonth::Cheshvan => "חשוון",
        HebrewMonth::Kislev => "כסלו",
        HebrewMonth::Teves => "טבת",
        HebrewMonth::Shvat => "שבט",
        HebrewMonth::Adar => "אדר",
        HebrewMonth::Adar1 => "אדר א",
        HebrewMonth::Adar2 => "אדר ב",
        HebrewMonth::Nissan => "ניסן",
        HebrewMonth::Iyar => "אייר",
        HebrewMonth::Sivan => "סיוון",
        HebrewMonth::Tammuz => "תמוז",
        HebrewMonth::Av => "אב",
        HebrewMonth::Elul => "אלול",
    }
}

pub fn hebrew_month_english(h: HebrewMonth) -> &'static str {
    match h {
        HebrewMonth::Tishrei => "Tishrei",
        HebrewMonth::Cheshvan => "Cheshvan",
        HebrewMonth::Kislev => "Kislev",
        HebrewMonth::Teves => "Teves",
        HebrewMonth::Shvat => "Shvat",
        HebrewMonth::Adar => "Adar",
        HebrewMonth::Adar1 => "Adar Rishon",
        HebrewMonth::Adar2 => "Adar Sheni",
        HebrewMonth::Nissan => "Nissan",
        HebrewMonth::Iyar => "Iyar",
        HebrewMonth::Sivan => "Sivan",
        HebrewMonth::Tammuz => "Tammuz",
        HebrewMonth::Av => "Av",
        HebrewMonth::Elul => "Elul",
    }
}
