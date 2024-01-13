//a lot of the imports have changed
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{Config, Sampler, TracerProvider};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::{EnvFilter, Registry};
use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing_stackdriver::CloudTraceConfiguration;
use crate::custom_span_processor::CustomSpanProcessor;

//this function is now async
pub(crate) async fn initialize_tracing_subscriber() {
    //this is the layer that used to be jaeger, now it's Stackdriver
    //it exports the traces/spans to GCP
    let otel_layer = construct_open_telemetry_layer().await;

    // used to export logs in gcp compatible format
    // make sure to replace with your own project id
    let stackdriver_layer =
        tracing_stackdriver::layer().with_cloud_trace(CloudTraceConfiguration {
            project_id: "otel-tutorial-project".to_string(),
        });

    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(otel_layer)
        .with(stackdriver_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set up global logger");
}

//the entire body of this function has changed
//
//we are now exporting traces to GCP Trace Explorer
//instead of Jaeger
async fn construct_open_telemetry_layer(
) -> OpenTelemetryLayer<Layered<EnvFilter, Registry, Registry>, opentelemetry_sdk::trace::Tracer> {
    //when running inside gcp we don't need authentication
    let authorizer = opentelemetry_stackdriver::GcpAuthorizer::new()
        .await
        .expect("Failed to create GCP authorizer.");

    //the tracer is the same trait we had with jaeger exporting
    //
    //driver is the future that we need to run that will export
    //all the trace batches in the background
    let (stackdriver_tracer, driver) = opentelemetry_stackdriver::Builder::default()
        .build(authorizer)
        .await
        .expect("Failed to create Stackdriver tracer.");

    //we need to explicitly spawn the fiber that will export batches of traces to GCP
    //
    //internally it blocks on the channel receiver so the fiber will complete
    //when the sender is dropped but we should still join the returned handle
    //on shutdown
    //we're skipping that here to reduce the boilerplate code
    tokio::spawn(driver);

    let provider = TracerProvider::builder()
        .with_batch_exporter(stackdriver_tracer, Tokio)
        .with_config(Config {
            //we're using ParentBased sampling, which means that
            //we'll respect whatever whoever called us decided
            //when it comes to sampling a specific request
            //
            //in case there is no decision by the parent span, we're sampling
            //with ratio 1.0 (100%) so we're recording all the requests
            sampler: Box::new(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                1.0,
            )))),
            ..Default::default()
        })
        .with_span_processor(CustomSpanProcessor::new())
        .build();

    let tracer = provider.tracer("Example application");

    //install the tracer provider globally (this was done under the hood for us when we were using jaeger)
    global::set_tracer_provider(provider);

    //use W3 standard for context propagation in open telemetry
    global::set_text_map_propagator(TraceContextPropagator::new());

    OpenTelemetryLayer::new(tracer)
}