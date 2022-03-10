use crate::operation::{metric_service, status, Config, Logger, Metric};
use axum::routing::{get, post};
use axum::{Router, Server};
use std::error::Error;

use handler::*;

use axum::extract::Extension;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use log::info;
use r2d2::Pool;
use std::net::SocketAddr;

mod action;
mod handler;
mod model;
mod operation;

#[rustfmt::skip] // because this is generated code.
mod schema;

#[macro_use]
extern crate diesel;
extern crate lazy_static;

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub async fn start() -> Result<(), Box<dyn Error>> {
  let logger: Logger = Logger::init_logger();
  let metric: Metric = Metric::init();

  let _ = slog_scope::set_global_logger(logger.root_logger).cancel_reset();

  let database_manager: ConnectionManager<MysqlConnection> =
    ConnectionManager::<MysqlConnection>::new(Config::database_url());
  let pool: DbPool = Pool::new(database_manager).expect("Failed to create pool!");

  metric.report_initial_metrics(&pool.get().expect("Failed to report initial metrics!"));

  let app = Router::new()
    .route("/metrics", get(metric_service))
    .route("/status", get(status))
    .route("/api/gauge/increment/:gauge_name", post(increment_gauge))
    .route("/api/gauge/decrement/:gauge_name", post(decrement_gauge))
    .route("/api/gauge/create/:gauge_name", post(create_gauge))
    .layer(Extension(pool.clone()))
    .layer(Extension(metric.clone()));

  let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], Config::operation_port()));
  info!(
    "*** persistent gauge service started, listening on port {} ***",
    Config::operation_port()
  );

  let server = Server::bind(&addr).serve(app.into_make_service());

  match tokio::try_join!(tokio::spawn(server)) {
    Ok(_) => Ok(()),
    Err(e) => panic!("The server failed: {:?}", e),
  }
}
