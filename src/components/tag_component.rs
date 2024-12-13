#[derive(Debug)]
pub struct TagComponentParams<'a> {
    pub tag: &'a str,
}

#[derive(Debug)]
pub struct TagComponent {
    tag: String,
}

impl TagComponent {
    #[inline]
    pub fn new(params: &TagComponentParams) -> Self {
        Self {
            tag: params.tag.to_string(),
        }
    }

    #[inline]
    pub fn tag(&self) -> &str {
        &self.tag
    }
}
