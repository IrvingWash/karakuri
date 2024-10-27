#[derive(Debug, PartialEq)]
pub struct TagComponent {
    value: String,
}

impl TagComponent {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
