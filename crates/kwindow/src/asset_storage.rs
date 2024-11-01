use std::collections::HashMap;

use raylib::{texture::Texture2D, RaylibThread};

use crate::WindowCtx;

#[derive(Debug)]
pub struct AssetStorage {
    textures: HashMap<&'static str, Texture2D>,
    thread: RaylibThread,
}

impl AssetStorage {
    pub fn new(thread: RaylibThread) -> Self {
        Self {
            textures: HashMap::new(),
            thread,
        }
    }

    pub fn add_texture(
        &mut self,
        name: &'static str,
        path: &'static str,
        ctx: &mut WindowCtx,
    ) -> Result<(), String> {
        if self.textures.contains_key(&name) {
            return Ok(());
        }

        let texture = ctx.load_texture(&self.thread, path)?;

        self.textures.insert(name, texture);

        Ok(())
    }

    pub fn texture(&self, name: &'static str) -> Option<&Texture2D> {
        self.textures.get(&name)
    }
}
