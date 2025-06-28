Entity :: struct {
    using token: Entity_Token;

    signature: Signature;

    name: string;
    tags: [..]string;
    transform: Transform_Component;

    on_start: (self: *Entity);
    on_update: (self: *Entity, dt: float);
}

Entity_Token :: struct {
    id: u32;
    generation: u32;
}

Entity_Params :: struct {
    name: string;
    tags: [..]string;
    transform: Transform_Component;

    on_start: (self: *Entity); // @Todo `Code` instead of these?
    on_update: (self: *Entity, dt: float);

    sprite_component: Maybe.Maybe(Sprite_Component);
    behavior_component: Maybe.Maybe(*Behavior_Component); // @Todo `Code` instead of behavior component?
}

operator == :: (a: Entity_Token, b: Entity_Token) -> bool {
    return a.id == b.id && a.generation == b.generation;
}

#scope_module

create_entity :: (
    token: Entity_Token,
    params: Entity_Params
) -> Entity {
    return .{
        token = token,

        name = params.name,
        tags = params.tags,
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

destroy_entity :: (entity: Entity) {
    array_free(entity.tags);
}

destroy_entity_params :: (entity_params: Entity_Params) {
    array_free(entity_params.tags);

    if Maybe.is_some(entity_params.behavior_component) {
        free(entity_params.behavior_component);
    }
}
