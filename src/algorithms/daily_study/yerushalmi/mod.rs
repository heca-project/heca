use crate::args::types::Language;
use crate::prelude::Json;
use crate::prelude::JsonPrinter;
use constants::YERUSHALMI;
use std::io::{BufWriter, StdoutLock, Write};

mod constants;
#[derive(Debug, Clone)]
pub struct YerushalmiYomi {
    masechta_english: &'static str,
    masechta_json: &'static str,
    masechta_hebrew: &'static str,
    daf: u8,
}

impl YerushalmiYomi {
    pub fn from_days(day: u16) -> Self {
        let mut day = day;
        let mut index = 0;
        let mut masechta_english;
        let mut masechta_json;
        let mut masechta_hebrew;

        let daf = loop {
            masechta_english = YERUSHALMI[index].0;
            masechta_hebrew = YERUSHALMI[index].1;
            masechta_json = YERUSHALMI[index].2;

            if day < (YERUSHALMI[index].3 as u16) {
                break day as u8;
            } else {
                day -= YERUSHALMI[index].3 as u16;
                index += 1;
            }
        };
        Self {
            masechta_english,
            masechta_json,
            masechta_hebrew,
            daf,
        }
    }
    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let mut p = if language == Language::English {
            lock.write(self.masechta_english.as_bytes()).ok()?
        } else {
            lock.write(self.masechta_hebrew.as_bytes()).ok()?
        };
        p += lock.write(b" ").ok()?;
        let mut daf_arr = [b'\0'; 3];
        let count_y = itoa::write(&mut daf_arr[..], self.daf + 1).unwrap();
        p += lock.write(&daf_arr[..count_y]).ok()?;
        Some(p)
    }
}

impl JsonPrinter for YerushalmiYomi {
    fn json_print(&self, json: &mut Json<'_, '_>) {
        json.print_map_unchecked("type", "Yerushalmi");
        json.next();
        json.start_new_map("topic");
        json.print_map_unchecked("masechta", self.masechta_json);
        json.next();
        json.print_map_u8("daf", self.daf + 1);
        json.end();
    }
}

/*
impl Serialize for YerushalmiYomi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::types::*;
        let mut state = serializer.serialize_struct("Day", 2)?;
        state.serialize_field("masechta", &self.masechta_json)?;
        state.serialize_field("daf", &(self.daf + 1))?;
        state.end()
    }
}
*/
