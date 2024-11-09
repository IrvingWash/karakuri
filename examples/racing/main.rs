use karakuri::{asset_storage_adapter::TexturePayload, Game, GameConfig};
use kmath::Vector2;
use kutils::Color;
use prefabs::{operator_prefab, player_prefab, race_track_prefab};

mod prefabs;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        clear_color: Color::BLUE,
        debug: true,
        resolution: Vector2::new(800.0, 600.0),
        target_fps: 60,
        title: "Racing",
    });

    let halved_resolution = game.resolution().to_divided(2.0);

    game.asset_storage()
        .set_textures_base_path("./examples/racing/assets/sprites")
        .add_textures(vec![
            TexturePayload {
                name: "car",
                path: "car.png",
            },
            TexturePayload {
                name: "race_track",
                path: "race_track.png",
            },
        ])?;

    game.set_scene(vec![
        player_prefab(),
        operator_prefab(),
        race_track_prefab(&halved_resolution),
    ]);

    game.start();

    Ok(())
}
