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

#[cfg(test)]
mod tag_component_tests {
    use super::TagComponent;

    #[test]
    fn test_value() {
        const NAME: &str = "Test";

        let tag = TagComponent::new(NAME.to_owned());

        assert_eq!(tag.value(), NAME);
    }
}
