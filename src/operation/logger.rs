use std::io::Stdout;

use crate::operation::Config;

use slog::{o, Drain, Fuse, Logger as SLogger};
use slog_async::Async;
use slog_json::Json;

#[derive(Clone)]
pub struct Logger {
  pub root_logger: SLogger,
}

impl Logger {
  pub fn init_logger(config: &Config) -> Self {
    let json: Fuse<Json<Stdout>> = Json::new(std::io::stdout())
      .add_default_keys()
      .build()
      .fuse();

    let drain: Fuse<Async> = Async::default(json).fuse();

    let root_logger: SLogger = SLogger::root(drain, o!("application_name" => "persistent-gauge"));

    // unwrap ok, because I want the application to panic if the logging cannot be initiated
    slog_stdlog::init_with_level(config.get_log_level()).unwrap();

    Self { root_logger }
  }
}
