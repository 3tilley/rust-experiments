use raw_sync::events::EventInit;
use raw_sync::Timeout;
use shared_memory::{Shmem, ShmemConf};
use std::io::{Read, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let port = u16::from_str(&args[1]).unwrap();
    let nodelay = bool::from_str(&args[2]).unwrap();
    let mut wrapper = ipc::tcp::TcpStreamWrapper::from_port(port, nodelay);
    let mut buf = [0u8; 4];
    while let Ok(_) = wrapper.stream.read(&mut buf) {
        if buf.eq(b"ping") {
            wrapper.stream.write(b"pong").unwrap();
        } else {
            panic!("Received unknown value {:?}", buf)
        }
    }
}
