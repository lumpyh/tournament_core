use crate::container::HasId;
use crate::container::UidContainer;
use crate::group::Group;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Round {
    id: u32,
    groups: UidContainer<Group>,
}

impl Round {
    pub fn new(n_groups: u32) -> Self {
        let mut res = Self::default();
        for _i in 0..n_groups {
            let group = Group::default();
            res.groups.push(group);
        }
        res
    }
}

impl HasId for Round {
    fn get_id(&self) -> u32 {
        self.id
    }
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
