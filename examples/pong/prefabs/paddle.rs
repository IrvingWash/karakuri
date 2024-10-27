use karakuri::components::{
    BehaviorComponent, ComponentPayload, SpriteComponent, TagComponent, TransformComponent,
};
use kmath::Vector2;
use kutils::{Color, Size};

#[derive(PartialEq, Debug)]
pub enum Side {
    Left,
    Right,
}

pub fn paddle_prefab(side: Side, resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        sprite: Some(SpriteComponent::new(Size::new(30, 200), Color::WHITE)),
        transform: Some(TransformComponent::default()),
        tag: if side == Side::Left {
            Some(TagComponent::new(String::from("left-paddle")))
        } else {
            Some(TagComponent::new(String::from("right-paddle")))
        },
        behavior: Some(Box::new(Paddle {
            side,
            speed: 300.0,
            resolution: resolution.clone(),
        })),
    }
}

#[derive(Debug)]
struct Paddle {
    side: Side,
    speed: f64,
    resolution: Size,
}

impl BehaviorComponent for Paddle {
    fn on_start(&mut self, ctx: karakuri::components::Ctx) {
        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(&ctx.entity)
            .unwrap();
        let sprite = ctx
            .registry
            .get_component::<SpriteComponent>(&ctx.entity)
            .unwrap();

        let edge_offset = 50.0;

        if self.side == Side::Left {
            transform.position.set(&Vector2::new(
                edge_offset + (sprite.size.width as f64) / 2.0,
                (self.resolution.height / 2) as f64,
            ));
        } else {
            transform.position.set(&Vector2::new(
                (self.resolution.width as f64) - edge_offset - (sprite.size.width as f64) / 2.0,
                (self.resolution.height / 2) as f64,
            ));
        }
    }

    fn on_update(&mut self, ctx: karakuri::components::Ctx) {
        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(&ctx.entity)
            .unwrap();

        if self.side == Side::Left {
            if ctx.input_result.w {
                transform.position.y -= self.speed * ctx.delta_time;
            }
            if ctx.input_result.s {
                transform.position.y += self.speed * ctx.delta_time;
            }
        } else {
            if ctx.input_result.up {
                transform.position.y -= self.speed * ctx.delta_time;
            }
            if ctx.input_result.down {
                transform.position.y += self.speed * ctx.delta_time;
            }
        }
    }

    fn on_destroy(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
