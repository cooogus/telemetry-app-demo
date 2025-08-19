use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{MetricExporter, Protocol, WithExportConfig}; // <-- add this
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider},
    Resource,
};
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("recogni-app-sim tick...");
    // Resource becomes Prom labels via collector's resource_to_telemetry flag
    // Describes the entity emitting telemetry (service name + attributes).
    // Because Collector Prom exporter has resource_to_telemetry: true,
    // these become Prometheus labels (e.g., deployment_environment="prod")
    let resource = Resource::builder()
        .with_service_name("recogni-app-sim")
        .with_attributes(vec![
            KeyValue::new("deployment.environment", "prod"),
            KeyValue::new("region", "us-west"),
            KeyValue::new("site", "sjc1"),
            KeyValue::new("cluster", "cluster-0"),
            KeyValue::new("node", "node-0"),
            KeyValue::new("chip", "chip-0"),
        ])
        .build();

    // OTLP metric exporter Collector gRPC (otel-collector:4317)
    // how to send, every 5s
    let exporter = MetricExporter::builder()
        .with_tonic()
        .with_endpoint("http://otel-collector:4317")
        .with_protocol(Protocol::Grpc)
        .with_timeout(Duration::from_secs(5))
        .build()?;

    // send metrics every 5 seconds
    let reader = PeriodicReader::builder(exporter)
        .with_interval(Duration::from_secs(5))
        .build();

    // Meter provider
    // owns all metric instruments
    let provider = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_reader(reader)
        .build();

    // creates instrumentation scope -> the app
    global::set_meter_provider(provider.clone());
    let meter = global::meter("recogni.app");

    // what we're measuring (instruments)
    // latency + chip temp

    // synchronous instruments -> call record() in every observation
    let latency = meter
        .f64_histogram("recogni.inference.latency_ms")
        .with_unit("ms")
        .with_description("E2E inference latency")
        .build();

    // Common attributes
    let common = [KeyValue::new("service", "inference-api")];

    // Shared state for the gauge to read
    let temp_state = Arc::new(Mutex::new(72.0_f64));
    let temp_state_cb = temp_state.clone();

    // Build the observable gauge: chip temp
    // asynchronous instrument -> dont push values directly
    // have to register a callback and sdk invokes it on every collection (every 5s)
    let _temp_gauge = meter
        .f64_observable_gauge("recogni.chip.temperature_c")
        .with_unit("C")
        .with_description("Chip temperature")
        .with_callback(move |observer| {
            if let Ok(t) = temp_state_cb.lock() {
                observer.observe(*t, &[KeyValue::new("service", "inference-api")]);
            }
        })
        .build();

    println!("recogni-app-sim: sending fake metrics to otel-collector:4317 ...");

    // fake data loop
    let mut rng = rand::rng();
    loop {
        println!("recogni-app-sim: starting long-running loop");
        // Fake latency
        let latency_ms: f64 = 35.0 + rng.random_range(-8.0..8.0);
        latency.record(f64::max(latency_ms, 0.5), &common);

        // Update absolute temperature for the gauge callback to read
        let new_temp = 72.0 + rng.random_range(-3.0..3.0);
        if let Ok(mut t) = temp_state.lock() {
            *t = new_temp;
        }
        println!("recogni-app-sim tick...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    // since we want it to keep generating things forever (until we run docker compose stop)
    // we comment these lines out

    // provider.shutdown()?;
    // Ok(())
}
