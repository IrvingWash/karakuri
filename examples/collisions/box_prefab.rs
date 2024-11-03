use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, SpriteComponent, TagComponent,
    TransformComponent,
};
use kmath::Vector2;
use kutils::Color;
use kwindow::KeyboardKey;

pub fn box_prefab(interactive: bool) -> ComponentPayload {
    let tag = if interactive {
        String::from("interactive_box")
    } else {
        String::from("box")
    };

    let scale = if interactive { 2.0 } else { 1.0 };

    ComponentPayload {
        box_collider: Some(BoxColliderComponent::default()),
        tag: Some(TagComponent::new(tag)),
        transform: Some(TransformComponent {
            position: Vector2::new(400.0, 300.0),
            scale: Vector2::new(3.0 * scale, 2.0 * scale),
            rotation: 0.0,
        }),
        behavior: if interactive {
            Some(Box::new(MyBoxInteractive {}))
        } else {
            Some(Box::new(MyBox {}))
        },
        sprite: Some(SpriteComponent::from_texture_name("square")),
        ..Default::default()
    }
}

#[derive(Debug)]
struct MyBox {}

impl BehaviorComponent for MyBox {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn on_update(&mut self, ctx: karakuri::components::Ctx) {
        let mut sprite = ctx
            .registry
            .get_component_mut::<SpriteComponent>(ctx.entity)
            .unwrap();

        sprite.tint = Color::WHITE;
    }

    fn on_collision(&mut self, _other: &kec::Entity, ctx: karakuri::components::Ctx) {
        let mut sprite = ctx
            .registry
            .get_component_mut::<SpriteComponent>(ctx.entity)
            .unwrap();

        sprite.tint = Color::YELLOW;
    }
}

#[derive(Debug)]
struct MyBoxInteractive {}

impl BehaviorComponent for MyBoxInteractive {
    fn on_update(&mut self, ctx: karakuri::components::Ctx) {
        let speed = 10.0;
        let delta_time = ctx.delta_time;

        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(ctx.entity)
            .unwrap();

        if ctx.input_processor.is_down(KeyboardKey::KEY_UP) {
            transform.position.y -= speed * delta_time;
        }

        if ctx.input_processor.is_down(KeyboardKey::KEY_LEFT) {
            transform.position.x -= speed * delta_time;
        }

        if ctx.input_processor.is_down(KeyboardKey::KEY_DOWN) {
            transform.position.y += speed * delta_time;
        }

        if ctx.input_processor.is_down(KeyboardKey::KEY_RIGHT) {
            transform.position.x += speed * delta_time;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
