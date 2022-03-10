use axum::http::StatusCode;
use axum::{
  body,
  response::{IntoResponse, Response},
};
use log::debug;

type Name = String;

pub(crate) enum GaugeResponse {
  Incremented(i64),
  Decremented(i64),
  Created(Name),
}

impl IntoResponse for GaugeResponse {
  fn into_response(self) -> Response {
    let body = match self {
      GaugeResponse::Incremented(new_gauge_value) | GaugeResponse::Decremented(new_gauge_value) => {
        debug!("New gauge value: {}", new_gauge_value);
        body::boxed(body::Full::from(new_gauge_value.to_string()))
      }
      GaugeResponse::Created(name) => {
        debug!("Gauge created. Name: {}", name);
        body::boxed(body::Full::from(name))
      }
    };
    Response::builder()
      .status(StatusCode::OK)
      .body(body)
      .unwrap()
  }
}
