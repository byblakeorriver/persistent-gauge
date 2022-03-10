use crate::action::{find_gauge_by_name, update_gauge_value};
use crate::model::{GaugeError, GaugeResponse};
use crate::{DbPool, Metric};
use axum::extract::{Extension, Path};

pub(crate) async fn decrement_gauge(
  Path(gauge_name): Path<String>,
  Extension(metric): Extension<Metric>,
  Extension(pool): Extension<DbPool>,
) -> Result<GaugeResponse, GaugeError> {
  let connection = pool.get()?;

  match find_gauge_by_name(&gauge_name, &connection) {
    Ok(gauge) => match update_gauge_value(&gauge_name, gauge.value - 1, &connection) {
      Ok(v) => {
        metric.decrement_gauge(&gauge_name);
        Ok(GaugeResponse::Decremented(v))
      }
      Err(e) => Err(GaugeError::FailedToDecrement(e.to_string())),
    },
    Err(e) => Err(GaugeError::NotFound(e.to_string())),
  }
}
