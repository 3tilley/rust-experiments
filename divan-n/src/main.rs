// main.rs
use std::hint::black_box;
use std::thread::sleep;
use std::time::Duration;

pub fn add_multiple(left: usize, right: usize, n: usize) -> usize {
    let mut sum = 0;
    for _ in 0..n {
        sum += black_box(left) + right
    }
    sum
}

pub fn sleeper(micros: usize, n: usize) {
    for _ in 0..n {
        sleep(Duration::from_micros(micros as u64))
    }
}

#[divan::bench(args = [1, 10, 100, 1000, 10000])]
fn add_n_times(bencher: divan::Bencher, n: usize) {
    bencher.counter(n).bench_local(|| add_multiple(3, 4, n));
}

#[divan::bench(args = [1, 10, 100, 1000], sample_count=10)]
fn sleep_n_times(bencher: divan::Bencher, n: usize) {
    bencher.counter(n).bench_local(|| sleeper(100, n));
}

fn main() {
    divan::Divan::from_args().run_benches();
}
