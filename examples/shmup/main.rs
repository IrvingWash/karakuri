use karakuri::utils::Color;
use karakuri::{Game, GameConfig};
use kmath::Vector2;
use prefabs::{
    background_prefab, enemy_spawner_prefab, laser_destroyer_prefab, operator_prefab,
    player_prefab, LaserDestroyerPosition,
};

mod prefabs;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        title: "Shmup",
        clear_color: Color::BLACK,
        resolution: Vector2::new(800.0, 600.0),
        target_fps: 60,
        debug: true,
    });

    game.add_texture(
        "player-straight",
        "./examples/shmup/assets/sprites/player-straight.png",
    )?;
    game.add_texture(
        "player-left",
        "./examples/shmup/assets/sprites/player-left.png",
    )?;
    game.add_texture(
        "player-right",
        "./examples/shmup/assets/sprites/player-right.png",
    )?;
    game.add_texture(
        "projectile-green",
        "./examples/shmup/assets/sprites/projectile-green.png",
    )?;
    game.add_texture(
        "projectile-blue",
        "./examples/shmup/assets/sprites/projectile-blue.png",
    )?;
    game.add_texture(
        "enemy-straight",
        "./examples/shmup/assets/sprites/enemy-straight.png",
    )?;
    game.add_texture("explosion", "./examples/shmup/assets/sprites/explosion.png")?;
    game.add_texture(
        "cosmos",
        "./examples/shmup/assets/sprites/background/background.png",
    )?;
    game.add_texture(
        "stars",
        "./examples/shmup/assets/sprites/background/stars.png",
    )?;

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
