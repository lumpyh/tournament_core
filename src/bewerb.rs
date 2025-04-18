use crate::container::{HasId, UidContainer};
use crate::group::{Group, GroupId};
use crate::round::Round;
use serde::{Deserialize, Serialize};

use crate::tournament::{BewerbIdentifier, SimpleBewerbData};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BewerbId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
}

impl From<&BewerbIdentifier> for BewerbId {
    fn from(id: &BewerbIdentifier) -> Self{
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Bewerb {
    id: BewerbId,
    n_rounds: u32,
    n_groups: u32,
    rounds: UidContainer<Round>,
}

impl Bewerb {
    pub fn new(name: String, n_rounds: u32, n_groups: u32) -> Self {
        let mut res = Bewerb {
            id: BewerbId {
                bewerb_id: 0,
                bewerb_name: name.clone(),
            },
            n_rounds,
            n_groups,
            rounds: UidContainer::default(),
        };

        for _i in 0..n_rounds {
            let round = Round::new(&res.id, n_groups);
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

    pub fn get_group_by_id(&mut self, id: &GroupId) -> Option<&mut Group> {
        let round = self.rounds.get_mut(id.round_id)?;
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
        for round in self.rounds.iter_mut() {
            round.set_bewerb_id(id);
        }
    }
}
