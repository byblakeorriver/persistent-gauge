use crate::model::{Gauge, GaugeUpdate, NewGauge};

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{insert_into, update};

pub fn find_gauge_by_name(name: String, connection: &MysqlConnection) -> Result<Gauge, Error> {
  use crate::schema::gauge;

  match gauge::table.find(name).get_result::<Gauge>(connection) {
    Ok(gauge) => Ok(gauge),
    Err(e) => Err(e),
  }
}

pub fn add_new_gauge(
  new_gauge: NewGauge,
  connection: &MysqlConnection,
) -> Result<Option<NewGauge>, Error> {
  use crate::schema::gauge::dsl::*;

  let ng: NewGauge = new_gauge.clone();

  let inserted_gauge: usize = insert_into(gauge).values(vec![ng]).execute(connection)?;

  if inserted_gauge == 1 {
    Ok(Some(new_gauge))
  } else {
    Ok(None)
  }
}

pub fn update_gauge_value(
  update_gauge_name: String,
  gauge_update: GaugeUpdate,
  connection: &MysqlConnection,
) -> Result<Option<GaugeUpdate>, Error> {
  use crate::schema::gauge::dsl::*;

  let target = gauge.filter(name.eq(update_gauge_name));
  let updated_gauge: usize = update(target)
    .set(gauge_update.clone())
    .execute(connection)?;

  if updated_gauge == 1 {
    Ok(Some(gauge_update))
  } else {
    Ok(None)
  }
}
