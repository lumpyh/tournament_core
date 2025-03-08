use chrono::{NaiveDate, NaiveDateTime};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};

use crate::tournament_service::tournament::SimpleDay;

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {
    pub id: u32,
    date: NaiveDate,
    n_ts: u32,
    n_kp: u32,
}

impl From<SimpleDay> for Day {
    fn from(sday: SimpleDay) -> Self {
        let date = match sday.date {
            Some(date) => NaiveDateTime::from_timestamp(date.seconds, 0).date(),
            None => NaiveDateTime::from_timestamp(0, 0).date(),
        };

        Day {
            id: 0,
            date,
            n_ts: sday.number_time_slots,
            n_kp: sday.number_arenas,
        }
    }
}

impl Into<SimpleDay> for &Day {
    fn into(self) -> SimpleDay {
        let date = Some(Timestamp {
            seconds: NaiveDateTime::from(self.date).timestamp(),
            nanos: 0,
        });
        SimpleDay {
            id: self.id,
            date,
            number_time_slots: self.n_ts,
            number_arenas: self.n_kp,
        }
    }
}
