use opentelemetry::{global, KeyValue, runtime};
use std::{error::Error, thread, time::Duration};
use std::sync::Mutex;
use opentelemetry::sdk::metrics::{MeterProvider, PeriodicReader};
use opentelemetry::sdk::Resource;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::{ManualReader, Pipeline};
use opentelemetry_sdk::metrics::reader::MetricReader;
use tracing::{Instrument, span, trace, warn};
use tracing_subscriber::prelude::*;

mod work;

fn init_meter_provider() -> MeterProvider {
    let exporter = opentelemetry_stdout::MetricsExporterBuilder::default()
        // uncomment the below lines to pretty print output.
        //  .with_encoder(|writer, data|
        //    Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
        .build();

    let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();

    // let reader = opentelemetry::sdk::metrics::ManualReader::builder().build();
    MeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "metrics-basic-example",
        )]))
        .build()
}

// fn init_manual_reader() -> (ManualReader, MeterProvider) {
//     let reader = ManualReader::builder().build();
//     let pipeline = opentelemetry_sdk::metrics::pipeline::Pipeline
//     let provider = MeterProvider::builder()
//         .with_reader(&reader)
//         .with_resource(Resource::new(vec![KeyValue::new(
//             "service.name",
//             "metrics-basic-example",
//         )]))
//         .build();
//     (reader, provider)
// }

fn read_manually(reader: ManualReader) {

    let mut metrics = ResourceMetrics {
        resource: Resource::empty(),
        scope_metrics: vec![],
    };
    reader.collect(&mut metrics).expect("Unable to read metrics");
    println!("{:?}", metrics);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Install an otel pipeline with a simple span processor that exports data one at a time when
    // spans end. See the `install_batch` option on each exporter's pipeline builder to see how to
    // export in batches.
    // let tracer = opentelemetry_jaeger::new_agent_pipeline()
    //     .with_service_name("report_example")
    //     .install_simple()?;
    // let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    // let (reader, provider) = init_manual_reader();
    let provider = init_meter_provider();
    global::set_meter_provider(provider);

    {
        let root = span!(tracing::Level::INFO, "app_start", work_units = 2);
        let _enter = root.enter();

        let work_result = work::expensive_work();

        span!(tracing::Level::INFO, "faster_work")
            .in_scope(|| thread::sleep(Duration::from_millis(10)));

        warn!("About to exit!");
        trace!("status: {}", work_result);
    } // Once this scope is closed, all spans inside are closed as well

    // Shut down the current tracer provider. This will invoke the shutdown
    // method on all span processors. span processors should export remaining
    // spans before return.
    global::shutdown_tracer_provider();
    // read_manually(reader);
    // provider.

    Ok(())
}
