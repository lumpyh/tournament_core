use crate::container::{HasId, UidContainer};
use crate::group::{Group, GroupId};
use crate::round::{Round, RoundSaveable};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::tournament::{BewerbIdentifier, SimpleBewerbData};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BewerbId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
}

impl From<&BewerbIdentifier> for BewerbId {
    fn from(id: &BewerbIdentifier) -> Self {
        Self {
            bewerb_name: id.name.clone(),
            bewerb_id: id.id,
        }
    }
}

impl From<&BewerbId> for BewerbIdentifier {
    fn from(id: &BewerbId) -> Self {
        Self {
            id: id.bewerb_id,
            name: id.bewerb_name.to_owned(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Bewerb {
    id: BewerbId,
    pub n_rounds: u32,
    n_groups: u32,
    rounds: UidContainer<Round>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BewerbSaveable {
    id: BewerbId,
    n_rounds: u32,
    n_groups: u32,
    rounds: Vec<RoundSaveable>,
}

impl From<&Bewerb> for BewerbSaveable {
    fn from(bewerb: &Bewerb) -> Self {
        let rounds = bewerb.rounds.iter().map(|x| x.into()).collect();

        Self {
            id: bewerb.id.clone(),
            n_rounds: bewerb.n_rounds,
            n_groups: bewerb.n_groups,
            rounds,
        }
    }
}

impl Bewerb {
    pub fn from_saveable(bewerb: &BewerbSaveable) -> Self {
        let mut rounds: UidContainer<Round> = Default::default();
        for round in bewerb.rounds.iter() {
            rounds.insert(Round::from_saveable(round));
        }

        Self {
            id: bewerb.id.clone(),
            n_rounds: bewerb.n_rounds,
            n_groups: bewerb.n_groups,
            rounds,
        }
    }

    pub fn new(bewerb_id: u32, name: String, n_rounds: u32, n_groups: u32) -> Self {
        let mut res = Bewerb {
            id: BewerbId {
                bewerb_id,
                bewerb_name: name.clone(),
            },
            n_rounds,
            n_groups,
            rounds: UidContainer::default(),
        };

        for i in 0..n_rounds {
            let round = Round::new(&res.id, n_groups, i);
            res.rounds.push(round);
        }

        res
    }

    pub fn get_all_groups(&self) -> Vec<GroupId> {
        let mut res = Vec::new();

        for round in self.rounds.iter() {
            let mut round_groups = round.get_all_groups();
            res.append(&mut round_groups);
        }
        res
    }

    pub fn get_free_groups(&self) -> Vec<GroupId> {
        let mut res = Vec::new();

        for round in self.rounds.iter() {
            let mut round_groups = round.get_free_groups();
            res.append(&mut round_groups);
        }
        res
    }

    pub fn get_group_by_id(&self, id: &GroupId) -> Option<Arc<Group>> {
        let round = self.rounds.get(id.round_id)?;
        round.get_group_by_id(id)
    }
}

impl From<&Bewerb> for SimpleBewerbData {
    fn from(bewerb: &Bewerb) -> Self {
        Self {
            id: bewerb.id.bewerb_id,
            name: bewerb.id.bewerb_name.to_owned(),
            n_rounds: bewerb.n_rounds,
            n_groups: bewerb.n_groups,
        }
    }
}

impl HasId for Bewerb {
    fn get_id(&self) -> u32 {
        self.id.bewerb_id
    }
    fn set_id(&mut self, id: u32) {
        self.id.bewerb_id = id;
    }
}
