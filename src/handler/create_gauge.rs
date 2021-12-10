use crate::action::{add_new_gauge, find_gauge_by_name};
use crate::model::NewGauge;
use crate::{DbPool, Metric};
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use diesel::result::Error;
use log::{debug, error};
use opentelemetry::KeyValue;

pub async fn create_gauge(
  Path(gauge_name): Path<String>,
  Extension(metric): Extension<Metric>,
  Extension(pool): Extension<DbPool>,
) -> impl IntoResponse {
  let connection = pool
    .get()
    .expect("Couldn't get database connection from pool!");

  match find_gauge_by_name(gauge_name.clone(), &connection) {
    Ok(_) => {
      let error = format!(
        "Cannot create a gauge that already exists: {:?}",
        gauge_name
      );
      debug!("{}", error);
      (StatusCode::CONFLICT, error)
    }
    Err(e) => match e {
      Error::NotFound => {
        debug!("Creating gauge: {:?}", gauge_name);
        match add_new_gauge(
          NewGauge {
            name: gauge_name.clone(),
            value: 0,
          },
          &connection,
        ) {
          Ok(Some(_)) => {
            debug!("Created new gauge: {:?}", gauge_name);
            metric
              .issue_gauge
              .add(0, &[KeyValue::new("issue-type", gauge_name.clone())]);
            (StatusCode::OK, gauge_name)
          }
          _ => {
            let error = format!("Failed to create new gauge: {:?}", gauge_name);
            error!("{}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, error)
          }
        }
      }
      _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    },
  }
}
