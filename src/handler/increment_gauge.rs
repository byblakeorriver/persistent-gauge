use crate::action::{find_gauge_by_name, update_gauge_value};
use crate::model::{GaugeError, GaugeResponse};
use crate::{DbPool, Metric};
use axum::extract::{Extension, Path};
use opentelemetry::KeyValue;

pub(crate) async fn increment_gauge(
  Path(gauge_name): Path<String>,
  Extension(metric): Extension<Metric>,
  Extension(pool): Extension<DbPool>,
) -> Result<GaugeResponse, GaugeError> {
  let connection = pool.get()?;

  match find_gauge_by_name(&gauge_name, &connection) {
    Ok(gauge) => match update_gauge_value(&gauge_name, gauge.value + 1, &connection) {
      Ok(v) => {
        metric
          .persistent_gauge
          .add(1, &[KeyValue::new("issue-type", gauge_name)]);
        Ok(GaugeResponse::Incremented(v))
      }
      Err(e) => Err(GaugeError::FailedToIncrement(e.to_string())),
    },
    Err(e) => Err(GaugeError::NotFound(e.to_string())),
  }
}
