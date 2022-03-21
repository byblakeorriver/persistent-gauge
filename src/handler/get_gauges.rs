use crate::action::find_all_gauges;
use crate::model::{GaugeError, GaugeResponse};
use crate::DbPool;
use axum::extract::Extension;
use axum::Json;

pub(crate) async fn get_gauges(
  Extension(pool): Extension<DbPool>,
) -> Result<Json<GaugeResponse>, GaugeError> {
  let connection = pool.get()?;

  match find_all_gauges(&connection) {
    Ok(gauges) => Ok(Json(GaugeResponse::Gauges(gauges))),
    Err(e) => Err(GaugeError::FailedToGet(e.to_string())),
  }
}
