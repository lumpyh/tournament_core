use crate::container::{HasId, UidContainer};
use crate::group::{Group, GroupId};
use crate::round::Round;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BewerbId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Bewerb {
    id: BewerbId,
    rounds: UidContainer<Round>,
}

impl Bewerb {
    pub fn new(name: String, n_rounds: u32, n_groups: u32) -> Self {
        let mut res = Bewerb {
            id: BewerbId {
                bewerb_id: 0,
                bewerb_name: name.clone(),
            },
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

    pub fn get_group_by_id(&mut self, id: &GroupId) -> Option<&mut Group> {
        let Some(round) = self.rounds.get(id.round_id) else {
            return None;
        };

        round.get_group_by_id(id)
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
