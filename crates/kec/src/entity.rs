use std::{cmp::PartialEq, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    key: usize,
    unique_id: usize,
}

impl Entity {
    #[inline]
    pub fn new(key: usize, unique_id: usize) -> Self {
        Self { key, unique_id }
    }

    #[inline]
    pub fn key(&self) -> usize {
        self.key
    }

    #[inline]
    pub fn unique_id(&self) -> usize {
        self.unique_id
    }
}
