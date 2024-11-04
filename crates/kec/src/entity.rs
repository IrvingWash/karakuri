use std::{cmp::PartialEq, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    id: usize,
    unique_id: usize,
}

impl Entity {
    pub fn new(id: usize, unique_id: usize) -> Self {
        Self { id, unique_id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn unique_id(&self) -> usize {
        self.unique_id
    }
}
