use divan::Bencher;
use std::hint::black_box;
use ipc::tcp::TcpRunner;

fn main() {
    divan::main();
    // divan::Divan::from_args().run_benches();
}

// #[divan::bench(threads=1)]
#[divan::bench(sample_size = 1)]
fn stdin_stdout_full_1000(bencher: Bencher) {
    let mut pipe_runner = ipc::pipes::PipeRunner::new(false);
    bencher
        // .with_inputs()
        .counter(divan::counter::ItemsCount::new(1000usize))
        .bench_local(move || pipe_runner.run(1, false));
}

// // #[divan::bench]
// fn stdin_stdout_2000(bencher: Bencher) {
//     let mut pipe_runner_2 = ipc::PipeRunner::new(true);
//     bencher
//         .counter(divan::counter::ItemsCount::new(2000usize))
//         .bench_local(|| {
//         pipe_runner_2.run(1000)
//     });
// }
//
#[divan::bench]
fn stdin_stdout_1000(bencher: Bencher) {
    // println!("Starting proc");
    let mut pipe_runner = ipc::pipes::PipeRunner::new(false);
    // println!("Preparing");
    let mut return_buffer = pipe_runner.prepare();
    bencher
        .counter(divan::counter::ItemsCount::new(1000usize))
        .bench_local(move || {
            // println!("Starting run");
            pipe_runner.run_inner(1000, &mut return_buffer)
        });
}

#[divan::bench]
fn shared_memory_10000(bencher: Bencher) {
    // println!("Starting proc");
    let mut shmem_runner = ipc::shmem::ShmemRunner::new(true);
    // println!("Preparing");
    bencher
        .counter(divan::counter::ItemsCount::new(10000usize))
        .bench_local(move || {
            // println!("Starting run");
            shmem_runner.run(10000, false);
        });
}

#[divan::bench]
fn tcp_1000_nodelay(bencher: Bencher) {
    let n = 1000usize;
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
fn tcp_1000_yesdelay(bencher: Bencher) {
    let n = 1000usize;
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
fn udp_1000(bencher: Bencher) {
    let n = 1000usize;
    let mut udp_runner = ipc::udp::UdpRunner::new(true);
    bencher
        .counter(divan::counter::ItemsCount::new(n))
        .bench_local(move || {
            // println!("Starting run");
            udp_runner.run(n, false);
        });
}

#[divan::bench]
fn add() -> i32 {
    black_box(2) + black_box(1)
}
