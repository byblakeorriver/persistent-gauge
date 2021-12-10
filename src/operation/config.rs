use log::Level;
use serde::Deserialize;

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

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
  #[serde(default = "default_operation_port")]
  pub operation_port: u16,

  #[serde(default = "default_log_level")]
  pub log_level: String,

  #[serde(default = "default_gauge_data_user")]
  pub gauge_data_user: String,

  #[serde(default = "default_gauge_data_password")]
  pub gauge_data_password: String,

  #[serde(default = "default_database_address")]
  pub database_address: String,
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

  pub fn get_log_level(&self) -> Level {
    match self.log_level.clone().as_str() {
      "trace" => Level::Trace,
      "debug" => Level::Debug,
      "warn" => Level::Warn,
      "error" => Level::Error,
      _ => Level::Info,
    }
  }

  pub fn database_url(&self) -> String {
    format!(
      "mysql://{}:{}@{}/gauge@gauge",
      self.gauge_data_user, self.gauge_data_password, self.database_address
    )
  }
}
