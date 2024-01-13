use opentelemetry::trace::{Span as _, TraceResult};
use opentelemetry::{Context, KeyValue};
use opentelemetry_sdk::export::trace::SpanData;
use opentelemetry_sdk::trace::{Span, SpanProcessor};
use std::fmt::Debug;

#[derive(Debug)]
pub struct CustomSpanProcessor {}

impl CustomSpanProcessor {
    pub fn new() -> Self {
        CustomSpanProcessor {}
    }
}

const GCP_SERVICE_NAME_ATTRIBUTE: &str = "service.name";

const INTEGRATION_ENGINE_SERVICE_NAME: &str = "Example Application";

impl SpanProcessor for CustomSpanProcessor {
    fn on_start(&self, span: &mut Span, _cx: &Context) {
        span.set_attribute(KeyValue::new(
            GCP_SERVICE_NAME_ATTRIBUTE,
            INTEGRATION_ENGINE_SERVICE_NAME,
        ));
    }

    fn on_end(&self, _span: SpanData) {}

    fn force_flush(&self) -> TraceResult<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> TraceResult<()> {
        Ok(())
    }
}
