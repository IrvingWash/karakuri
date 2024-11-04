use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, RigidBodyComponent, SpriteComponent,
    TagComponent, TransformComponent,
};

pub fn player_laser_prefab(transform: TransformComponent) -> ComponentPayload {
    ComponentPayload {
        transform: Some(transform),
        box_collider: Some(BoxColliderComponent::default()),
        tag: Some(TagComponent::new(String::from("player_laser"))),
        sprite: Some(SpriteComponent::from_texture_name("laser_blue")),
        rigid_body: Some(RigidBodyComponent::default()),
        behavior: Some(Box::new(PlayerLaser { speed: 100.0 })),
        ..Default::default()
    }
}

#[derive(Debug)]
struct PlayerLaser {
    speed: f64,
}

impl BehaviorComponent for PlayerLaser {
    fn on_start(&mut self, ctx: karakuri::components::Ctx) {
        let mut rigid_body = ctx
            .registry
            .get_component_mut::<RigidBodyComponent>(ctx.entity)
            .unwrap();

        rigid_body.velocity.y = -self.speed;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
