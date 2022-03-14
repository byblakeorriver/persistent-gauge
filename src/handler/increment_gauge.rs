use crate::action::{find_gauge_by_name, update_gauge_value};
use crate::model::{GaugeError, GaugeResponse};
use crate::{DbPool, Metric};
use axum::extract::{Extension, Path};
use axum::Json;

pub(crate) async fn increment_gauge(
  Path(gauge_name): Path<String>,
  Extension(metric): Extension<Metric>,
  Extension(pool): Extension<DbPool>,
) -> Result<Json<GaugeResponse>, GaugeError> {
  let connection = pool.get()?;

  match find_gauge_by_name(&gauge_name, &connection) {
    Ok(gauge) => match update_gauge_value(&gauge_name, gauge.value + 1, &connection) {
      Ok(v) => {
        metric.increment_gauge(&gauge_name);
        Ok(Json(GaugeResponse::Incremented(v)))
      }
      Err(e) => Err(GaugeError::FailedToIncrement(e.to_string())),
    },
    Err(e) => Err(GaugeError::NotFound(e.to_string())),
  }
}
