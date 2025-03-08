use serde::{Deserialize, Serialize};
use crate::container::HasId;
use crate::round::RoundId;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GroupId {
    pub bewerb_name: String,
    pub bewerb_id: u32,
    pub round_id: u32,
    pub group_id: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Group {
    id: GroupId,
    fencers: Vec<u32>,
}

impl Group {
    pub fn new(id: &RoundId) -> Self {
        let mut res = Self::default();
        res.id.bewerb_name = id.bewerb_name.clone();
        res.id.bewerb_id = id.bewerb_id;
        res.id.round_id = id.round_id;
        res
    }
}

impl HasId for Group {
    fn get_id(&self) -> u32 {
        self.id.group_id
    }
    fn set_id(&mut self, id: u32) {
        self.id.group_id = id;
    }
}
