use crate::bewerb::BewerbId;
use crate::container::HasId;
use crate::container::UidContainer;
use crate::group::{Group, GroupId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RoundId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
    pub round_id: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Round {
    id: RoundId,
    groups: UidContainer<Group>,
}

impl Round {
    pub fn new(bewerb_id: &BewerbId, n_groups: u32) -> Self {
        let mut res = Self::default();
        res.id.bewerb_name = bewerb_id.bewerb_name.clone();
        res.id.bewerb_id = bewerb_id.bewerb_id;
        for _i in 0..n_groups {
            let group = Group::new(&res.id);
            res.groups.push(group);
        }
        res
    }

    pub fn get_all_groups(&self) -> Vec<GroupId> {
        self.groups.iter().map(|x| (*x.id()).clone()).collect()
    }

    pub fn get_free_groups(&self) -> Vec<GroupId> {
        self.groups
            .iter()
            .filter(|x| x.get_arena().is_none())
            .map(|x| (*x.id()).clone())
            .collect()
    }

    pub fn set_bewerb_id(&mut self, id: u32) {
        self.id.bewerb_id = id;
        for group in self.groups.iter_mut() {
            group.set_bewerb_id(id);
        }
    }

    pub fn get_group_by_id(&mut self, id: &GroupId) -> Option<&mut Group> {
        self.groups.get_mut(id.group_id)
    }
}

impl HasId for Round {
    fn get_id(&self) -> u32 {
        self.id.round_id
    }
    fn set_id(&mut self, id: u32) {
        self.id.round_id = id;
        for group in self.groups.iter_mut() {
            group.set_round_id(id);
        }
    }
}
