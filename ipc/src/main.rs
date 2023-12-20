use ipc::shmem::ShmemRunner;
use clap::Parser;
use ipc::pipes::PipeRunner;
use std::io::{Read, Write};

fn main() {
    let args = Cli::parse();
    match args.method {
        Method::Stdout => {
            let mut pr = PipeRunner::new(false);
            pr.run(args.number, true);
        }
        Method::Shmem => {
            let mut runner = ShmemRunner::new(args.start_child);
            runner.run(args.number, true);
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

    #[arg(short, long, action)]
    start_child: bool,
}
