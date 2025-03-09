use chrono::{NaiveDate, NaiveDateTime};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};

use crate::container::{HasId, UidContainer};
use crate::timeslot::Timeslot;
use crate::tournament_service::tournament::SimpleDay;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Day {
    pub id: u32,
    date: NaiveDate,
    n_ts: u32,
    n_kp: u32,
    timeslots: UidContainer<Timeslot>,
}

impl Day {
    pub fn new(date: NaiveDate, n_ts: u32, n_kp: u32) -> Self {
        let mut res = Self::default();
        res.date = date;
        res.n_ts = n_ts;
        res.n_kp = n_kp;

        for _i in 0..n_ts {
            let ts = Timeslot::new(res.id, n_kp);
            res.timeslots.push(ts);
        }

        res
    }
}

impl HasId for Day {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
        for ts in self.timeslots.iter_mut() {
            ts.set_day_id(id);
        }
    }
}

impl From<SimpleDay> for Day {
    fn from(sday: SimpleDay) -> Self {
        let date = match sday.date {
            Some(date) => NaiveDateTime::from_timestamp(date.seconds, 0).date(),
            None => NaiveDateTime::from_timestamp(0, 0).date(),
        };

        Self::new(date, sday.number_time_slots, sday.number_arenas)
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
