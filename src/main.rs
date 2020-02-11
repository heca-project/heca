mod algorithms;
mod args;
mod convert;
mod list;
mod prelude;

use crate::args::types;
use crate::args::types::AppError;
use crate::args::types::*;
use crate::prelude::*;
use std::io::BufWriter;
use std::io::Write;
use std::io::{stderr, stdout, StdoutLock};

fn main() {
    let stdout = stdout();
    let stderr = stderr();
    let mut lock_stdout = BufWriter::with_capacity(1024 * 1024, stdout.lock());
    let mut lock_stderr = BufWriter::with_capacity(1024 * 1024, stderr.lock());

    let output_type = output_type();
    if let Err(err) = app(std::env::args(), output_type, &mut lock_stdout) {
        if output_type == OutputType::JSON {
            err.json_print(&mut lock_stderr);
            lock_stderr.write(b"\n").unwrap();
            lock_stderr.flush().unwrap();
        } else {
            lock_stderr.write(format!("{}", err).as_bytes()).unwrap();
        }
        std::process::exit(1);
    }
    lock_stderr.flush().unwrap();
    lock_stdout.flush().unwrap();
}

fn output_type() -> OutputType {
    let mut args = std::env::args();
    loop {
        let arg = args.next();
        if arg == None {
            break;
        } else if let Some(arg) = arg {
            if arg == "--print=json" {
                return OutputType::JSON;
            } else if arg == "--print" {
                if let Some(next) = args.next() {
                    if next == "json" {
                        return OutputType::JSON;
                    }
                }
            }
        }
    }

    if let Ok(json_str) = std::env::var("JSON") {
        if json_str == "YES" {
            return OutputType::JSON;
        }
    }

    OutputType::Pretty
}

fn app<'a, 'b, I, T>(
    args: I,
    output_type: OutputType,
    lock: &'a mut BufWriter<StdoutLock<'b>>,
) -> Result<(), AppError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = args::build_args(args, output_type)?;

    match args.command {
        Command::List(ref sub_args) => sub_args.run(&args, lock)?,
        Command::Convert(ref sub_args) => sub_args.run(&args, lock)?,
    };

    Ok(())
}
