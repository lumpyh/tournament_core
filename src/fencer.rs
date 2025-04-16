use crate::bewerb::BewerbId;
use crate::container::HasId;
use crate::group::GroupId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BewerbGroup {
    bewerb_id: BewerbId,
    groups: Vec<GroupId>,
}

impl BewerbGroup {
    pub fn new(id: &BewerbId) -> Self {
        Self {
            bewerb_id: id.clone(),
            groups: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Fencer {
    id: u32,
    name: String,
    bewerbs: Vec<BewerbGroup>,
}

use crate::tournament::SimpleFencer;

impl From<&Fencer> for SimpleFencer {
    fn from(fencer: &Fencer) -> Self {
        Self {
            id: fencer.id,
            name: fencer.name.to_owned(),
            bewerbs: fencer.bewerbs.iter().map(|x| (&x.bewerb_id).into()).collect(),
        }
    }
}

impl Fencer {
    pub fn new(name: String, bewerbs: &Vec<BewerbId>) -> Self {
        Self {
            id: 0,
            name,
            bewerbs: bewerbs.iter().map(|x| BewerbGroup::new(x)).collect(),
        }
    }

    pub fn update(&mut self, sf: SimpleFencer) {
        self.name = sf.name;
        self.bewerbs = sf.bewerbs.iter().map(|x| BewerbGroup::new(&x.into())).collect();
    }
}

impl HasId for Fencer {
    fn get_id(&self) -> u32 {
        self.id
    }
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
