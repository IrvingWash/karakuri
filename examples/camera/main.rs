use karakuri::{Game, GameConfig};
use kutils::{Color, Size};
use prefabs::{operator_prefab, player_prefab};

mod prefabs;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        clear_color: Color::BLUE,
        debug: true,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Camera",
    });

    let resolution = game.resolution();

    game.add_texture("player", "./examples/camera/assets/sprites/square.png")?;

    game.set_scene(vec![player_prefab(&resolution), operator_prefab()]);

    game.start();

    Ok(())
}
