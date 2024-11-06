#[derive(Debug, PartialEq)]
pub struct TagComponent {
    value: String,
}

impl TagComponent {
    #[inline]
    pub const fn new(value: String) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn value(&self) -> &String {
        &self.value
    }
}
