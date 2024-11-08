use karakuri::{Game, GameConfig};
use kutils::{Color, Size};
use prefabs::{operator_prefab, player_prefab, race_track_prefab};

mod prefabs;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        clear_color: Color::BLUE,
        debug: true,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Camera",
    });

    let halved_resolution = game.resolution().halved();

    game.add_texture("player", "./examples/camera/assets/sprites/square.png")?;
    game.add_texture(
        "race_track",
        "./examples/camera/assets/sprites/race_track.png",
    )?;

    game.set_scene(vec![
        player_prefab(),
        operator_prefab(),
        race_track_prefab(&halved_resolution),
    ]);

    game.start();

    Ok(())
}
