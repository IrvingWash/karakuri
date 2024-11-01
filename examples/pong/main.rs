use karakuri::utils::{Color, Size};
use karakuri::{Game, GameConfig};
use prefabs::{
    ball::ball_prefab,
    paddle::{paddle_prefab, Side},
};

mod prefabs;

pub fn main() {
    let mut game = Game::new(GameConfig {
        clear_color: Color::BLACK,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Pong",
    });

    let resolution = game.resolution();

    game.set_scene(vec![
        paddle_prefab(Side::Left, &resolution),
        paddle_prefab(Side::Right, &resolution),
        ball_prefab(&resolution),
    ]);

    game.start();
}
