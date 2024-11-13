use std::{cmp::PartialEq, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    key: usize,
    unique_id: usize,
}

impl Entity {
    #[inline]
    pub const fn new(key: usize, unique_id: usize) -> Self {
        Self { key, unique_id }
    }

    #[inline]
    pub const fn key(&self) -> usize {
        self.key
    }

    #[inline]
    pub const fn unique_id(&self) -> usize {
        self.unique_id
    }
}

#[cfg(test)]
mod entity_tests {
    use super::*;

    #[test]
    fn test_ids() {
        let entity = Entity::new(0, 1);

        assert_eq!(entity.key(), 0);
        assert_eq!(entity.unique_id(), 1);
    }
}
