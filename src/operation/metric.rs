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

const PERSISTENT_GAUGE_NAME: &str = "persistent_gauge";
const PERSISTENT_GAUGE_NAME_TAG: &str = "name";
const PERSISTENT_GAUGE_DESCRIPTION: &str = "Persistent gauge per name.";

#[derive(Clone)]
pub(crate) struct Metric {
  pub(crate) prometheus_exporter: PrometheusExporter,
  pub(crate) persistent_gauge: UpDownCounter<i64>,
}

impl Metric {
  pub(crate) fn init() -> Self {
    let prometheus_exporter: PrometheusExporter = exporter().init();
    let meter: Meter = global::meter(PERSISTENT_GAUGE_NAME);

    let persistent_gauge: UpDownCounter<i64> = meter
      .i64_up_down_counter(PERSISTENT_GAUGE_NAME)
      .with_description(PERSISTENT_GAUGE_DESCRIPTION)
      .init();

    Self {
      prometheus_exporter,
      persistent_gauge,
    }
  }

  pub(crate) fn increment_gauge(&self, name: &str) {
    self.persistent_gauge.add(
      1,
      &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, name.to_string())],
    );
  }

  pub(crate) fn decrement_gauge(&self, name: &str) {
    self.persistent_gauge.add(
      -1,
      &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, name.to_string())],
    );
  }

  pub(crate) fn create_new_gauge_metric(&self, name: &str) {
    self.persistent_gauge.add(
      0,
      &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, name.to_string())],
    );
  }

  pub(crate) fn report_initial_metrics(&self, connection: &MysqlConnection) {
    match find_all_gauges(connection) {
      Ok(gauges) => gauges.iter().for_each(|g| {
        self.persistent_gauge.add(
          g.value,
          &[KeyValue::new(PERSISTENT_GAUGE_NAME_TAG, g.name.clone())],
        );
      }),
      Err(e) => panic!("Could not report initial metrics: {:?}", e),
    }
  }
}

pub(crate) async fn metric_service(Extension(metric): Extension<Metric>) -> impl IntoResponse {
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
