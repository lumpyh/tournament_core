use chrono::{NaiveDate, NaiveDateTime};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};

use crate::arena_slot::{ArenaSlot, ArenaSlotId};
use crate::container::{HasId, UidContainer};
use crate::timeslot::Timeslot;
use crate::tournament::{DayData, SimpleDay};

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
        let mut res = Self {
            date,
            n_ts,
            n_kp,
            ..Default::default()
        };

        for _i in 0..n_ts {
            let ts = Timeslot::new(res.id, n_kp);
            res.timeslots.push(ts);
        }

        res
    }

    pub fn get_arena(&mut self, id: &ArenaSlotId) -> Option<&mut ArenaSlot> {
        let ts = self.timeslots.get_mut(id.timeslot_id)?;

        ts.get_arena(id)
    }
}

impl From<&Day> for DayData {
    fn from(day: &Day) -> Self {
        let timeslots = day.timeslots.iter().map(|x| x.into()).collect();

        Self { timeslots }
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

impl From<&Day> for SimpleDay {
    fn from(val: &Day) -> Self {
        let date = Some(Timestamp {
            seconds: NaiveDateTime::from(val.date).timestamp(),
            nanos: 0,
        });
        SimpleDay {
            id: val.id,
            date,
            number_time_slots: val.n_ts,
            number_arenas: val.n_kp,
        }
    }
}
