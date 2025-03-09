use crate::container::HasId;
use crate::round::RoundId;
use serde::{Deserialize, Serialize};

use crate::arena_slot::ArenaSlotId;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GroupId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
    pub round_id: u32,
    pub group_id: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Group {
    id: GroupId,
    arena_slot: Option<ArenaSlotId>,
    fencers: Vec<u32>,
}

impl Group {
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

    pub fn set_arena(&mut self, id: ArenaSlotId) {
        self.arena_slot = Some(id);
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
