use karakuri::{Game, GameConfig};
use kutils::{Color, Size};
use prefabs::paddle::{paddle_prefab, Side};

mod prefabs;

pub fn main() {
    let mut game = Game::new(GameConfig {
        clear_color: Color::BLACK,
        min_update_fps: 30,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Pong",
    });

    let resolution = game.resolution();

    game.set_scene(vec![
        paddle_prefab(Side::Left, &resolution),
        paddle_prefab(Side::Right, &resolution),
    ]);

    game.start();
}
