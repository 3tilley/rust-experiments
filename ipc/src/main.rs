use std::io::{Read, Write};
use clap::Parser;
use lib::PipeRunner;

mod lib;

fn main() {
    let args = Cli::parse();
    let mut pr = PipeRunner::new(false);
    pr.run(args.number);
    let mut pr2 = PipeRunner::new(false);
    pr2.run(args.number);
}

// #[divan::bench]
// pub fn pipe_n(n: usize) {
//
// }

enum Method {
    Stdout,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    number: usize,
}
