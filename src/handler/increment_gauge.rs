use crate::action::{find_gauge_by_name, update_gauge_value};
use crate::model::GaugeUpdate;
use crate::{DbPool, Metric};
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::{debug, error};
use opentelemetry::KeyValue;

pub async fn increment_gauge(
  Path(gauge_name): Path<String>,
  Extension(metric): Extension<Metric>,
  Extension(pool): Extension<DbPool>,
) -> impl IntoResponse {
  let connection = pool
    .get()
    .expect("Couldn't get database connection from pool!");

  match find_gauge_by_name(gauge_name.clone(), &connection) {
    Ok(gauge) => {
      debug!("Incrementing gauge: {:?}", gauge_name);
      let gauge_update = GaugeUpdate {
        value: Some(gauge.value + 1),
      };
      match update_gauge_value(gauge_name.clone(), gauge_update, &connection) {
        Ok(Some(GaugeUpdate { value: Some(v) })) => {
          metric
            .issue_gauge
            .add(1, &[KeyValue::new("issue-type", gauge_name)]);
          (StatusCode::OK, v.to_string())
        }
        _ => {
          let error = format!("Failed to decrement gauge: {:?}", gauge_name);
          error!("{}", error);
          (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
      }
    }
    Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
  }
}
