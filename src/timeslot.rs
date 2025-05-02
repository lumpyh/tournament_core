use crate::arena_slot::{ArenaSlot, ArenaSlotId, ArenaSlotSaveable};
use crate::bewerb::Bewerb;
use crate::container::{HasId, UidContainer};
use crate::tournament;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TimeslotId {
    pub day_id: u32,
    pub timeslot_id: u32,
}

#[derive(Debug, Default)]
pub struct Timeslot {
    pub id: TimeslotId,
    arenas: Vec<Arc<ArenaSlot>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TimeslotSaveable {
    id: TimeslotId,
    arenas: Vec<ArenaSlotSaveable>,
}

impl From<&Timeslot> for TimeslotSaveable {
    fn from(ts: &Timeslot) -> Self {
        let arenas = ts.arenas.iter().map(|x| x.as_ref().into()).collect();

        Self {
            id: ts.id.clone(),
            arenas,
        }
    }
}

impl Timeslot {
    pub fn from_timeslot_saveable(
        ts_saveables: TimeslotSaveable,
        bewerbs: &mut UidContainer<Bewerb>,
    ) -> Self {
        let mut arenas: Vec<Arc<ArenaSlot>> = Default::default();
        for ts_saveable in ts_saveables.arenas.iter() {
            arenas.push(ArenaSlot::from_arena_slot_saveable(
                ts_saveable.clone(),
                bewerbs,
            ));
        }

        Self {
            id: ts_saveables.id,
            arenas,
        }
    }

    pub fn new(id: TimeslotId, n_kp: u32) -> Self {
        let mut res = Self {
            id,
            ..Default::default()
        };

        for i in 0..n_kp {
            let id = ArenaSlotId {
                day_id: res.id.day_id,
                timeslot_id: res.id.timeslot_id,
                arena_slot_id: i,
            };

            let arena = ArenaSlot::new(id);
            res.arenas.push(Arc::new(arena));
        }
        res
    }

    pub fn get_arena(&self, id: &ArenaSlotId) -> Option<Arc<ArenaSlot>> {
        self.arenas.iter().find(|x| x.id() == id).cloned()
    }
}

impl From<&Timeslot> for tournament::TimeslotData {
    fn from(ts: &Timeslot) -> Self {
        let arenas = ts.arenas.iter().map(|x| x.as_ref().into()).collect();

        Self { arenas }
    }
}

impl HasId for Timeslot {
    fn get_id(&self) -> u32 {
        self.id.timeslot_id
    }

    fn set_id(&mut self, id: u32) {
        self.id.timeslot_id = id;
    }
}
