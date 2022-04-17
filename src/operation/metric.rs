use lazy_static::lazy_static;

use axum::http::{Request, StatusCode};
use opentelemetry::metrics::{Meter, UpDownCounter, ValueRecorder};
use opentelemetry::{global, KeyValue};
use opentelemetry_prometheus::{exporter, PrometheusExporter};

use prometheus::proto::MetricFamily;
use prometheus::{Encoder, TextEncoder};

use crate::action::find_all_gauges;
use axum::extract::MatchedPath;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use diesel::MysqlConnection;
use log::error;
use tokio::time::Instant;

type MetricLabel = &'static str;

const PERSISTENT_GAUGE_NAME: MetricLabel = "persistent_gauge";
const PERSISTENT_GAUGE_NAME_TAG: MetricLabel = "name";
const PERSISTENT_GAUGE_DESCRIPTION: MetricLabel = "Persistent gauge per name.";

const HTTP_REQUESTS_DURATION_SECONDS_NAME: MetricLabel = "http_requests_duration_seconds";
const HTTP_REQUESTS_DURATION_SECONDS_METHOD_TAG: MetricLabel = "method";
const HTTP_REQUESTS_DURATION_SECONDS_PATH_TAG: MetricLabel = "path";
const HTTP_REQUESTS_DURATION_SECONDS_STATUS_TAG: MetricLabel = "status";
const HTTP_REQUESTS_DURATION_SECONDS_DESCRIPTION: MetricLabel =
  "Request duration seconds per method, path, and status.";

lazy_static! {
  static ref METRIC: Metric = Metric::init();
  static ref HISTOGRAM_BOUNDARIES: Vec<f64> =
    vec![0.001, 0.002, 0.003, 0.004, 0.005, 0.006, 0.007, 0.008, 0.009, 0.01];
}

#[derive(Clone)]
pub(crate) struct Metric {
  pub(crate) prometheus_exporter: PrometheusExporter,
  pub(crate) persistent_gauge: UpDownCounter<i64>,
  pub(crate) http_requests_duration_seconds: ValueRecorder<f64>,
}

impl Metric {
  pub(crate) fn init() -> Self {
    let prometheus_exporter: PrometheusExporter = exporter()
      .with_default_histogram_boundaries(HISTOGRAM_BOUNDARIES.clone())
      .init();

    let meter: Meter = global::meter(PERSISTENT_GAUGE_NAME);

    let persistent_gauge: UpDownCounter<i64> = meter
      .i64_up_down_counter(PERSISTENT_GAUGE_NAME)
      .with_description(PERSISTENT_GAUGE_DESCRIPTION)
      .init();

    let http_requests_duration_seconds: ValueRecorder<f64> = meter
      .f64_value_recorder(HTTP_REQUESTS_DURATION_SECONDS_NAME)
      .with_description(HTTP_REQUESTS_DURATION_SECONDS_DESCRIPTION)
      .init();

    Self {
      prometheus_exporter,
      persistent_gauge,
      http_requests_duration_seconds,
    }
  }

  pub(crate) fn increment_gauge(name: &str) {
    METRIC.persistent_gauge.add(
      1,
      &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, name.to_string())],
    );
  }

  pub(crate) fn decrement_gauge(name: &str) {
    METRIC.persistent_gauge.add(
      -1,
      &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, name.to_string())],
    );
  }

  pub(crate) fn create_new_gauge_metric(name: &str) {
    METRIC.persistent_gauge.add(
      0,
      &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, name.to_string())],
    );
  }

  pub(crate) fn report_initial_metrics(connection: &MysqlConnection) {
    match find_all_gauges(connection) {
      Ok(gauges) => gauges.iter().for_each(|g| {
        METRIC.persistent_gauge.add(
          g.value,
          &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, g.name.clone())],
        );
      }),
      Err(e) => panic!("Could not report initial metrics: {:?}", e),
    }
  }

  pub(crate) fn record_http_requests_duration_seconds(
    latency: f64,
    method: String,
    path: String,
    status: String,
  ) {
    let labels: &[KeyValue; 3] = &[
      KeyValue::new(HTTP_REQUESTS_DURATION_SECONDS_METHOD_TAG, method),
      KeyValue::new(HTTP_REQUESTS_DURATION_SECONDS_PATH_TAG, path),
      KeyValue::new(HTTP_REQUESTS_DURATION_SECONDS_STATUS_TAG, status),
    ];

    METRIC
      .http_requests_duration_seconds
      .record(latency, labels);
  }
}

pub(crate) async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
  let start: Instant = Instant::now();
  let path: String = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
    matched_path.as_str().to_owned()
  } else {
    req.uri().path().to_owned()
  };
  let method: String = req.method().clone().to_string();

  let response: Response = next.run(req).await;

  let status: String = response.status().as_u16().to_string();

  let latency: f64 = start.elapsed().as_secs_f64();

  Metric::record_http_requests_duration_seconds(latency, method, path, status);

  response
}

pub(crate) async fn metric_service() -> impl IntoResponse {
  let encoder: TextEncoder = TextEncoder::new();
  let mut buffer: Vec<u8> = Vec::new();
  let metric_family: Vec<MetricFamily> = METRIC.prometheus_exporter.registry().gather();
  match encoder.encode(&metric_family, &mut buffer) {
    Ok(_) => {}
    Err(e) => {
      error!("Failed to encode metrics: {:?}", e);
    }
  };
  (StatusCode::OK, buffer)
}
