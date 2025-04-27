use crate::container::HasId;
use crate::group::GroupId;
use crate::timeslot::TimeslotId;
use serde::{Deserialize, Serialize};

use crate::tournament::{ArenaData, ArenaIdentifier};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ArenaSlotId {
    pub day_id: u32,
    pub timeslot_id: u32,
    pub arena_slot_id: u32,
}

impl From<&ArenaSlotId> for ArenaIdentifier {
    fn from(id: &ArenaSlotId) -> Self {
        Self {
            day_id: id.day_id,
            timeslot_id: id.timeslot_id,
            arena_id: id.arena_slot_id,
        }
    }
}

impl From<ArenaIdentifier> for ArenaSlotId {
    fn from(id: ArenaIdentifier) -> Self {
        Self {
            day_id: id.day_id,
            timeslot_id: id.timeslot_id,
            arena_slot_id: id.arena_id,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ArenaSlot {
    id: ArenaSlotId,
    group: Option<GroupId>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ArenaSlotSaveable {
    id: ArenaSlotId,
    group: Option<GroupId>,
}

impl From<&ArenaSlot> for ArenaSlotSaveable {
    fn from(arena_slot: &ArenaSlot) -> Self {
        Self {
            id: arena_slot.id.clone(),
            group: arena_slot.group.clone(),
        }
    }
}

impl ArenaSlot {
    pub fn from_arena_slot_saveable(as_save_able: ArenaSlotSaveable) -> Self {
        Self {
            id: as_save_able.id,
            group: as_save_able.group,
        }
    }

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

    pub fn get_group(&self) -> Option<&GroupId> {
        self.group.as_ref()
    }

    pub fn set_group(&mut self, id: Option<GroupId>) {
        self.group = id;
    }
}

impl From<&ArenaSlot> for ArenaData {
    fn from(arena: &ArenaSlot) -> Self {
        let id = Some((&arena.id).into());
        let group = arena.group.as_ref().map(|x| x.into());

        Self { id, group }
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
