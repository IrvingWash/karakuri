use karakuri::{
    utils::{Color, Resolution},
    Engine,
};

fn main() {
    let mut engine = Engine::new(
        String::from("Breakout"),
        Resolution::new(800, 600),
        Color::black(),
        60,
        30,
    );

    engine.start();
}
