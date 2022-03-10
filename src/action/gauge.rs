use crate::model::{Gauge, GaugeUpdate, NewGauge};

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{insert_into, update};

pub(crate) fn find_all_gauges(connection: &MysqlConnection) -> Result<Vec<Gauge>, Error> {
  use crate::schema::gauge;
  gauge::table.load::<Gauge>(connection)
}

pub(crate) fn find_gauge_by_name(name: &str, connection: &MysqlConnection) -> Result<Gauge, Error> {
  use crate::schema::gauge;
  gauge::table.find(name).get_result::<Gauge>(connection)
}

pub fn add_new_gauge(gauge_name: String, connection: &MysqlConnection) -> Result<String, Error> {
  use crate::schema::gauge::dsl::*;

  insert_into(gauge)
    .values(vec![NewGauge {
      name: gauge_name.clone(),
      value: 0,
    }])
    .execute(connection)?;

  Ok(gauge_name)
}

pub fn update_gauge_value(
  update_gauge_name: &str,
  gauge_value: i64,
  connection: &MysqlConnection,
) -> Result<i64, Error> {
  use crate::schema::gauge::dsl::*;

  let target = gauge.filter(name.eq(update_gauge_name));
  update(target)
    .set(GaugeUpdate {
      value: Some(gauge_value),
    })
    .execute(connection)?;

  Ok(gauge_value)
}
