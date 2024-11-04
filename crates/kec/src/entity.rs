use std::cmp::PartialEq;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Entity {
    id: usize,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
