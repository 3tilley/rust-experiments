use ipc::iceoryx::IceoryxWrapper;

fn main() {
    let wrapper = ipc::iceoryx::IceoryxWrapper::new(false);
    IceoryxWrapper::print_services(false);
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
