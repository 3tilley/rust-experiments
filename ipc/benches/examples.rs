// This is in place to allow me to test a local Divan change
#[cfg(feature = "dev-divan")]
extern crate divan_dev as divan;

use divan::Bencher;
use ipc::tcp::TcpRunner;
use std::hint::black_box;

const N: usize = 1000;

fn main() {
    divan::main();
    // divan::Divan::from_args().run_benches();
}

// This creates the return array "holding the response" before passing it to the function
#[divan::bench]
fn stdin_stdout_no_preallocate(bencher: Bencher) {
    let n = N;
    let mut pipe_runner = ipc::pipes::PipeRunner::new(false);
    bencher
        .counter(n)
        .bench_local(move || pipe_runner.run(n, false));
}

// This is a test as to whether it's more efficient to preallocate the return array
#[divan::bench]
fn stdin_stdout(bencher: Bencher) {
    let n = N;
    let mut pipe_runner = ipc::pipes::PipeRunner::new(false);
    let mut return_buffer = pipe_runner.prepare();
    bencher
        .counter(divan::counter::ItemsCount::new(n))
        .bench_local(move || pipe_runner.run_inner(n, &mut return_buffer));
}

#[divan::bench]
fn tcp_nodelay(bencher: Bencher) {
    let n = N;
    // println!("Starting proc");
    let mut tcp_runner = ipc::tcp::TcpRunner::new(true, true);
    // println!("Preparing");
    bencher
        .counter(divan::counter::ItemsCount::new(n))
        .bench_local(move || {
            // println!("Starting run");
            tcp_runner.run(n, false);
        });
}

#[divan::bench]
fn tcp_yesdelay(bencher: Bencher) {
    let n = N;
    // println!("Starting proc");
    let mut tcp_runner = ipc::tcp::TcpRunner::new(true, false);
    // println!("Preparing");
    bencher
        .counter(divan::counter::ItemsCount::new(n))
        .bench_local(move || {
            // println!("Starting run");
            tcp_runner.run(n, false);
        });
}

#[divan::bench]
fn udp(bencher: Bencher) {
    let n = N;
    let mut udp_runner = ipc::udp::UdpRunner::new(true);
    bencher
        .counter(divan::counter::ItemsCount::new(n))
        .bench_local(move || {
            // println!("Starting run");
            udp_runner.run(n, false);
        });
}

#[divan::bench]
fn shared_memory(bencher: Bencher) {
    // let n = N * 100;
    let n = N;
    // println!("Starting proc");
    let mut shmem_runner = ipc::shmem::ShmemRunner::new(true);
    // println!("Preparing");
    bencher
        .counter(divan::counter::ItemsCount::new(n))
        .bench_local(move || {
            // println!("Starting run");
            shmem_runner.run(n, false);
        });
}
