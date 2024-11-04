use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, RigidBodyComponent, SpriteComponent,
    TagComponent, TransformComponent,
};
use karakuri::math::Vector2;

pub fn enemy_laser_prefab(position: Vector2) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            position,
            rotation: 180.0,
            ..Default::default()
        }),
        box_collider: Some(BoxColliderComponent::default()),
        tag: Some(TagComponent::new(String::from("enemy_laser"))),
        sprite: Some(SpriteComponent::from_texture_name("projectile-blue")),
        rigid_body: Some(RigidBodyComponent::default()),
        behavior: Some(Box::new(EnemyLaser { speed: 70.0 })),
        ..Default::default()
    }
}

#[derive(Debug)]
struct EnemyLaser {
    speed: f64,
}

impl BehaviorComponent for EnemyLaser {
    fn on_start(&mut self, ctx: karakuri::components::Ctx) {
        let mut rigid_body = ctx
            .registry
            .get_component_mut::<RigidBodyComponent>(ctx.entity)
            .unwrap();

        rigid_body.velocity.y = self.speed;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
