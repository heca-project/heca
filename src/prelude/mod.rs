pub mod print;
use crate::args::types::Language;
use crate::args::types::{AppError, MainArgs};
pub use print::*;
use std::io::BufWriter;
use std::io::{StdoutLock, Write};

pub trait Runnable {
    fn run<'a, 'b>(
        &self,
        args: &MainArgs,
        lock: &'a mut BufWriter<StdoutLock<'b>>,
    ) -> Result<(), AppError>;
}
pub trait MyToString {
    fn to_string(&self, language: Language) -> &'static str;
}

pub(crate) fn hebrew_month_to_string(input: heca_lib::prelude::HebrewMonth) -> &'static str {
    use heca_lib::prelude::HebrewMonth::*;
    match input {
        Tishrei => "Tishrei",
        Cheshvan => "Cheshvan",
        Kislev => "Kislev",
        Teves => "Teves",
        Shvat => "Shvat",
        Adar => "Adar",
        Adar1 => "Adar1",
        Adar2 => "Adar2",
        Nissan => "Nissan",
        Iyar => "Iyar",
        Sivan => "Sivan",
        Tammuz => "Tammuz",
        Av => "Av",
        Elul => "Elul",
    }
}

pub(crate) fn write_i64(input: i64, lock: &mut BufWriter<StdoutLock<'_>>) {
    let mut arr = [b'\0'; 20];
    let count = itoa::write(&mut arr[..], input).unwrap();
    lock.write(&arr[..count]).unwrap();
}

pub(crate) trait JsonPrinter {
    fn json_print(&self, json: &mut Json<'_, '_>);
}

pub(crate) fn string_to_json(s: &str) -> String {
    s.replace('"', "\\\"")
}

pub(crate) struct Json<'a, 'b> {
    lock: &'a mut BufWriter<StdoutLock<'b>>,
}

impl<'a, 'b> Json<'a, 'b> {
    pub(crate) fn new(lock: &'a mut BufWriter<StdoutLock<'b>>) -> Json<'a, 'b> {
        Json { lock }
    }
    pub(crate) fn start(&mut self) {
        self.lock.write(b"{").unwrap();
    }
    pub(crate) fn print_map_unchecked(&mut self, key: &str, val: &str) {
        self.lock.write(b"\"").unwrap();
        self.lock.write(key.as_bytes()).unwrap();
        self.lock.write(b"\":\"").unwrap();
        self.lock.write(val.as_bytes()).unwrap();
        self.lock.write(b"\"").unwrap();
    }
    pub(crate) fn start_new_map(&mut self, key: &str) {
        self.lock.write(b"\"").unwrap();
        self.lock.write(key.as_bytes()).unwrap();
        self.lock.write(b"\":{").unwrap();
    }
    pub(crate) fn start_new_array(&mut self, key: &str) {
        self.lock.write(b"\"").unwrap();
        self.lock.write(key.as_bytes()).unwrap();
        self.lock.write(b"\":[").unwrap();
    }
    pub(crate) fn end_array(&mut self) {
        self.lock.write(b"]").unwrap();
    }
    pub(crate) fn print_map_u8(&mut self, key: &str, val: u8) {
        self.lock.write(b"\"").unwrap();
        self.lock.write(key.as_bytes()).unwrap();
        self.lock.write(b"\":").unwrap();

        let mut val_arr: [u8; 10] = [0; 10];
        let count_val_arr = itoa::write(&mut val_arr[..], val).unwrap();

        self.lock.write(&val_arr[0..count_val_arr]).unwrap();
    }
    pub(crate) fn end(&mut self) {
        self.lock.write(b"}").unwrap();
    }
    pub(crate) fn next(&mut self) {
        self.lock.write(b",").unwrap();
    }
}
