use crate::schema::gauge;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "gauge"]
#[primary_key("name")]
pub struct Gauge {
  pub name: String,
  pub value: i64,
  pub last_modified: NaiveDateTime,
}

#[derive(Insertable, Clone, Serialize, Deserialize, Debug)]
#[table_name = "gauge"]
pub struct NewGauge {
  pub name: String,
  pub value: i64,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "gauge"]
pub struct GaugeUpdate {
  pub value: Option<i64>,
}
