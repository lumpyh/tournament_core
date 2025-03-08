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
}

impl HasId for Round {
    fn get_id(&self) -> u32 {
        self.id.round_id
    }
    fn set_id(&mut self, id: u32) {
        self.id.round_id = id;
    }
}
