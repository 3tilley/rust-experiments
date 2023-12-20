use raw_sync::events::EventInit;
use raw_sync::Timeout;
use shared_memory::{Shmem, ShmemConf};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let handle = &args[1];
    // First byte is the producer busy event, second byte is the consumer busy event, rest is message
    let mut wrapper = ipc::shmem::ShmemWrapper::new(Some(handle.clone()));
    // unsafe {
    //     eprintln!("Con data: {:?}", wrapper.shmem.as_slice());
    // }
    // eprintln!("Con: waiting for event");
    loop {
        if wrapper.their_event.wait(Timeout::Infinite).is_ok() {
            let data = wrapper.read();
            // eprintln!("Con: Found data: {:?}", data);
            if wrapper.read() == b"ping" {
                // eprintln!("Con: Freezing sync");
                wrapper.signal_start();
                // eprintln!("Con: Writing data");
                wrapper.write(b"pong");
                // eprintln!("Con: Releasing sync");
                wrapper.signal_finished();
            } else {
                panic!("Didn't receive ping")
            }
        }
    }
}
