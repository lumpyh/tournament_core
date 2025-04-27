use crate::bewerb::BewerbId;
use crate::container::HasId;
use crate::error::Error;
use crate::group::GroupId;
use crate::tournament::SimpleFencer;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::slice::{Iter, IterMut};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    name: Mutex<String>,
    bewerbs: Mutex<Vec<BewerbGroup>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FencerSaveable {
    id: u32,
    name: String,
    bewerbs: Vec<BewerbGroup>,
}

impl From<&Fencer> for FencerSaveable {
    fn from(fencer: &Fencer) -> Self {
        Self {
            id: fencer.id,
            name: fencer.name.lock().unwrap().to_owned(),
            bewerbs: fencer.bewerbs.lock().unwrap().clone(),
        }
    }
}

impl From<FencerSaveable> for Fencer {
    fn from(fs: FencerSaveable) -> Fencer {
        Self {
            id: fs.id,
            name: Mutex::new(fs.name),
            bewerbs: Mutex::new(fs.bewerbs),
        }
    }
}

impl From<&Fencer> for SimpleFencer {
    fn from(fencer: &Fencer) -> Self {
        Self {
            id: fencer.id,
            name: fencer.name.lock().unwrap().to_owned(),
            bewerbs: fencer
                .bewerbs
                .lock()
                .unwrap()
                .iter()
                .map(|x| (&x.bewerb_id).into())
                .collect(),
        }
    }
}

impl Fencer {
    pub fn new(name: String, bewerbs: Vec<BewerbId>) -> Self {
        Self {
            id: 0,
            name: Mutex::new(name),
            bewerbs: Mutex::new(bewerbs.iter().map(BewerbGroup::new).collect()),
        }
    }

    pub fn update(&self, sf: SimpleFencer) {
        *self.name.lock().unwrap() = sf.name;
        *self.bewerbs.lock().unwrap() = sf
            .bewerbs
            .iter()
            .map(|x| BewerbGroup::new(&x.into()))
            .collect();
    }

    pub fn is_same(&self, sf: &SimpleFencer) -> bool {
        self.id == sf.id && *self.name.lock().unwrap() == sf.name
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

#[derive(Default, Debug)]
pub struct Fencers {
    fencers: Vec<Arc<Fencer>>,
}

impl Fencers {
    fn get_new_id(&self) -> u32 {
        let mut id = 0;
        let ids: Vec<u32> = self.fencers.iter().map(|x| x.get_id()).collect();
        while ids.contains(&id) {
            id += 1;
        }
        id
    }

    pub fn push(&mut self, mut item: Fencer) {
        let id = self.get_new_id();
        item.set_id(id);
        self.fencers.push(Arc::new(item));
    }

    pub fn remove(&mut self, id: u32) {
        self.fencers.retain(|x| x.get_id() != id);
    }

    pub fn get(&self, id: u32) -> Option<Arc<Fencer>> {
        self.fencers.iter().find(|x| x.get_id() == id).cloned()
    }

    pub fn from(fencers: Vec<FencerSaveable>) -> Self {
        let mut vec = Vec::new();
        for fencer in fencers {
            let item = fencer.into();
            vec.push(Arc::new(item));
        }

        Self { fencers: vec }
    }

    pub fn iter(&self) -> Iter<'_, Arc<Fencer>> {
        self.fencers.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Arc<Fencer>> {
        self.fencers.iter_mut()
    }

    pub fn save_to_file(&self) -> Result<(), Error> {
        let path = std::path::Path::new("fencers.json");
        let path = std::path::absolute(path)?;
        println!("save fancers to {path:?}");

        let file = File::create(path)?;
        let vec: Vec<FencerSaveable> = self.into();
        serde_json::to_writer_pretty(file, &vec)?;
        Ok(())
    }

    pub fn from_json_file() -> Result<Self, Error> {
        let file = File::open("fencers.json")?;
        let fencers: Vec<FencerSaveable> = serde_json::from_reader(file)?;
        let fencers = Self::from(fencers);
        Ok(fencers)
    }
}

impl From<&Fencers> for Vec<FencerSaveable> {
    fn from(fencers: &Fencers) -> Vec<FencerSaveable> {
        let mut vec = Vec::new();
        for fencer in &fencers.fencers {
            let fs = fencer.as_ref().into();
            vec.push(fs);
        }
        vec
    }
}
