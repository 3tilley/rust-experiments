
fn main() {
    let mut arr = [0u8, 0, 0, 0, 0];
    let wrapper = ipc::iceoryx::IceoryxWrapper::new(false);
    loop {
        while let Some(recv_payload) = wrapper.subscriber.receive().unwrap() {
            if !recv_payload.eq(b"ping") {
                panic!("Received unexpected payload")
            }
        }

        let sample = wrapper.publisher.loan_uninit().unwrap();
        let payload = sample.write_payload(*b"pong");
        payload.send().unwrap();
    }
}
