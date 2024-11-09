use karakuri::{asset_storage_adapter::TexturePayload, AssetStorageAdapter};

pub fn load_textures(asset_storage: AssetStorageAdapter) -> Result<(), String> {
    asset_storage
        .set_textures_base_path("./examples/shmup/assets/sprites")
        .add_textures(vec![
            TexturePayload {
                name: "player-straight",
                path: "player-straight.png",
            },
            TexturePayload {
                name: "player-left",
                path: "player-left.png",
            },
            TexturePayload {
                name: "player-right",
                path: "player-right.png",
            },
            TexturePayload {
                name: "projectile-green",
                path: "projectile-green.png",
            },
            TexturePayload {
                name: "projectile-blue",
                path: "projectile-blue.png",
            },
            TexturePayload {
                name: "enemy-straight",
                path: "enemy-straight.png",
            },
            TexturePayload {
                name: "explosion",
                path: "explosion.png",
            },
            TexturePayload {
                name: "cosmos",
                path: "background/background.png",
            },
            TexturePayload {
                name: "stars",
                path: "background/stars.png",
            },
        ])?;

    Ok(())
}
