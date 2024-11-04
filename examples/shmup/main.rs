use karakuri::utils::{Color, Size};
use karakuri::{Game, GameConfig};
use prefabs::{enemy_spawner_prefab, laser_destroyer_prefab, player_prefab};

mod prefabs;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        title: "Shmup",
        clear_color: Color::BLACK,
        resolution: Size::new(800, 600),
        target_fps: 60,
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
        "laser_blue",
        "./examples/shmup/assets/sprites/laser_blue.png",
    )?;
    game.add_texture("enemy_red", "./examples/shmup/assets/sprites/enemy_red.png")?;

    let resolution = game.resolution();

    game.set_scene(vec![
        player_prefab(&resolution),
        laser_destroyer_prefab(),
        enemy_spawner_prefab(),
    ]);

    game.start();

    Ok(())
}
