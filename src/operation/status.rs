use axum::body::{Bytes, Full};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use std::convert::Infallible;

#[derive(Serialize)]
struct Status {
  healthy: bool,
}

impl IntoResponse for Status {
  type Body = Full<Bytes>;
  type BodyError = Infallible;

  fn into_response(self) -> Response<Self::Body> {
    (StatusCode::OK, Json(self)).into_response()
  }
}

// TODO: make a logical status check
pub async fn status() -> impl IntoResponse {
  Status { healthy: true }.into_response()
}
