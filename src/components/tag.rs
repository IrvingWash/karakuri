pub struct Tag {
    value: String,
}

impl Tag {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
