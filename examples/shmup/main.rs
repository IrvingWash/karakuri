use karakuri::utils::Color;
use karakuri::{Game, GameConfig};
use kmath::Vector2;
use prefabs::{
    background_prefab, enemy_spawner_prefab, laser_destroyer::LaserDestroyerPosition,
    laser_destroyer_prefab, operator_prefab, player_prefab,
};
use texture_loader::load_textures;

mod prefabs;
mod texture_loader;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        title: "Shmup",
        clear_color: Color::BLACK,
        resolution: Vector2::new(800.0, 600.0),
        target_fps: 60,
        debug: true,
    });

    load_textures(game.asset_storage())?;

    let resolution = game.resolution();

    game.set_scene(vec![
        operator_prefab(),
        player_prefab(&resolution),
        laser_destroyer_prefab(LaserDestroyerPosition::Top),
        laser_destroyer_prefab(LaserDestroyerPosition::Bottom),
        enemy_spawner_prefab(),
        background_prefab(&resolution, "cosmos", 0),
        background_prefab(&resolution, "stars", 1),
    ]);

    game.start();

    Ok(())
}
