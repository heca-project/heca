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
    let output_type = output_type();
    let args = match args::build_args(std::env::args(), output_type) {
        Err(err) => {
            let stderr = stderr();
            let mut lock_stderr = BufWriter::with_capacity(1024 * 1024, stderr.lock());

            if output_type == OutputType::JSON {
                err.json_print(&mut lock_stderr);
                lock_stderr.write(b"\n").unwrap();
            } else {
                lock_stderr.write(format!("{}", err).as_bytes()).unwrap();
            }
            lock_stderr.flush().unwrap();
            std::process::exit(1);
        }
        Ok(res) => res,
    };

    let stdout = stdout();
    let stderr = stderr();
    let mut lock_stdout = BufWriter::with_capacity(1024 * 1024, stdout.lock());
    let mut lock_stderr = BufWriter::with_capacity(1024 * 1024, stderr.lock());

    if let Err(err) = app(args, &mut lock_stdout) {
        if output_type == OutputType::JSON {
            err.json_print(&mut lock_stderr);
            lock_stderr.write(b"\n").unwrap();
        } else {
            lock_stderr.write(format!("{}", err).as_bytes()).unwrap();
        }
        lock_stderr.flush().unwrap();
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

fn app<'a, 'b>(args: MainArgs, lock: &'a mut BufWriter<StdoutLock<'b>>) -> Result<(), AppError> {
    match args.command {
        Command::List(ref sub_args) => sub_args.run(&args, lock)?,
        Command::Convert(ref sub_args) => sub_args.run(&args, lock)?,
    };

    Ok(())
}
