use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, RigidBodyComponent, SpriteComponent,
    TagComponent, TransformComponent,
};
use karakuri::math::Vector2;
use karakuri::UpdateContext;

pub fn enemy_laser_prefab(position: Vector2) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            position,
            rotation: 180.0,
            scale: Vector2::new(2.0, 2.0),
        }),
        box_collider: Some(BoxColliderComponent::default()),
        tag: Some(TagComponent::new(String::from("enemy_laser"))),
        sprite: Some(SpriteComponent {
            texture_name: "projectile-blue",
            layer: 2,
            ..Default::default()
        }),
        rigid_body: Some(RigidBodyComponent::default()),
        behavior: Some(Box::new(EnemyLaser { speed: 50.0 })),
        ..Default::default()
    }
}

#[derive(Debug)]
struct EnemyLaser {
    speed: f64,
}

impl BehaviorComponent for EnemyLaser {
    fn on_start(&mut self, ctx: UpdateContext) {
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
