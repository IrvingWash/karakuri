Components :: #import,dir "../components";

Entity :: struct {
    using token: Entity_Token;
    using params: Entity_Params;
}

Entity_Params :: struct {
    transform: Components.Transform;
    behavior: *Behavior;
    on_start: (self: *Entity);
    on_update: (self: *Entity, dt: float);
}

Entity_Token :: struct {
    id: u32;
    generation: u32;
}

#scope_module

create_entity :: (token: Entity_Token, params: Entity_Params) -> Entity {
    return .{
        token = token,
        params = params,
    };
}

destroy_entity :: (entity: *$T/interface Entity_Params) {
    if entity.behavior != null {
        free(entity.behavior);
    }
}
