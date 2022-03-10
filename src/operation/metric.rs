use axum::http::StatusCode;
use opentelemetry::metrics::{Meter, UpDownCounter};
use opentelemetry::{global, KeyValue};
use opentelemetry_prometheus::{exporter, PrometheusExporter};

use prometheus::proto::MetricFamily;
use prometheus::{Encoder, TextEncoder};

use crate::action::find_all_gauges;
use axum::extract::Extension;
use axum::response::IntoResponse;
use diesel::MysqlConnection;
use log::error;

#[derive(Clone)]
pub struct Metric {
  pub(crate) prometheus_exporter: PrometheusExporter,
  pub(crate) issue_gauge: UpDownCounter<i64>,
}

impl Metric {
  pub fn init() -> Self {
    let prometheus_exporter: PrometheusExporter = exporter().init();
    let meter: Meter = global::meter("persistent_gauge");

    let issue_gauge: UpDownCounter<i64> = meter
      .i64_up_down_counter("issue.gauge")
      .with_description("Number of issues by issue type.")
      .init();

    Self {
      prometheus_exporter,
      issue_gauge,
    }
  }

  pub fn report_initial_metrics(&self, connection: &MysqlConnection) {
    match find_all_gauges(connection) {
      Ok(gauges) => gauges.iter().for_each(|g| {
        self
          .issue_gauge
          .add(g.value, &[KeyValue::new("issue-type", g.name.clone())]);
      }),
      Err(e) => panic!("Could not report initial metrics: {:?}", e),
    }
  }
}

pub async fn metric_service(Extension(metric): Extension<Metric>) -> impl IntoResponse {
  let encoder: TextEncoder = TextEncoder::new();
  let mut buffer: Vec<u8> = Vec::new();
  let metric_family: Vec<MetricFamily> = metric.prometheus_exporter.registry().gather();
  match encoder.encode(&metric_family, &mut buffer) {
    Ok(_) => {}
    Err(e) => {
      error!("Failed to encode metrics: {:?}", e);
    }
  };
  (StatusCode::OK, buffer)
}
