#import "Maybe";

Entity :: struct {
    using token: Entity_Token;

    signature: Signature;

    transform: Transform_Component;

    on_start: (self: *Entity);
    on_update: (self: *Entity, dt: float);
}

Entity_Token :: struct {
    id: u32;
    generation: u32;
}

Entity_Params :: struct {
    transform: Transform_Component;

    on_start: (self: *Entity);
    on_update: (self: *Entity, dt: float);

    sprite_component: Maybe(Sprite_Component);
    behavior_component: Maybe(*Behavior_Component);
}

#scope_module

create_entity :: (
    token: Entity_Token,
    params: Entity_Params
) -> Entity {
    return .{
        token = token,

        transform = params.transform,

        on_start = params.on_start,
        on_update = params.on_update,
    };
}

start_entity :: (entity: *Entity) {
    if entity.on_start != null {
        entity.on_start(entity);
    }
}

update_entity :: (entity: *Entity, dt: float) {
    if entity.on_update != null {
        entity.on_update(entity, dt);
    }
}
