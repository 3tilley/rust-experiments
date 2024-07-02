use std::process::{Child, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};
use iceoryx2::port::publisher::Publisher;
use iceoryx2::port::subscriber::Subscriber;
use iceoryx2::prelude::*;
use crate::ExecutionResult;

pub struct IceoryxWrapper {
    pub publisher: Publisher<zero_copy::Service, [u8; 4]>,
    pub subscriber: Subscriber<zero_copy::Service, [u8; 4]>,
}

impl IceoryxWrapper {
    pub fn new(is_producer: bool) -> IceoryxWrapper {
        const PRODUCER_SEND: &'static str = "ipc/Producer/Send";
        const CONSUMER_SEND: &'static str = "ipc/Consumer/Send";
        let (publisher, subscriber) = if is_producer {
            let send_name = ServiceName::new(PRODUCER_SEND).unwrap();
            let recv_name = ServiceName::new(CONSUMER_SEND).unwrap();
            let send_service = zero_copy::Service::new(&send_name).publish_subscribe().create().unwrap();
            let recv_service = zero_copy::Service::new(&recv_name).publish_subscribe().create().unwrap();

            let services = zero_copy::Service::list().unwrap();
            println!("\nProd - Services\n");
            for service in services {
                println!("\n{:#?}", &service);
            }

            (send_service.publisher().create().unwrap(), recv_service.subscriber().create().unwrap())

        } else {
            let send_name = ServiceName::new(CONSUMER_SEND).unwrap();
            let recv_name = ServiceName::new(PRODUCER_SEND).unwrap();

            let services = zero_copy::Service::list().unwrap();
            println!("\nCon - Services\n");
            for service in services {
                println!("\n{:#?}", &service);
            }

            let send_service = zero_copy::Service::new(&send_name).publish_subscribe().open().unwrap();
            let recv_service = zero_copy::Service::new(&recv_name).publish_subscribe().open().unwrap();
            (send_service.publisher().create().unwrap(), recv_service.subscriber().create().unwrap())
        };

        IceoryxWrapper {
            publisher,
            subscriber,
        }
    }
}

pub struct IceoryxRunner {
    child_proc: Option<Child>,
    wrapper: IceoryxWrapper,
}

impl IceoryxRunner {
    pub fn new(start_child: bool) -> IceoryxRunner {
        // let start_child = false;
        let wrapper = IceoryxWrapper::new(true);
        sleep(Duration::from_millis(1000));
        let exe = crate::executable_path("iceoryx_consumer");
        let child_proc = if start_child {
            Some(
                Command::new(exe)
                    .spawn()
                    .unwrap()
            )
        } else {
            None
        };
        // Awkward sleep again to wait for consumer to be ready
        sleep(Duration::from_millis(1000));
        Self {
            child_proc,
            wrapper
        }

    }

    pub fn run(&mut self, n: usize, print: bool) {
        let start = Instant::now();
        // self.wrapper.subscriber.
        for _ in 0..n {
            let sample = self.wrapper.publisher.loan_uninit().unwrap();
            let send_payload = sample.write_payload((*b"ping").into());
            send_payload.send().unwrap();

            while let Some(recv_payload) = self.wrapper.subscriber.receive().unwrap() {
                if !recv_payload.eq(b"pong") {
                    panic!("Received unexpected payload")
                }
                println!("Received {recv_payload:?}");
            }
        }
        if print {
            let elapsed = start.elapsed();
            let res = ExecutionResult::new("Iceoryx".to_string(), elapsed, n);
            res.print_info();
        }

    }
}

impl Drop for IceoryxRunner {
    fn drop(&mut self) {
        if let Some(ref mut c) = self.child_proc {
            c.kill().unwrap();
        }
    }
}