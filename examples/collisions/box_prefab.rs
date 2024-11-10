use std::any::Any;

use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, SpriteComponent, TagComponent,
    TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;
use karakuri::utils::Color;
use karakuri::window::KeyboardKey;
use karakuri::UpdateContext;

pub fn box_prefab(interactive: bool) -> ComponentPayload {
    let tag = if interactive {
        String::from("controlled_box")
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
            Some(Box::new(MyBoxControlled {}))
        } else {
            Some(Box::new(MyBox::default()))
        },
        sprite: Some(SpriteComponent {
            texture_name: "square",
            layer: if interactive { 0 } else { 1 },
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[derive(Debug, Default)]
struct MyBox {
    controlled: Option<Entity>,
}

impl BehaviorComponent for MyBox {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn on_start(&mut self, ctx: UpdateContext) {
        self.controlled = ctx
            .registry
            .find_entity(&TagComponent::new(String::from("controlled_box")));
    }

    fn on_update(&mut self, ctx: UpdateContext) {
        let mut sprite = ctx
            .registry
            .get_component_mut::<SpriteComponent>(ctx.entity)
            .unwrap();

        sprite.tint = Color::WHITE;

        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(ctx.entity)
            .unwrap();

        transform.position.move_towards(
            &ctx.registry
                .get_component::<TransformComponent>(self.controlled.as_ref().unwrap())
                .unwrap()
                .position,
            10.0 * ctx.delta_time,
        );
    }

    fn on_collision(&mut self, _other: &kec::Entity, ctx: UpdateContext) {
        let mut sprite = ctx
            .registry
            .get_component_mut::<SpriteComponent>(ctx.entity)
            .unwrap();

        sprite.tint = Color::YELLOW;
    }
}

#[derive(Debug)]
struct MyBoxControlled {}

impl BehaviorComponent for MyBoxControlled {
    fn on_update(&mut self, ctx: UpdateContext) {
        let speed = if ctx.input_processor.is_down(KeyboardKey::KEY_LEFT_SHIFT) {
            90.0
        } else {
            30.0
        };

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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
