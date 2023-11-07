use tracing::{error, event, info_span, instrument, span, Level};
use opentelemetry_api::{global, KeyValue};
use std::thread;
use std::time::Duration;

#[instrument]
pub fn expensive_work() -> &'static str {
    // Setup meters and counters
    let meter = global::meter("mylib");
    let counter_step_1 = meter.u64_counter("step_1").with_description("Calls to expensive step 1").init();
    let counter_step_2 = meter.u64_counter("step_2").init();

    counter_step_1.add(1u64, &[]);
    span!(Level::INFO, "expensive_step_1")
        .in_scope(|| thread::sleep(Duration::from_millis(25)));

    // Short form of the span above
    info_span!("expensive_step_2")
        .in_scope(|| {
            thread::sleep(Duration::from_millis(25));
            let pid = std::process::id();
            match pid % 8 {
                x if (0..=3).contains(&x) => {
                    event!(Level::INFO, name = "success", result = x);
                    counter_step_2.add(1, [
                        KeyValue::new("result", "success")
                    ].as_ref());
                    "Service call succeeded"
                },
                x => {
                    // Shorthand for event!(tracing::Level::ERROR, "failure", result=x);
                    error!(name = "failure", result = x);
                    counter_step_2.add(1, [
                        KeyValue::new("result", "failure")
                    ].as_ref());
                    "Service call failed"
                },
            }
        })
}
