use raw_sync::events::EventInit;
use raw_sync::Timeout;
use shared_memory::{Shmem, ShmemConf};
use std::fmt::format;
use std::io::{Read, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let our_port = u16::from_str(&args[2]).unwrap();
    let their_port = u16::from_str(&args[1]).unwrap();
    let mut wrapper = ipc::udp::UdpStreamWrapper::from_port(our_port);
    wrapper
        .socket
        .connect(format!("127.0.0.1:{}", their_port))
        .unwrap();
    let mut buf = [0u8; 4];
    loop {
        wrapper.socket.recv(&mut buf).unwrap();
        if buf.eq(b"ping") {
            wrapper.socket.send(b"pong").unwrap();
        } else {
            panic!("Received unknown value {:?}", buf)
        }
    }
}
