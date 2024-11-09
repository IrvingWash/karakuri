use karakuri::utils::Color;
use karakuri::{Game, GameConfig};
use kmath::Vector2;
use prefabs::{
    ball::ball_prefab,
    paddle::{paddle_prefab, PaddleSide},
};

mod prefabs;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        clear_color: Color::BLACK,
        resolution: Vector2::new(800.0, 600.0),
        target_fps: 60,
        title: "Pong",
        ..Default::default()
    });

    game.add_texture("square", "./examples/pong/assets/sprites/square.png")?;

    let resolution = game.resolution();

    game.set_scene(vec![
        paddle_prefab(PaddleSide::Left, &resolution),
        paddle_prefab(PaddleSide::Right, &resolution),
        ball_prefab(&resolution),
    ]);

    game.start();

    Ok(())
}
