use crate::bewerb::Bewerb;
use crate::container::{HasId, UidContainer};
use crate::group::{Group, GroupId};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::tournament::{ArenaData, ArenaIdentifier};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
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

#[derive(Debug, Default)]
pub struct ArenaSlot {
    pub id: ArenaSlotId,
    group: Mutex<Option<Arc<Group>>>,
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
            group: arena_slot
                .group
                .lock()
                .unwrap()
                .as_ref()
                .cloned()
                .map(|x| x.id().clone()),
        }
    }
}

impl ArenaSlot {
    pub fn from_arena_slot_saveable(
        as_save_able: ArenaSlotSaveable,
        bewerbs: &mut UidContainer<Bewerb>,
    ) -> Arc<Self> {
        let mut group = None;
        let group_id = as_save_able.group;
        if let Some(group_id) = group_id {
            if let Some(bewerb) = bewerbs.get(group_id.bewerb_id) {
                group = bewerb.get_group_by_id(&group_id);
            }
            if group.is_none() {
                println!("setup of arena_slot group \"{:?}\" not fround", group_id);
            }
        }

        let res = Arc::new(Self {
            id: as_save_able.id,
            group: Mutex::new(None), //TODO
        });

        if let Some(group) = group {
            Group::add_to_arenaslot(group, res.clone());
        }

        res
    }

    pub fn new(id: ArenaSlotId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn id(&self) -> &ArenaSlotId {
        &self.id
    }

    pub fn get_group(&self) -> Option<Arc<Group>> {
        self.group.lock().unwrap().clone()
    }

    pub fn set_group(&self, id: Option<Arc<Group>>) {
        *self.group.lock().unwrap() = id;
    }
}

impl From<&ArenaSlot> for ArenaData {
    fn from(arena: &ArenaSlot) -> Self {
        let id = Some((&arena.id).into());
        let group = arena
            .group
            .lock()
            .unwrap()
            .clone()
            .map(|x| (&x.id()).into());

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
