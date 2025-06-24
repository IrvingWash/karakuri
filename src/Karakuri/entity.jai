#import Maybe;

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
    behavior_component: Maybe(*Behavior);
}

#scope_module

create_entity :: (
    token: Entity_Token,
    signature: u32,
    params: Entity_Params
) -> Entity {
    return .{
        token = token,

        signature = signature,

        transform = params.transform,

        on_start = params.on_start,
        on_update = params.on_update,
    };
}
