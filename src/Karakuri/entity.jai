Entity :: struct {
    using token:  Entity_Token;

    signature:    Signature;

    name:         string;
    tags:         [..]string;
    transform:    Transform_Component;

    on_start:     (self: *Entity);
    on_update:    (self: *Entity, dt: float);
    on_collision: (self: *Entity, other: *Entity);
    on_destroy:   (self: *Entity);
}

Entity_Token :: struct {
    id:         u32;
    generation: u32;
}

Entity_Params :: struct {
    name:                   string;
    tags:                   [..]string;
    transform:              Transform_Component;

    on_start:               (self: *Entity);
    on_update:              (self: *Entity, dt: float);
    on_collision:           (self: *Entity, other: *Entity);
    on_destroy:             (self: *Entity);

    sprite_component:       Maybe.Maybe(Sprite_Component);
    behavior_component:     Maybe.Maybe(*Behavior_Component);
    box_collider_component: Maybe.Maybe(Box_Collider_Component);
    camera_component:       Maybe.Maybe(Camera_Component);
}

operator == :: (a: Entity_Token, b: Entity_Token) -> bool {
    return a.id == b.id && a.generation == b.generation;
}

entity_has_tags :: (entity: Entity, tags: ..string) -> bool {
    has_all_tags := true;

    for tag: tags {
        if !array_find(entity.tags, tag) {
            has_all_tags = false;

            break;
        }
    }

    return has_all_tags;
}

#scope_module

create_entity :: (
    token:  Entity_Token,
    params: Entity_Params
) -> Entity {
    return .{
        token = token,

        name = params.name,
        tags = params.tags,
        transform = params.transform,

        on_start = params.on_start,
        on_update = params.on_update,
        on_collision = params.on_collision,
        on_destroy = params.on_destroy,
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

destroy_entity :: (entity: *Entity) {
    if entity.on_destroy != null {
        entity.on_destroy(entity);
    }

    ok, behavior := world_get_component(entity, Behavior_Component);
    if ok {
        free(behavior);
    }

    entity.signature = 0;

    entity.on_start   = null;
    entity.on_update  = null;
    entity.on_collision  = null;
    entity.on_destroy = null;

    array_free(entity.tags);
}

destroy_entity_params :: (entity_params: Entity_Params) {
    array_free(entity_params.tags);

    if Maybe.is_some(entity_params.behavior_component) {
        free(entity_params.behavior_component.some);
    }
}
