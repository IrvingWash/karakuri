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

destroy_entity :: (entity: Entity) {}

uses_behavior_with_as :: (T: Type) -> bool #compile_time {
    ti := cast(*Type_Info) T;

    if ti.type != .STRUCT {
        return false;
    }

    tis := cast(*Type_Info_Struct) ti;

    for member: tis.members {
        if !(member.flags & .AS) {
            continue;
        }

        if member.type != type_info(Behavior) {
            continue;
        }

        return true;
    }

    return false;
}
