use opentelemetry::{global, KeyValue};
use std::{error::Error, thread, time::Duration};
use std::sync::{Arc, Mutex, Weak};
// use opentelemetry_sdk::
use opentelemetry_sdk::metrics::data::{ResourceMetrics, Temporality};
use opentelemetry_sdk::metrics::{Aggregation, InstrumentKind, ManualReader, MeterProvider, PeriodicReader, Pipeline};
use opentelemetry_sdk::metrics::reader::{AggregationSelector, MetricReader, TemporalitySelector};
use opentelemetry_sdk::{Resource, runtime};
use tracing::{Instrument, span, trace, warn};
use tracing_subscriber::prelude::*;

mod work;

fn init_meter_provider() -> MeterProvider {
    // let exporter = opentelemetry_stdout::MetricsExporterBuilder::default()
    //     // uncomment the below lines to pretty print output.
    //     //  .with_encoder(|writer, data|
    //     //    Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
    //     .build();
    // let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();

    let exporter = opentelemetry_sdk::testing::metrics::in_memory_exporter::InMemoryMetricsExporterBuilder::new();

    let reader = opentelemetry_sdk::metrics::ManualReader::builder();
    let out_reader = reader.build();
    MeterProvider::builder()
        .with_reader(out_reader)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "metrics-basic-example",
        )]))
        .build()
}

#[derive(Debug)]
pub struct MyExporter {
    reader: Arc<ManualReader>,
}

impl TemporalitySelector for MyExporter {
    /// Note: Prometheus only supports cumulative temporality so this will always be
    /// [Temporality::Cumulative].
    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.reader.temporality(kind)
    }
}

impl AggregationSelector for MyExporter {
    fn aggregation(&self, kind: InstrumentKind) -> Aggregation {
        self.reader.aggregation(kind)
    }
}
impl MetricReader for MyExporter {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.reader.register_pipeline(pipeline)
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> opentelemetry::metrics::Result<()> {
        self.reader.collect(rm)
    }

    fn force_flush(&self) -> opentelemetry::metrics::Result<()> {
        self.reader.force_flush()
    }

    fn shutdown(&self) -> opentelemetry::metrics::Result<()> {
        self.reader.shutdown()
    }
}

fn init_manual_reader() -> (Arc<ManualReader>, MeterProvider) {
    let reader = Arc::new(ManualReader::builder().build());
    let exporter = MyExporter { reader: Arc::clone(&reader) };
    // let pipeline = opentelemetry_sdk::metrics::pipeline::Pipeline
    let provider = MeterProvider::builder()
        .with_reader(exporter)
        // .with_resource(Resource::new(vec![KeyValue::new(
        //     "service.name",
        //     "metrics-basic-example",
        // )]))
        .build();
    (Arc::clone(&reader), provider)
}

fn read_manually(reader: &ManualReader) {

    let mut metrics = ResourceMetrics {
        resource: Resource::empty(),
        scope_metrics: vec![],
    };
    reader.collect(&mut metrics).expect("Unable to read metrics");
    println!("Resources: {:?}", metrics.resource.len());
    for resource in metrics.resource.iter() {
        println!("Resource: {:?}", resource);
    }
    println!("Scopes: {}", metrics.scope_metrics.len());

    for scope in metrics.scope_metrics.iter() {
        println!("Scope: {:?}", scope);

        for inner_metrics in scope.metrics.iter() {
            println!("Metrics: {:?}", inner_metrics);
        }
    }

    println!("\n{:?}", metrics);
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

    let (reader, provider) = init_manual_reader();
    // let provider = init_meter_provider();
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
    let clone_reader = Arc::clone(&reader);
    read_manually(&clone_reader);
    // provider.

    Ok(())
}
