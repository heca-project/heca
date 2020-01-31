pub mod constants;
pub mod get_omer;
pub mod print;
use crate::args::types::{AppError, MainArgs};
pub use get_omer::get_omer;
pub use print::*;
use std::io::BufWriter;
use std::io::{stderr, stdout, StdoutLock};

pub trait Runnable {
    fn run(&self, args: &MainArgs, lock: &mut BufWriter<StdoutLock<'_>>) -> Result<(), AppError>;
}
