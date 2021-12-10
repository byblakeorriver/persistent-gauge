use crate::operation::{metric_service, status, Config, Logger, Metric};
use axum::handler::{get, post};
use axum::{AddExtensionLayer, Router, Server};
use std::error::Error;

use handler::*;

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

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub async fn start() -> Result<(), Box<dyn Error>> {
  let conf: Config = Config::load_config();
  let logger: Logger = Logger::init_logger(&conf);
  let metric: Metric = Metric::init();

  let _ = slog_scope::set_global_logger(logger.root_logger).cancel_reset();

  let database_manager: ConnectionManager<MysqlConnection> =
    ConnectionManager::<MysqlConnection>::new(conf.database_url());
  let pool: DbPool = Pool::new(database_manager).expect("Failed to create pool!");

  metric.report_initial_metrics(&pool.get().expect("Failed to report initial metrics!"));

  let app = Router::new()
    .route("/metrics", get(metric_service))
    .route("/status", get(status))
    .route("/api/gauge/increment/:gauge_name", post(increment_gauge))
    .route("/api/gauge/decrement/:gauge_name", post(decrement_gauge))
    .route("/api/gauge/create/:gauge_name", post(create_gauge))
    .layer(AddExtensionLayer::new(pool.clone()))
    .layer(AddExtensionLayer::new(metric.clone()))
    .layer(AddExtensionLayer::new(conf.clone()));

  let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], conf.operation_port));
  info!(
    "*** persistent gauge service started, listening on port {} ***",
    &conf.operation_port
  );

  let server = Server::bind(&addr).serve(app.into_make_service());

  match tokio::try_join!(tokio::spawn(server)) {
    Ok(_) => Ok(()),
    Err(e) => panic!("The server failed: {:?}", e),
  }
}
