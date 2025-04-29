use crate::container::HasId;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::arena_slot::ArenaSlotId;
use crate::tournament::GroupIdentifier;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GroupId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
    pub round_id: u32,
    pub group_id: u32,
}

impl From<&GroupId> for GroupIdentifier {
    fn from(id: &GroupId) -> Self {
        Self {
            name: id.bewerb_name.to_owned(),
            bewerb_id: id.bewerb_id,
            round_id: id.round_id,
            group_id: id.group_id,
        }
    }
}

impl From<GroupIdentifier> for GroupId {
    fn from(id: GroupIdentifier) -> Self {
        Self {
            bewerb_name: id.name.to_owned(),
            bewerb_id: id.bewerb_id,
            round_id: id.round_id,
            group_id: id.group_id,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Group {
    id: Mutex<GroupId>,
    arena_slot: Mutex<Option<ArenaSlotId>>,
    fencers: Vec<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GroupSaveable {
    id: GroupId,
    arena_slot: Option<ArenaSlotId>,
    fencers: Vec<u32>,
}

impl From<&Group> for GroupSaveable {
    fn from(group: &Group) -> Self {
        Self {
            id: group.id.lock().unwrap().clone(),
            arena_slot: group.arena_slot.lock().unwrap().clone(),
            fencers: group.fencers.clone(),
        }
    }
}

impl Group {
    pub fn from_saveable(group: &GroupSaveable) -> Self {
        Self {
            id: Mutex::new(group.id.clone()),
            arena_slot: Mutex::new(group.arena_slot.clone()),
            fencers: group.fencers.clone(),
        }
    }

    pub fn new(id: GroupId) -> Self {
        let id = Mutex::new(id);

        Self {
            id,
            arena_slot: Mutex::new(None),
            fencers: Vec::new(),
        }
    }

    pub fn id(&self) -> GroupId {
        self.id.lock().unwrap().clone()
    }

    pub fn get_arena(&self) -> Option<ArenaSlotId> {
        self.arena_slot.lock().unwrap().clone()
    }

    pub fn set_arena(&self, id: Option<ArenaSlotId>) {
        *self.arena_slot.lock().unwrap() = id;
    }
}

impl HasId for Group {
    fn get_id(&self) -> u32 {
        self.id.lock().unwrap().group_id
    }
    fn set_id(&mut self, id: u32) {
        self.id.lock().unwrap().group_id = id;
    }
}
