use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use std::io::{Read, stdin, Write};
use raw_sync::events::{BusyEvent, EventImpl, EventInit, EventState};
use raw_sync::Timeout;
use shared_memory::{Shmem, ShmemConf};

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

fn shmem_conf() -> ShmemConf {
    let shmem =  ShmemConf::new().size(6);
    shmem
}

pub struct ShmemWrapper {
    pub shmem: Shmem,
    pub owner: bool,
    pub our_event: Box<dyn EventImpl>,
    pub their_event: Box<dyn EventImpl>,
}

impl ShmemWrapper {
    pub fn new(handle: Option<String>) -> ShmemWrapper {
        let owner = handle.is_none();
        let (mut shmem, this_event_index) =
            match handle {
                None => (shmem_conf().create().unwrap(), 0),
                Some(h) => (shmem_conf().os_id(&h).open().expect(&format!("Unable to open the shared memory at {}", h)), 1),
            };
        let mut bytes = unsafe { shmem.as_slice_mut() };
        let (our_event, _) = unsafe { BusyEvent::new(bytes.get_mut(this_event_index).unwrap(), false).unwrap() };
        let (their_event, _) = unsafe { BusyEvent::new(bytes.get_mut(1 - this_event_index).unwrap(), false).unwrap() };
        if owner {
            our_event.set(EventState::Clear).unwrap();
            their_event.set(EventState::Clear).unwrap();
        }
        ShmemWrapper {
            shmem,
            owner,
            our_event,
            their_event,
        }
    }

    pub fn signal_start(&mut self) {
        self.our_event.set(EventState::Clear).unwrap()
    }
    pub fn signal_finished(&mut self) {
        self.our_event.set(EventState::Signaled).unwrap()
    }

    pub fn write(&mut self, data: &[u8; 4]) {
        let mut bytes = unsafe { self.shmem.as_slice_mut() };

        for i in 0..data.len() {
            bytes[i + 2] = data[i];
        }
    }

    pub fn read(&self) -> &[u8] {
        unsafe { &self.shmem.as_slice()[2..6] }
    }
}

// #[derive(Debug)]
pub struct ShmemRunner {
    pub child_proc: Option<Child>,
    pub wrapper: ShmemWrapper,
}

impl ShmemRunner {
    pub fn new(start_child: bool) -> ShmemRunner {
        let wrapper = ShmemWrapper::new(None);

        let id = wrapper.shmem.get_os_id();
        let exe = executable_path("shmem_consumer");
        let child_proc = if start_child {
            Some(Command::new(exe).args(&[id]).spawn().unwrap())
        } else {
            None
        };
        ShmemRunner { child_proc, wrapper }
    }

    pub fn run(&mut self, n: usize) {
        eprintln!("Prod: Mem handle - {}", self.wrapper.shmem.get_os_id());
        unsafe { eprintln!("Prod data: {:?}", self.wrapper.shmem.as_slice()); }
        eprintln!("Prod: Freezing sync");
        self.wrapper.signal_start();
        eprintln!("Prod: Writing data");
        self.wrapper.write(b"ping");
        eprintln!("Prod: Releasing sync");
        self.wrapper.signal_finished();
        unsafe { eprintln!("Prod data: {:?}", self.wrapper.shmem.as_slice()); }
        unsafe {
            if self.wrapper.their_event.wait(Timeout::Infinite).is_ok() {
                eprintln!("Prod: received event");
                let str = self.wrapper.shmem.as_slice_mut();
                eprintln!("Prod: received data: {:?}", str)
            }
        }
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
        let exe = executable_path("pipes_consumer");
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
