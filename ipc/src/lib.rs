use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use std::io::{Read, stdin, Write};
use std::os::windows::io::AsRawHandle;
use std::os::windows::process::CommandExt;

pub struct ExecutionResult {
    name: String,
    start: Instant,
    elapsed: Duration,
    cycles: usize
}

impl ExecutionResult {
    fn new(name: String, start: Instant, elapsed: Duration, cycles: usize) -> ExecutionResult {
        ExecutionResult { name, start, elapsed, cycles }
    }

    fn print_info(&self) {
        let duration = humantime::Duration::from(self.elapsed);
        println!("{} cycles completed in {}", self.cycles, duration)
    }
}

pub struct PipeRunner {
    pub pipe_proc: Child,
}

impl PipeRunner {
    pub fn new(p0: bool) -> PipeRunner {
        // let output_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap());
        // let output_dir = PathBuf::new();
        // let exe = output_dir.join("pipes_consumer.exe");
        #[cfg(debug_assertions)]
        let exe = PathBuf::from("./target/debug/pipes_consumer.exe");
        #[cfg(not(debug_assertions))]
        let exe = PathBuf::from("./target/release/pipes_consumer.exe");
        // let proc = Command::new(exe).stdin()
        PipeRunner { pipe_proc: Command::new(exe).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap() }
    }

    pub fn prepare(&mut self) -> [u8; 5] {
        let mut return_value = [0u8, 0,0,0,0];
        return_value

    }
    pub fn run_inner(&mut self, n: usize, mut return_value: &mut [u8; 5]) {
        if let Some(ref mut pipes_input) = self.pipe_proc.stdin {
           if let Some(ref mut pipes_output) = self.pipe_proc.stdout {
               for _ in 0..n {
                   pipes_input.write(b"ping\n").unwrap();
                   pipes_output.read_exact(return_value).unwrap();
                   if return_value != b"pong\n" {
                       panic!("Unexpected response")
                   }
               }
           }
        }
    }

    pub fn run(&mut self, n: usize) {
        let mut return_buffer = self.prepare();
        let instant = Instant::now();
        self.run_inner(n, &mut return_buffer);
        let elapsed = instant.elapsed();
        let res = ExecutionResult::new("Stdin/stdout".to_string(), instant, elapsed, n);
        // res.print_info()

    }

}

impl Drop for PipeRunner {
    fn drop(&mut self) {
        // println!("Killing subprocess");
        self.pipe_proc.kill().unwrap();
        // stdin().
    }
}
