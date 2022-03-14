use axum::http::StatusCode;
use axum::{
  body::{boxed, Full},
  response::{IntoResponse, Response},
};
use log::error;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) enum GaugeError {
  NotFound(String),
  AlreadyExists(String),
  FailedToDecrement(String),
  FailedToIncrement(String),
  FailedToCreate(String),
  FailedToGet(String),
  R2D2Error(String),
}

impl IntoResponse for GaugeError {
  fn into_response(self) -> Response {
    let (status_code, body) = match self {
      GaugeError::NotFound(error) => {
        let msg: String = format!("The gauge was not found in the database. Error: {}", error);
        error!("{}", msg);
        (StatusCode::NOT_FOUND, boxed(Full::from(msg)))
      }
      GaugeError::AlreadyExists(name) => {
        let msg: String = format!("The gauge you tried to create with name {} already exists. Try creating one with a different name.", name);
        error!("{}", msg);
        (StatusCode::CONFLICT, boxed(Full::from(msg)))
      }
      GaugeError::FailedToDecrement(error) => {
        let msg: String = format!("Failed to decrement the gauge. Error: {}", error);
        error!("{}", msg);
        (StatusCode::INTERNAL_SERVER_ERROR, boxed(Full::from(msg)))
      }
      GaugeError::FailedToIncrement(error) => {
        let msg: String = format!("Failed to increment the gauge. Error: {}", error);
        error!("{}", msg);
        (StatusCode::INTERNAL_SERVER_ERROR, boxed(Full::from(msg)))
      }
      GaugeError::FailedToCreate(error) => {
        let msg: String = format!("Failed to create gauge. Error: {}", error);
        error!("{}", msg);
        (StatusCode::INTERNAL_SERVER_ERROR, boxed(Full::from(msg)))
      }
      GaugeError::FailedToGet(error) => {
        let msg: String = format!("Failed to get gauges. Error: {}", error);
        error!("{}", msg);
        (StatusCode::INTERNAL_SERVER_ERROR, boxed(Full::from(msg)))
      }
      GaugeError::R2D2Error(error) => {
        let msg: String = format!("Error communicating with database. Error: {}", error);
        error!("{}", msg);
        (StatusCode::INTERNAL_SERVER_ERROR, boxed(Full::from(msg)))
      }
    };

    Response::builder().status(status_code).body(body).unwrap()
  }
}

impl From<r2d2::Error> for GaugeError {
  fn from(e: r2d2::Error) -> Self {
    GaugeError::R2D2Error(e.to_string())
  }
}

impl Display for GaugeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      GaugeError::NotFound(msg)
      | GaugeError::AlreadyExists(msg)
      | GaugeError::FailedToDecrement(msg)
      | GaugeError::FailedToIncrement(msg)
      | GaugeError::FailedToCreate(msg)
      | GaugeError::FailedToGet(msg)
      | GaugeError::R2D2Error(msg) => {
        write!(f, "{}", msg)
      }
    }
  }
}

impl Error for GaugeError {}
