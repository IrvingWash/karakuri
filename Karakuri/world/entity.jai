Components :: #import,dir "../components";

Entity :: struct {
    using token: Entity_Token;
    using params: Entity_Params;
}

Entity_Params :: struct {
    transform: Components.Transform;
    behavior: *Behavior;
}

Entity_Token :: struct {
    id: u32;
    generation: u32;
}

Behavior :: struct {
    on_start: (self: *Entity);
    on_update: (self: *Entity, dt: float);
}

// instantiate_behavior :: ($T: Type, v: T) -> *T {
// }

#scope_module

create_entity :: (token: Entity_Token, params: Entity_Params) -> Entity {
    return .{
        token = token,
        params = params,
    };
}

destroy_entity :: (entity: Entity) {}
