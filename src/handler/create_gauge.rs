use crate::action::{add_new_gauge, find_gauge_by_name};
use crate::model::{GaugeError, GaugeResponse};
use crate::{DbPool, Metric};
use axum::extract::{Extension, Path};
use axum::Json;
use diesel::result::Error;

pub(crate) async fn create_gauge(
  Path(gauge_name): Path<String>,
  Extension(metric): Extension<Metric>,
  Extension(pool): Extension<DbPool>,
) -> Result<Json<GaugeResponse>, GaugeError> {
  let connection = pool.get()?;

  match find_gauge_by_name(&gauge_name, &connection) {
    Ok(_) => Err(GaugeError::AlreadyExists(gauge_name)),
    Err(e) => match e {
      Error::NotFound => match add_new_gauge(gauge_name, &connection) {
        Ok(gauge_name) => {
          metric.create_new_gauge_metric(&gauge_name);
          Ok(Json(GaugeResponse::Created(gauge_name)))
        }
        Err(e) => Err(GaugeError::FailedToCreate(e.to_string())),
      },
      _ => Err(GaugeError::FailedToCreate(e.to_string())),
    },
  }
}
