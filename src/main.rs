use std::any::Any;

use karakuri::{
    components::{Behavior, ComponentPayload, Sprite, Tag, Transform},
    Engine,
};
use kmath::Vector2;
use kutils::Color;

struct Sonic {
    name: &'static str,
}

impl Behavior for Sonic {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn on_start(&mut self) {
        println!("Hi, {}", self.name)
    }

    fn on_update(&mut self, delta_time: f64) {
        println!("{}", delta_time);
    }

    fn on_destroy(&mut self) {
        println!("Bye, {}", self.name);
    }
}

pub fn main() {
    let mut engine = Engine::new();

    let scene = engine.scene();

    scene.create_entity(ComponentPayload {
        sprite: Some(Sprite::new(Color::RED, Vector2::new(50., 50.))),
        tag: Some(Tag::new("Sonic")),
        transform: Some(Transform::from_position(Vector2::new(100., 100.))),
        behavior: Some(Box::new(Sonic { name: "Sonic" })),
    });

    engine.play();
}
