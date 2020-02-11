mod constants;
use crate::algorithms::daily_study::rambam::constants::RAMBAM;
use crate::args::types::Language;
use crate::prelude::Json;
use crate::prelude::JsonPrinter;
use std::io::BufWriter;
use std::io::StdoutLock;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum RambamChapters {
    Three,
    One,
}

#[derive(Debug, Clone)]
pub struct RambamThreeChapter {
    ch1: RambamChapter,
    ch2: RambamChapter,
    ch3: RambamChapter,
}
impl JsonPrinter for RambamThreeChapter {
    fn json_print(&self, json: &mut Json<'_, '_>) {
        json.print_map_unchecked("type", "Rambam3Chapters");
        json.next();

        json.start_new_array("topic");

        json.start();
        json.print_map_unchecked("halacha", self.ch1.halacha_json);
        json.next();
        json.print_map_u8("chapter", self.ch1.chapter);
        json.end();

        json.next();

        json.start();
        json.print_map_unchecked("halacha", self.ch2.halacha_json);
        json.next();
        json.print_map_u8("chapter", self.ch2.chapter);
        json.end();

        json.next();

        json.start();
        json.print_map_unchecked("halacha", self.ch3.halacha_json);
        json.next();
        json.print_map_u8("chapter", self.ch3.chapter);
        json.end();

        json.end_array();
    }
}

impl RambamThreeChapter {
    pub fn from_days(day: u16) -> Self {
        let day_1 = day * 3;
        let day_2 = day * 3 + 1;
        let day_3 = day * 3 + 2;
        let ch1 = RambamChapter::from_days(day_1);
        let ch2 = RambamChapter::from_days(day_2);
        let ch3 = RambamChapter::from_days(day_3);
        Self { ch1, ch2, ch3 }
    }

    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let mut sum = self.ch1.pretty_print(lock, language)?;
        sum += lock.write(b" - ").ok()?;
        sum += self.ch3.pretty_print(lock, language)?;
        Some(sum)
    }
}

#[derive(Debug, Clone)]
pub struct RambamChapter {
    halacha_english: &'static str,
    halacha_json: &'static str,
    halacha_hebrew: &'static str,
    chapter: u8,
}
impl JsonPrinter for RambamChapter {
    fn json_print(&self, json: &mut Json<'_, '_>) {
        json.print_map_unchecked("type", "Rambam1Chapter");
        json.next();

        json.start_new_map("topic");
        json.print_map_unchecked("halacha", self.halacha_json);
        json.next();
        json.print_map_u8("chapter", self.chapter);
        json.end();
    }
}

impl RambamChapter {
    pub fn from_days(day: u16) -> Self {
        let mut day = day;
        let mut index = 0;
        let mut halacha_english;
        let mut halacha_json;
        let mut halacha_hebrew;

        let chapter = loop {
            halacha_english = RAMBAM[index].0;
            halacha_hebrew = RAMBAM[index].1;
            halacha_json = RAMBAM[index].2;

            if day < (RAMBAM[index].3 as u16) {
                break day as u8 + 1;
            } else {
                day -= RAMBAM[index].3 as u16;
                index += 1;
            }
        };
        Self {
            halacha_english,
            halacha_json,
            halacha_hebrew,
            chapter,
        }
    }

    pub fn pretty_print(
        &self,
        lock: &mut BufWriter<StdoutLock<'_>>,
        language: Language,
    ) -> Option<usize> {
        let mut p = if language == Language::English {
            lock.write(self.halacha_english.as_bytes()).ok()?
        } else {
            lock.write(self.halacha_hebrew.as_bytes()).ok()?
        };
        p += lock.write(b" ").ok()?;
        let mut daf_arr = [b'\0'; 3];
        let count_y = itoa::write(&mut daf_arr[..], self.chapter).unwrap();
        p += lock.write(&daf_arr[..count_y]).ok()?;
        Some(p)
    }
}
