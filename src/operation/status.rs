use axum::{
  body::{boxed, Full},
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub(crate) enum Status {
  Serving,
  NotServing,
}

impl IntoResponse for Status {
  fn into_response(self) -> Response {
    let status: StatusCode = match self {
      Status::Serving => StatusCode::OK,
      Status::NotServing => StatusCode::IM_A_TEAPOT,
    };
    Response::builder()
      .status(status)
      .body(boxed(Full::from(format!("Status: {}", status))))
      .unwrap()
  }
}

// TODO: make a logical status check
pub(crate) async fn status() -> Result<Status, Status> {
  Ok(Status::Serving)
}
