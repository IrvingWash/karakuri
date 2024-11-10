use kwindow::{AssetStorage, WindowCtx};

pub struct AssetStorageAdapter<'a> {
    asset_storage: &'a mut AssetStorage,
    ctx: &'a mut WindowCtx,
}

pub struct TexturePayload {
    pub name: &'static str,
    pub path: &'static str,
}

impl<'a> AssetStorageAdapter<'a> {
    #[inline]
    pub fn new(asset_storage: &'a mut AssetStorage, ctx: &'a mut WindowCtx) -> Self {
        Self { asset_storage, ctx }
    }

    #[inline]
    pub fn set_textures_base_path(self, base_path: &'static str) -> Self {
        self.asset_storage.set_textures_base_path(base_path);

        self
    }

    #[inline]
    pub fn add_textures(&mut self, textures: Vec<TexturePayload>) -> Result<(), String> {
        for texture in textures {
            self.asset_storage
                .add_texture(texture.name, texture.path, self.ctx)?;
        }

        Ok(())
    }
}
