pub mod constants;
pub mod get_omer;
pub mod print;
use crate::args::types::{AppError, MainArgs};
pub use get_omer::get_omer;
pub use print::*;
use std::io::BufWriter;
use std::io::{StdoutLock, Write};

pub trait Runnable {
    fn run(&self, args: &MainArgs, lock: &mut BufWriter<StdoutLock<'_>>) -> Result<(), AppError>;
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
