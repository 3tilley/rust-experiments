use clap::Parser;
use ipc::pipes::PipeRunner;
use ipc::shmem::ShmemRunner;
use ipc::tcp::TcpRunner;
use ipc::udp::UdpRunner;

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
        Method::Tcp => {
            let mut runner = TcpRunner::new(args.start_child, true);
            runner.run(args.number, true);
        }
        Method::Udp => {
            let mut runner = UdpRunner::new(true);
            runner.run(args.number, true);
        }
    }
}

#[derive(Debug, Default, Copy, Clone, clap::ValueEnum)]
enum Method {
    #[default]
    Stdout,
    Shmem,
    Tcp,
    Udp,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    number: usize,

    #[clap(short, long, default_value_t, value_enum)]
    method: Method,

    #[arg(short, long, action, default_value_t = true)]
    start_child: bool,
}
