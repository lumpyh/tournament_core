use crate::arena_slot::{ArenaSlot, ArenaSlotId, ArenaSlotSaveable};
use crate::container::{HasId, UidContainer};
use crate::tournament;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TimeslotId {
    pub day_id: u32,
    pub timeslot_id: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Timeslot {
    id: TimeslotId,
    arenas: UidContainer<ArenaSlot>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TimeslotSaveable {
    id: TimeslotId,
    arenas: Vec<ArenaSlotSaveable>,
}

impl From<&Timeslot> for TimeslotSaveable {
    fn from(ts: &Timeslot) -> Self {
        let arenas = ts.arenas.iter().map(|x| x.into()).collect();

        Self {
            id: ts.id.clone(),
            arenas,
        }
    }
}

impl Timeslot {
    pub fn from_timeslot_saveable(ts_saveables: TimeslotSaveable) -> Self {
        let mut arenas: UidContainer<ArenaSlot> = Default::default();
        for ts_saveable in ts_saveables.arenas.iter() {
            arenas.insert(ArenaSlot::from_arena_slot_saveable(ts_saveable.clone()));
        }

        Self {
            id: ts_saveables.id,
            arenas,
        }
    }

    pub fn new(day_id: u32, n_kp: u32) -> Self {
        let mut res = Self::default();
        res.id.day_id = day_id;

        for _i in 0..n_kp {
            let arena = ArenaSlot::new(res.id.clone());
            res.arenas.push(arena);
        }
        res
    }

    pub fn set_day_id(&mut self, id: u32) {
        self.id.day_id = id;
        for arena in self.arenas.iter_mut() {
            arena.set_day_id(id);
        }
    }

    pub fn get_arena(&mut self, id: &ArenaSlotId) -> Option<&mut ArenaSlot> {
        self.arenas.get_mut(id.arena_slot_id)
    }
}

impl From<&Timeslot> for tournament::TimeslotData {
    fn from(ts: &Timeslot) -> Self {
        let arenas = ts.arenas.iter().map(|x| x.into()).collect();

        Self { arenas }
    }
}

impl HasId for Timeslot {
    fn get_id(&self) -> u32 {
        self.id.timeslot_id
    }

    fn set_id(&mut self, id: u32) {
        self.id.timeslot_id = id;
        for arena in self.arenas.iter_mut() {
            arena.set_timeslot_id(id);
        }
    }
}
