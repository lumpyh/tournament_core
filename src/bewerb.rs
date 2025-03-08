use crate::container::{HasId, UidContainer};
use crate::round::Round;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Bewerb {
    id: u32,
    name: String,
    rounds: UidContainer<Round>,
}

impl Bewerb {
    pub fn new(name: String, n_rounds: u32, n_groups: u32) -> Self {
        let mut res = Bewerb {
            id: 0,
            name: name.clone(),
            rounds: UidContainer::default(),
        };

        for i in 0..n_rounds {
            let round = Round::new(n_groups);
            res.rounds.push(round);
        }

        res
    }
}

impl HasId for Bewerb {
    fn get_id(&self) -> u32 {
        self.id
    }
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
