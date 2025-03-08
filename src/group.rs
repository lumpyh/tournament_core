use serde::{Deserialize, Serialize};

use crate::container::HasId;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Group {
    id: u32,
    fencers: Vec<u32>,
}

impl HasId for Group {
    fn get_id(&self) -> u32 {
        self.id
    }
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
