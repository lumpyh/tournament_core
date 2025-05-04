use crate::bewerb::{Bewerb, BewerbId};
use crate::container::{HasId, UidContainer};
use crate::error::Error;
use crate::group::{Group, GroupId};
use crate::tournament::SimpleFencer;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::slice::{Iter, IterMut};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug, Default)]
pub struct BewerbGroup {
    pub bewerb_id: BewerbId,
    pub groups: Vec<Option<Arc<Group>>>,
}

impl BewerbGroup {
    pub fn new(id: &BewerbId) -> Self {
        Self {
            bewerb_id: id.clone(),
            groups: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BewerbGroupSaveable {
    bewerb_id: BewerbId,
    groups: Vec<Option<GroupId>>,
}

impl From<&BewerbGroup> for BewerbGroupSaveable {
    fn from(bewerb: &BewerbGroup) -> Self {
        let groups = bewerb
            .groups
            .iter()
            .map(|x| x.as_ref().map(|x| x.id()))
            .collect();
        Self {
            bewerb_id: bewerb.bewerb_id.clone(),
            groups,
        }
    }
}

#[derive(Debug, Default)]
pub struct Fencer {
    id: u32,
    name: Mutex<String>,
    bewerbs: Mutex<Vec<BewerbGroup>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FencerSaveable {
    id: u32,
    name: String,
    bewerbs: Vec<BewerbGroupSaveable>,
}

impl From<&Fencer> for FencerSaveable {
    fn from(fencer: &Fencer) -> Self {
        Self {
            id: fencer.id,
            name: fencer.name.lock().unwrap().to_owned(),
            bewerbs: fencer
                .bewerbs
                .lock()
                .unwrap()
                .iter()
                .map(|x| x.into())
                .collect(),
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
    pub fn from_saveable(fs: FencerSaveable, bewerbs: &UidContainer<Bewerb>) -> Arc<Fencer> {
        let mut bewerb_groups = Vec::new();
        for bewerb_group in fs.bewerbs.iter() {
            let Some(bewerb) = bewerbs
                .iter()
                .find(|x| x.get_id() == bewerb_group.bewerb_id.bewerb_id)
            else {
                println!(
                    "Warning: could not find bewerb {:?}",
                    bewerb_group.bewerb_id
                );
                continue;
            };

            let mut groups = Vec::new();
            for _i in 0..bewerb.n_rounds {
                groups.push(None);
            }

            let new_bewerb_group = BewerbGroup {
                bewerb_id: bewerb_group.bewerb_id.clone(),
                groups,
            };

            bewerb_groups.push(new_bewerb_group);
        }

        let res = Arc::new(Self {
            id: fs.id,
            name: Mutex::new(fs.name),
            bewerbs: Mutex::new(bewerb_groups),
        });

        let group_ids = fs
            .bewerbs
            .iter()
            .flat_map(|x| x.groups.iter())
            .filter_map(|x| x.clone());
        for group_id in group_ids {
            let Some(group) = bewerbs
                .iter()
                .find(|x| x.get_id() == group_id.bewerb_id)
                .and_then(|x| x.get_group_by_id(&group_id))
            else {
                println!("Cant find group \"{:?}\"", group_id);
                continue;
            };

            Group::add_fencer_to_group(group, res.clone());
        }

        res
    }

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

    pub fn add_group(&self, group: Arc<Group>) {
        let mut locked = self.bewerbs.lock().unwrap();
        let Some(bewerb) = locked
            .iter_mut()
            .find(|x| x.bewerb_id.bewerb_id == group.id().bewerb_id)
        else {
            println!("Error: Fencer not in that Bewerb \"{:?}\"", group.id());
            return;
        };

        let Some(slot) = bewerb.groups.get_mut(group.id().round_id as usize) else {
            println!(
                "Error: bewerb \"{:?}\" has not enough slots \"{:?}\"",
                bewerb.bewerb_id,
                group.id()
            );
            return;
        };

        if let Some(slot) = slot {
            slot.remove_fencer(self.id);
        }
        *slot = Some(group);
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

    pub fn from(fencers: Vec<FencerSaveable>, bewerbs: &UidContainer<Bewerb>) -> Self {
        let mut vec = Vec::new();
        for fencer in fencers {
            let item = Fencer::from_saveable(fencer, bewerbs);
            vec.push(item);
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

    pub fn from_json_file(bewerbs: &UidContainer<Bewerb>) -> Result<Self, Error> {
        let file = File::open("fencers.json")?;
        let fencers: Vec<FencerSaveable> = serde_json::from_reader(file)?;
        let fencers = Self::from(fencers, bewerbs);
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
