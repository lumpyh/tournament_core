use chrono::{NaiveDate, NaiveDateTime};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::arena_slot::{ArenaSlot, ArenaSlotId};
use crate::container::HasId;
use crate::timeslot::{Timeslot, TimeslotId, TimeslotSaveable};
use crate::tournament::{DayData, SimpleDay};

#[derive(Default, Debug)]
pub struct Day {
    pub id: u32,
    date: NaiveDate,
    n_ts: u32,
    n_kp: u32,
    timeslots: Vec<Timeslot>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct DaySaveable {
    pub id: u32,
    date: NaiveDate,
    n_ts: u32,
    n_kp: u32,
    timeslots: Vec<TimeslotSaveable>,
}

impl From<&Day> for DaySaveable {
    fn from(day: &Day) -> Self {
        let timeslots = day.timeslots.iter().map(|x| x.into()).collect();

        Self {
            id: day.id,
            date: day.date,
            n_ts: day.n_ts,
            n_kp: day.n_kp,
            timeslots,
        }
    }
}

impl Day {
    pub fn from_saveable(day: DaySaveable) -> Self {
        let mut timeslots: Vec<Timeslot> = Default::default();
        for ts in day.timeslots.iter() {
            timeslots.push(Timeslot::from_timeslot_saveable(ts.clone()));
        }

        Self {
            id: day.id,
            date: day.date,
            n_ts: day.n_ts,
            n_kp: day.n_kp,
            timeslots,
        }
    }

    pub fn new(id: u32, date: NaiveDate, n_ts: u32, n_kp: u32) -> Self {
        let mut res = Self {
            id,
            date,
            n_ts,
            n_kp,
            ..Default::default()
        };

        for i in 0..n_ts {
            let tid = TimeslotId {
                day_id: res.id,
                timeslot_id: i,
            };

            let ts = Timeslot::new(tid, n_kp);
            res.timeslots.push(ts);
        }
        res
    }

    pub fn get_arena(&mut self, id: &ArenaSlotId) -> Option<Arc<ArenaSlot>> {
        let ts = self
            .timeslots
            .iter()
            .find(|x| x.id.timeslot_id == id.timeslot_id)?;

        ts.get_arena(id)
    }

    pub fn from(id: u32, sday: SimpleDay) -> Self {
        let date = match sday.date {
            Some(date) => NaiveDateTime::from_timestamp(date.seconds, 0).date(),
            None => NaiveDateTime::from_timestamp(0, 0).date(),
        };

        Self::new(id, date, sday.number_time_slots, sday.number_arenas)
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
