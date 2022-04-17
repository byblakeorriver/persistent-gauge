use crate::Config;
use axum::body::Body;
use axum::http::Request;
use log::{error, info};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::trace::{Span, TraceError, Tracer as OtelTracer};
use opentelemetry::{global, Context, KeyValue};
use opentelemetry_http::HeaderExtractor;
use std::task::Poll;
use tower::Service;

const SERVICE_NAME: &str = "persistent-gauge";
const ROOT_TRACER_NAME: &str = "persistent-gauge-root";
const REQUEST_SPAN_NAME: &str = "persistent-gauge-request";
const HTTP_METHOD_KEY: &str = "app.http.method";
const HTTP_PATH_KEY: &str = "app.path";

#[derive(Debug)]
pub(crate) struct Tracer;

impl Tracer {
  pub(crate) fn init() -> Result<(), TraceError> {
    if Config::tracing_enabled() {
      global::set_text_map_propagator(TraceContextPropagator::new());

      match opentelemetry_jaeger::new_pipeline()
        .with_trace_config(
          opentelemetry::sdk::trace::config()
            .with_sampler(opentelemetry::sdk::trace::Sampler::TraceIdRatioBased(0.2)),
        )
        .with_agent_endpoint(Config::tracing_agent_address())
        .with_service_name(SERVICE_NAME)
        .install_batch(opentelemetry::runtime::Tokio)
      {
        Ok(tracer) => {
          info!("Tracing enabled.");
          tracer
        }
        Err(e) => {
          error!("Error when creating tracer: {:?}", e);
          return Err(e);
        }
      };
      Ok(())
    } else {
      info!("Tracing disabled.");
      Ok(())
    }
  }
}

pub struct TraceIngestLayer;

impl TraceIngestLayer {
  pub fn new() -> Self {
    Self {}
  }
}

impl<S> tower::Layer<S> for TraceIngestLayer {
  type Service = TraceIngest<S>;

  fn layer(&self, inner: S) -> Self::Service {
    TraceIngest::new(inner)
  }
}

#[derive(Clone, Copy)]
pub struct TraceIngest<S> {
  inner: S,
}

impl<S> TraceIngest<S> {
  pub fn new(inner: S) -> Self {
    Self { inner }
  }
}

impl<S> Service<Request<Body>> for TraceIngest<S>
where
  S: tower::Service<Request<Body>> + Clone + Send + 'static,
  S::Future: Send,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = S::Future;

  fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
    std::task::Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: Request<Body>) -> Self::Future {
    if Config::tracing_enabled() {
      let parent_cx = get_parent_context(&req);

      let mut span = opentelemetry::global::tracer(ROOT_TRACER_NAME)
        .start_with_context(REQUEST_SPAN_NAME, &parent_cx);
      span.set_attribute(KeyValue::new(HTTP_METHOD_KEY, req.method().to_string()));
      span.set_attribute(KeyValue::new(HTTP_PATH_KEY, req.uri().path().to_string()));
    }

    self.inner.call(req)
  }
}

fn get_parent_context(req: &Request<Body>) -> Context {
  opentelemetry::global::get_text_map_propagator(|propogator| {
    propogator.extract(&HeaderExtractor(req.headers()))
  })
}
