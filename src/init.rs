use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::Tracer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::{EnvFilter, fmt, Registry};

//this is new
use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;

pub(crate) fn initialize_tracing_subscriber() {
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(construct_open_telemetry_layer())
        .with(fmt::layer().pretty());

    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set up global logger");
}

fn construct_open_telemetry_layer(
) -> OpenTelemetryLayer<Layered<EnvFilter, Registry, Registry>, Tracer> {
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("Example Application")
        .install_batch(Tokio) //export traces in batches with Tokio
        .expect("Failed to install OpenTelemetry tracer.");

    //this is new
    //use W3 standard for context propagation in open telemetry
    global::set_text_map_propagator(TraceContextPropagator::new());

    OpenTelemetryLayer::new(tracer)
}