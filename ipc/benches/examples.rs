use std::hint::black_box;
use divan::Bencher;

fn main() {
    divan::main();
    // divan::Divan::from_args().run_benches();
}

// #[divan::bench(threads=1)]
#[divan::bench(sample_size=1)]
fn stdin_stdout_1000(bencher: Bencher) {
    let mut pipe_runner = ipc::PipeRunner::new(false);
    bencher
        // .with_inputs()
        .counter(divan::counter::ItemsCount::new(1000usize))
        .bench_local(move || {
        pipe_runner.run(1)
    });
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
fn stdin_stdout_inner_1000(bencher: Bencher) {
    // println!("Starting proc");
    let mut pipe_runner = ipc::PipeRunner::new(false);
    // println!("Preparing");
    let mut return_buffer = pipe_runner.prepare();
    bencher.counter(divan::counter::ItemsCount::new(1000usize))
        .bench_local(move || {
        // println!("Starting run");
        pipe_runner.run_inner(1000, &mut return_buffer)
    });
}

#[divan::bench]
fn add() -> i32 {
    black_box(2) + black_box(1)
}