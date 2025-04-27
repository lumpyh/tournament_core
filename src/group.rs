use crate::container::HasId;
use crate::round::RoundId;
use serde::{Deserialize, Serialize};

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
    id: GroupId,
    arena_slot: Option<ArenaSlotId>,
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
            id: group.id.clone(),
            arena_slot: group.arena_slot.clone(),
            fencers: group.fencers.clone(),
        }
    }
}

impl Group {
    pub fn from_saveable(group: &GroupSaveable) -> Self {
        Self {
            id: group.id.clone(),
            arena_slot: group.arena_slot.clone(),
            fencers: group.fencers.clone(),
        }
    }

    pub fn new(id: &RoundId) -> Self {
        let mut res = Self::default();
        res.id.bewerb_name = id.bewerb_name.clone();
        res.id.bewerb_id = id.bewerb_id;
        res.id.round_id = id.round_id;
        res
    }

    pub fn id(&self) -> &GroupId {
        &self.id
    }

    pub fn set_bewerb_id(&mut self, id: u32) {
        self.id.bewerb_id = id;
    }

    pub fn set_round_id(&mut self, id: u32) {
        self.id.round_id = id;
    }

    pub fn get_arena(&self) -> &Option<ArenaSlotId> {
        &self.arena_slot
    }

    pub fn set_arena(&mut self, id: Option<ArenaSlotId>) {
        self.arena_slot = id;
    }
}

impl HasId for Group {
    fn get_id(&self) -> u32 {
        self.id.group_id
    }
    fn set_id(&mut self, id: u32) {
        self.id.group_id = id;
    }
}
