use raw_sync::events::{EventImpl, EventInit};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub mod shmem;
pub mod pipes;

pub struct ExecutionResult {
    name: String,
    start: Instant,
    elapsed: Duration,
    cycles: usize,
}

impl ExecutionResult {
    fn new(name: String, start: Instant, elapsed: Duration, cycles: usize) -> ExecutionResult {
        ExecutionResult {
            name,
            start,
            elapsed,
            cycles,
        }
    }

    fn print_info(&self) {
        let duration = humantime::Duration::from(self.elapsed);
        let ps = 1_000_000f32 * (self.cycles as f32)  / (duration.as_micros() as f32);
        let per_op = humantime::Duration::from(Duration::from_nanos((1_000_000_000f32 / ps) as u64));
        println!("{} cycles completed in {} \n{} per second\n{} per operation", self.cycles, duration, ps, per_op);
    }
}

fn executable_path(name: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    let exe = name.to_owned() + ".exe";
    #[cfg(target_family = "unix")]
    let exe = name.to_owned();

    #[cfg(debug_assertions)]
    let out = PathBuf::from("./target/debug/").join(exe);
    #[cfg(not(debug_assertions))]
    let out = PathBuf::from("./target/release/").join(exe);

    out
}
