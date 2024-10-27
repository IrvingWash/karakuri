use karakuri::{Game, GameConfig};
use kutils::{Color, Size};
use prefabs::{knuckles_prefab, tails_prefab};

mod prefabs;

pub fn main() {
    let mut game = Game::new(GameConfig {
        title: "Sonic",
        resolution: Size::new(800, 600),
        target_fps: 60,
        min_update_fps: 30,
        clear_color: Color::BLUE,
    });

    game.set_scene(vec![knuckles_prefab(), tails_prefab()]);

    game.start();
}
