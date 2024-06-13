use raw_sync::events::EventInit;
use raw_sync::Timeout;
use shared_memory::{Shmem, ShmemConf};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let handle = &args[1];
    // First two bytes is the producer busy event, second two bytes is the consumer busy event.
    // The rest is our message
    let mut wrapper = ipc::shmem::ShmemWrapper::new(Some(handle.clone()));
    loop {
        if wrapper.their_event.wait(Timeout::Infinite).is_ok() {
            let data = wrapper.read();
            if wrapper.read() == b"ping" {
                wrapper.signal_start();
                wrapper.write(b"pong");
                wrapper.signal_finished();
            } else {
                panic!("Didn't receive ping")
            }
        }
    }
}
