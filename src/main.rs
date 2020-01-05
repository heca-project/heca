mod algorithms;
mod args;
mod convert;
mod list;
mod prelude;

use crate::args::types;
use crate::args::types::AppError;
use crate::args::types::*;
use crate::prelude::*;

fn main() {
    start_benchmark();
    let output_type = output_type();
    if let Err(err) = app(std::env::args(), output_type) {
        if output_type == OutputType::JSON {
            eprintln!("{}", serde_json::to_string(&err).unwrap());
        } else {
            eprintln!("{}", err);
        }
        stop_benchmark();
        std::process::exit(1);
    }
    stop_benchmark();
}

#[cfg(not(feature = "profile"))]
fn start_benchmark() {}

#[cfg(not(feature = "profile"))]
fn stop_benchmark() {}
#[cfg(feature = "profile")]
fn start_benchmark() {
    use cpuprofiler::*;
    PROFILER.lock().unwrap().start("/tmp/heca.profile").unwrap();
}

#[cfg(feature = "profile")]
fn stop_benchmark() {
    use cpuprofiler::*;
    PROFILER.lock().unwrap().stop().unwrap();
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

fn app<I, T>(args: I, output_type: OutputType) -> Result<(), AppError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = args::build_args(args, output_type)?;
    match args.command {
        Command::List(ref sub_args) => sub_args.run(&args)?,
        Command::Convert(ref sub_args) => sub_args.run(&args)?,
    };

    Ok(())
}
