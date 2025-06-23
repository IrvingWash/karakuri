Components :: #import,dir "../components";

Entity :: struct {
    using token: Entity_Token;
    using params: Entity_Params;
}

Entity_Params :: struct {
    transform: Components.Transform;
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

destroy_entity :: (entity: Entity) {}
