#[derive(Debug)]
pub struct NameComponent {
    value: String,
}

impl NameComponent {
    pub fn new(value: String) -> NameComponent {
        NameComponent { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
