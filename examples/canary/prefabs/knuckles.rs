use karakuri::components::{
    BehaviorComponent, ComponentPayload, Ctx, SpriteComponent, TagComponent, TransformComponent,
};
use kmath::Vector2;
use kutils::{Color, Size};

pub fn knuckles_prefab() -> ComponentPayload {
    ComponentPayload {
        tag: Some(TagComponent::new(String::from("Knuckles"))),
        transform: Some(TransformComponent::from_position(Vector2::new(300., 500.))),
        sprite: Some(SpriteComponent::new(Size::new(300, 300), Color::RED)),
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
        let mut transform_components = ctx.registry.get_component_vec_mut::<TransformComponent>();

        // TODO: This is a hack!
        let tails = 1;
        let tails_position = {
            transform_components[tails]
                .as_ref()
                .unwrap()
                .position
                .create_copy()
        };

        let knuckles_transform = transform_components[ctx.entity.id()].as_mut().unwrap();

        knuckles_transform
            .position
            .move_towards(&tails_position, self.speed * ctx.delta_time);
    }

    fn on_destroy(&mut self) {}
}
