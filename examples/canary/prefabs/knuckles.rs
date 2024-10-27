use karakuri::components::{
    BehaviorComponent, ComponentPayload, Ctx, SpriteComponent, TagComponent, TransformComponent,
};
use kmath::Vector2;
use kutils::{Color, Size};

pub fn knuckles_prefab() -> ComponentPayload {
    ComponentPayload {
        tag: Some(TagComponent::new(String::from("Knuckles"))),
        transform: Some(TransformComponent::from_position(Vector2::new(300., 500.))),
        sprite: Some(SpriteComponent::new(Size::new(100, 100), Color::RED)),
        behavior: Some(Box::new(Knuckles { speed: 50.0 })),
    }
}

#[derive(Debug)]
struct Knuckles {
    speed: f64,
}

impl BehaviorComponent for Knuckles {
    fn on_start(&mut self) {}

    fn on_update(&mut self, ctx: Ctx) {
        let tails = ctx
            .registry
            .find_entity(TagComponent::new(String::from("Tails")))
            .unwrap();

        let tails_transform = ctx
            .registry
            .get_component::<TransformComponent>(&tails)
            .unwrap();
        let mut knuckles_transfrom = ctx
            .registry
            .get_component_mut::<TransformComponent>(ctx.entity)
            .unwrap();

        knuckles_transfrom
            .position
            .move_towards(&tails_transform.position, self.speed * ctx.delta_time);
    }

    fn on_destroy(&mut self) {}
}
