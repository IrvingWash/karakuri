use std::{cmp::PartialEq, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    key: usize,
    unique_id: usize,
}

impl Entity {
    pub fn new(key: usize, unique_id: usize) -> Self {
        Self { key, unique_id }
    }

    pub fn key(&self) -> usize {
        self.key
    }

    pub fn unique_id(&self) -> usize {
        self.unique_id
    }
}
