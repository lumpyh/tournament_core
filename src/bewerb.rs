use crate::container::{HasId, UidContainer};
use crate::round::Round;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
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
}

impl HasId for Bewerb {
    fn get_id(&self) -> u32 {
        self.id.bewerb_id
    }
    fn set_id(&mut self, id: u32) {
        self.id.bewerb_id = id;
    }
}
