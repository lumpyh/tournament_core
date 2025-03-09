use serde::{Deserialize, Serialize};
use crate::container::HasId;
use crate::timeslot::TimeslotId;
use crate::group::GroupId;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ArenaSlotId {
    pub day_id: u32,
    pub timeslot_id: u32,
    pub arena_slot_id: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ArenaSlot {
    id: ArenaSlotId,
    group: Option<GroupId>,
}

impl ArenaSlot {
    pub fn new(ts: TimeslotId) -> Self {
        let mut res = Self::default();
        res.id.day_id = ts.day_id;
        res.id.timeslot_id = ts.timeslot_id;
        res
    }

    pub fn set_timeslot_id(&mut self, id: u32) {
        self.id.timeslot_id = id;
    }

    pub fn set_day_id(&mut self, id: u32) {
        self.id.day_id = id;
    }
}


impl HasId for ArenaSlot {
    fn get_id(&self) -> u32 {
        self.id.arena_slot_id
    }

    fn set_id(&mut self, id: u32) {
        self.id.arena_slot_id = id;
    }
}

