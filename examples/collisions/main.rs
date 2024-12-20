use karakuri::asset_storage_adapter::TexturePayload;
use karakuri::utils::Color;
use karakuri::{Game, GameConfig};

mod box_prefab;
mod operator_prefab;

use box_prefab::box_prefab;
use kmath::Vector2;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        clear_color: Color::BLUE,
        resolution: Vector2::new(800.0, 600.0),
        target_fps: 60,
        title: "Collisions",
        ..Default::default()
    });

    game.asset_storage().add_textures(vec![TexturePayload {
        name: "square",
        path: "./examples/collisions/assets/sprites/square.png",
    }])?;

    game.set_scene(vec![
        box_prefab(true),
        box_prefab(false),
        operator_prefab::operator_prefab(),
    ]);

    game.start();

    Ok(())
}
