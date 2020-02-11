use crate::args::types::Language;
use crate::prelude::Json;
use crate::prelude::JsonPrinter;
use std::io::{BufWriter, StdoutLock};
pub mod constants;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Daf {
    masechta_english: &'static str,
    masechta_json: &'static str,
    masechta_hebrew: &'static str,
    daf: u8,
}
impl JsonPrinter for Daf {
    fn json_print(&self, json: &mut Json<'_, '_>) {
        json.print_map_unchecked("type", "DafYomi");
        json.next();
        json.start_new_map("topic");
        json.print_map_unchecked("masechta", self.masechta_json);
        json.next();
        json.print_map_u8("daf", self.daf + 2);
        json.end();
    }
}
impl Daf {
    pub fn from_days(
        day: u16,
        gemaras: &[(&'static str, &'static str, &'static str, u8); 37],
    ) -> Self {
        let mut day = day;
        let mut index = 0;
        let mut masechta_english;
        let mut masechta_json;
        let mut masechta_hebrew;

        let daf = loop {
            masechta_english = gemaras[index].0;
            masechta_hebrew = gemaras[index].1;
            masechta_json = gemaras[index].2;

            if day < (gemaras[index].3 as u16 - 1) {
                break day as u8;
            } else {
                day -= gemaras[index].3 as u16 - 1;
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
        let count_y = itoa::write(&mut daf_arr[..], self.daf + 2).unwrap();
        p += lock.write(&daf_arr[..count_y]).ok()?;
        Some(p)
    }
}
