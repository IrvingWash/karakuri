use std::{collections::HashMap, path::PathBuf};

use raylib::{texture::Texture2D, RaylibThread};

use crate::WindowCtx;

#[derive(Debug)]
pub struct AssetStorage {
    textures_base_path: Option<PathBuf>,
    textures: HashMap<&'static str, Texture2D>,
    thread: RaylibThread,
}

impl AssetStorage {
    #[inline]
    pub fn new(thread: RaylibThread) -> Self {
        Self {
            textures: HashMap::new(),
            thread,
            textures_base_path: None,
        }
    }

    #[inline]
    pub fn set_textures_base_path(&mut self, base_path: &'static str) {
        self.textures_base_path = Some(PathBuf::from(base_path));
    }

    #[inline]
    pub fn add_texture(
        &mut self,
        name: &'static str,
        path: &'static str,
        ctx: &mut WindowCtx,
    ) -> Result<(), String> {
        if self.textures.contains_key(&name) {
            return Ok(());
        }

        let full_path = match &self.textures_base_path {
            None => path.to_owned(),
            Some(base_path) => {
                let mut clone = base_path.clone();

                clone.push(path);

                match clone.to_str() {
                    None => return Err(String::from("Incorrect path")),
                    Some(str) => str.to_owned(),
                }
            }
        };

        let texture = ctx.load_texture(&self.thread, &full_path)?;

        self.textures.insert(name, texture);

        Ok(())
    }

    #[inline]
    pub fn texture(&self, name: &'static str) -> Option<&Texture2D> {
        self.textures.get(&name)
    }
}
