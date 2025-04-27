use crate::bewerb::BewerbId;
use crate::container::HasId;
use crate::group::{Group, GroupId, GroupSaveable};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoundId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
    pub round_id: u32,
}

#[derive(Debug, Default)]
pub struct Round {
    id: RoundId,
    groups: Vec<Arc<Group>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RoundSaveable {
    id: RoundId,
    groups: Vec<GroupSaveable>,
}

impl From<&Round> for RoundSaveable {
    fn from(round: &Round) -> Self {
        let groups = round.groups.iter().map(|x| x.as_ref().into()).collect();
        Self {
            id: round.id.clone(),
            groups,
        }
    }
}

impl Round {
    pub fn from_saveable(round: &RoundSaveable) -> Self {
        let mut groups: Vec<Arc<Group>> = Default::default();
        for group in &round.groups {
            groups.push(Arc::new(Group::from_saveable(group)));
        }

        Self {
            id: round.id.clone(),
            groups,
        }
    }

    pub fn new(bewerb_id: &BewerbId, n_groups: u32, round_id: u32) -> Self {
        let mut res = Self::default();
        res.id.bewerb_name = bewerb_id.bewerb_name.clone();
        res.id.bewerb_id = bewerb_id.bewerb_id;
        res.id.round_id = round_id;
        for i in 0..n_groups {
            let id = GroupId {
                bewerb_name: bewerb_id.bewerb_name.clone(),
                bewerb_id: bewerb_id.bewerb_id,
                round_id,
                group_id: i,
            };

            let group = Group::new(id);
            res.groups.push(Arc::new(group));
        }
        res
    }

    pub fn get_all_groups(&self) -> Vec<GroupId> {
        self.groups.iter().map(|x| x.id()).collect()
    }

    pub fn get_free_groups(&self) -> Vec<GroupId> {
        self.groups
            .iter()
            .filter(|x| x.get_arena().is_none())
            .map(|x| x.id())
            .collect()
    }

    pub fn set_bewerb_id(&mut self, id: u32) {
        self.id.bewerb_id = id;
        for group in self.groups.iter_mut() {
            group.set_bewerb_id(id);
        }
    }

    pub fn get_group_by_id(&mut self, id: &GroupId) -> Option<Arc<Group>> {
        self.groups
            .iter_mut()
            .find(|x| x.id().group_id == id.group_id)
            .cloned()
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
