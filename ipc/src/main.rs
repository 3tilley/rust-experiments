use std::io::{Read, Write};
use clap::Parser;
use lib::PipeRunner;
use crate::lib::ShmemRunner;

mod lib;

fn main() {
    let args = Cli::parse();
    match args.method {
        Method::Stdout => {
            let mut pr = PipeRunner::new(false);
            pr.run(args.number);
        },
        Method::Shmem => {
            let mut runner = ShmemRunner::new();
            runner.run(10);
        }
    }
}

// #[divan::bench]
// pub fn pipe_n(n: usize) {
//
// }

#[derive(Debug, Default, Copy, Clone, clap::ValueEnum)]
enum Method {
    #[default]
    Stdout,
    Shmem,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    number: usize,

    #[clap(short, long, default_value_t, value_enum)]
    method: Method,
}
