#[derive(Debug)]
pub struct Entity {
    id: usize,
}

impl Entity {
    pub fn new(id: usize) -> Entity {
        Entity { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
