use lazy_static::lazy_static;
use log::Level;
use serde::Deserialize;

lazy_static! {
  static ref CONFIG: Config = Config::load_config();
}

fn default_operation_port() -> u16 {
  80u16
}

fn default_log_level() -> String {
  "".into()
}

fn default_gauge_data_user() -> String {
  "myuser".into()
}

fn default_gauge_data_password() -> String {
  "mypassword".into()
}

fn default_database_address() -> String {
  "mariadb:3306".into()
}

fn default_database_name() -> String {
  "gauge@gauge".into()
}

fn default_tracing_enabled() -> bool {
  false
}

fn default_tracing_agent_address() -> String {
  "jaeger:6831".into()
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
  #[serde(default = "default_operation_port")]
  operation_port: u16,

  #[serde(default = "default_log_level")]
  log_level: String,

  #[serde(default = "default_gauge_data_user")]
  gauge_data_user: String,

  #[serde(default = "default_gauge_data_password")]
  gauge_data_password: String,

  #[serde(default = "default_database_address")]
  database_address: String,

  #[serde(default = "default_database_name")]
  database_name: String,

  #[serde(default = "default_tracing_enabled")]
  tracing_enabled: bool,

  #[serde(default = "default_tracing_agent_address")]
  tracing_agent_address: String,
}

impl Config {
  pub fn load_config() -> Self {
    match envy::from_env::<Config>() {
      Ok(config) => config,
      Err(e) => {
        panic!("failed to load configuration: {:?}", e);
      }
    }
  }

  pub fn get_log_level() -> Level {
    match CONFIG.log_level.as_str() {
      "trace" => Level::Trace,
      "debug" => Level::Debug,
      "warn" => Level::Warn,
      "error" => Level::Error,
      _ => Level::Info,
    }
  }

  pub fn database_url() -> String {
    format!(
      "mysql://{}:{}@{}/{}",
      CONFIG.gauge_data_user,
      CONFIG.gauge_data_password,
      CONFIG.database_address,
      CONFIG.database_name,
    )
  }

  pub fn operation_port() -> u16 {
    CONFIG.operation_port
  }

  pub fn tracing_enabled() -> bool {
    CONFIG.tracing_enabled
  }

  pub fn tracing_agent_address() -> String {
    CONFIG.tracing_agent_address.clone()
  }
}
