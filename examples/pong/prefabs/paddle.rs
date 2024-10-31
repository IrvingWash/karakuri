use karakuri::components::{
    BehaviorComponent, ComponentPayload, FigureComponent, TagComponent, TransformComponent,
};
use karakuri::kmath::Vector2;
use karakuri::kutils::{Color, Size};
use kwindow::KeyboardKey;

#[derive(PartialEq, Debug)]
pub enum Side {
    Left,
    Right,
}

pub fn paddle_prefab(side: Side, resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        figure: Some(FigureComponent::new(Size::new(30, 200), Color::WHITE)),
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
        ..Default::default()
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
        let figure = ctx
            .registry
            .get_component::<FigureComponent>(&ctx.entity)
            .unwrap();

        let edge_offset = 50.0;

        if self.side == Side::Left {
            transform.position.set(&Vector2::new(
                edge_offset + (figure.size.width as f64) / 2.0,
                (self.resolution.height / 2) as f64,
            ));
        } else {
            transform.position.set(&Vector2::new(
                (self.resolution.width as f64) - edge_offset - (figure.size.width as f64) / 2.0,
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
            if ctx
                .input_processor
                .is_down(KeyboardKey::KEY_W, ctx.window_ctx)
            {
                transform.position.y -= self.speed * ctx.delta_time;
            }
            if ctx
                .input_processor
                .is_down(KeyboardKey::KEY_S, ctx.window_ctx)
            {
                transform.position.y += self.speed * ctx.delta_time;
            }
        } else {
            if ctx
                .input_processor
                .is_down(KeyboardKey::KEY_UP, ctx.window_ctx)
            {
                transform.position.y -= self.speed * ctx.delta_time;
            }
            if ctx
                .input_processor
                .is_down(KeyboardKey::KEY_DOWN, ctx.window_ctx)
            {
                transform.position.y += self.speed * ctx.delta_time;
            }
        }
    }

    fn on_destroy(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
