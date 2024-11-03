use karakuri::utils::{Color, Size};
use karakuri::{Game, GameConfig};

mod box_prefab;

use box_prefab::box_prefab;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        clear_color: Color::BLUE,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Collisions",
    });

    game.add_texture("square", "./examples/collisions/assets/sprites/square.png")?;

    game.set_scene(vec![box_prefab(true), box_prefab(false)]);

    game.start();

    Ok(())
}
