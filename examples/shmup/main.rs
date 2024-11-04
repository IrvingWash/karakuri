use karakuri::utils::{Color, Size};
use karakuri::{Game, GameConfig};
use prefabs::{laser_destroyer_prefab, player_prefab};

mod prefabs;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        title: "Shmup",
        clear_color: Color::BLACK,
        resolution: Size::new(800, 600),
        target_fps: 60,
    });

    game.add_texture("ship_blue", "./examples/shmup/assets/sprites/ship_blue.png")?;
    game.add_texture(
        "laser_blue",
        "./examples/shmup/assets/sprites/laser_blue.png",
    )?;

    let resolution = game.resolution();

    game.set_scene(vec![player_prefab(&resolution), laser_destroyer_prefab()]);

    game.start();

    Ok(())
}
