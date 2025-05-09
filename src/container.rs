use serde::{Deserialize, Serialize};
use std::slice::{Iter, IterMut};

pub trait HasId {
    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UidContainer<T> {
    vec: Vec<T>,
}

impl<T: HasId> UidContainer<T> {
    pub fn insert(&mut self, t: T) {
        self.vec.push(t);
    }

    fn get_new_id(&self) -> u32 {
        let mut id = 0;
        let ids: Vec<u32> = self.vec.iter().map(|x| x.get_id()).collect();
        while ids.contains(&id) {
            id += 1;
        }
        id
    }

    pub fn get_next_id(&self) -> u32 {
        self.get_new_id()
    }

    pub fn push(&mut self, mut item: T) {
        let id = self.get_new_id();
        item.set_id(id);
        self.vec.push(item);
    }

    pub fn remove(&mut self, id: u32) {
        self.vec.retain(|x| x.get_id() != id);
    }

    pub fn get(&self, id: u32) -> Option<&T> {
        self.iter().find(|x| x.get_id() == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut T> {
        self.iter_mut().find(|x| x.get_id() == id)
    }
}

impl<T> UidContainer<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.vec.iter_mut()
    }
}
