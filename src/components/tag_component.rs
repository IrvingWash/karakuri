#[derive(Debug, PartialEq)]
pub struct TagComponent {
    value: String,
}

impl TagComponent {
    #[inline]
    pub fn new(value: String) -> Self {
        Self { value }
    }

    #[inline]
    pub fn value(&self) -> &String {
        &self.value
    }
}
