use std::time::Duration;
use raw_sync::events::EventInit;
use raw_sync::Timeout;
use shared_memory::{Shmem, ShmemConf};

fn main() {
    let mut args : Vec<String> = std::env::args().collect();
    eprintln!("Con: args {:?}", args);
    let handle = &args[1];
    eprintln!("Con: {:?}", handle);
    // First byte is the producer busy event, second byte is the consumer busy event, rest is message
    let mut wrapper = ipc::ShmemWrapper::new(Some(handle.clone()));
    eprintln!("Con: Mem handle - {}", wrapper.shmem.get_os_id());
    unsafe { eprintln!("Con data: {:?}", wrapper.shmem.as_slice()); }
    eprintln!("Con: waiting for event");
    if wrapper.their_event.wait(Timeout::Infinite).is_ok() {
        let data = wrapper.read();
        eprintln!("Con: Found data: {:?}", data);
        if wrapper.read() == b"ping" {
            eprintln!("Con: Freezing sync");
            wrapper.signal_start();
            eprintln!("Con: Writing data");
            wrapper.write(b"pong");
            eprintln!("Con: Releasing sync");
            wrapper.signal_finished();
        }
    }

}