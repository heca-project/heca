#![cfg_attr(test, feature(test))]

extern crate cpuprofiler;
extern crate heca_lib;
extern crate time;
#[macro_use]
extern crate clap;

use clap::App;
use cpuprofiler::PROFILER;
use heca_lib::*;

mod args;
fn main() {
    use args;
    let args = args::build_args();
}

#[cfg(test)]
mod test {
    extern crate test;
    use test::Bencher;
    #[bench]
    fn how_long_it_takes_to_par_iter(b: &mut Bencher) {
        b.iter(|| {
            use rayon::prelude::*;
            test::black_box({
                (0..10000)
                    .into_par_iter()
                    .map(|x| x * 2)
                    .collect::<Vec<i32>>()
            })
        });
    }
    #[bench]
    fn how_long_it_takes_to_iter(b: &mut Bencher) {
        b.iter(|| {
            use rayon::prelude::*;
            test::black_box({ (0..10000).into_iter().map(|x| x * 2).collect::<Vec<i32>>() })
        });
    }

}
