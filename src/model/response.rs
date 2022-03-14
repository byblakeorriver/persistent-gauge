use crate::model::Gauge;
use serde::{Deserialize, Serialize};

type Name = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum GaugeResponse {
  Incremented(i64),
  Decremented(i64),
  Created(Name),
  Gauges(Vec<Gauge>),
}
