#import "Math";

// Represents transformations of an object
Transform_Component :: struct {
    position: Vector2;
    scale: Vector2 = .{1, 1};
    rotation: float;
}

// Represents a sprite
Sprite_Component :: struct {
    texture_path: string;
    color: Vector4 = .{1, 1, 1, 1};
    origin: Vector2;
    texture: Sprite_Renderer.Texture = ---;
    flip: struct {
        x: bool;
        y: bool;
    };
}

// Represents a behavior where all the specific data can be placed
Behavior_Component :: struct {}

instantiate_behavior :: (v: $B) -> *Behavior_Component
#modify { return uses_behavior_with_as(B); }
{
    behavior: *B = New(B);
    behavior.* = v;

    return behavior;
}

#scope_module

Component_Type :: enum_flags u64 {
    Unknown;
    Transform;
    Sprite;
    Behavior;
}

component_to_component_type :: inline ($$Component: Type) -> Component_Type {
    if Component == Transform_Component {
        return .Transform;
    } else if Component == Sprite_Component {
        return .Sprite;
    } else if Component == Behavior_Component {
        return .Behavior;
    }

    return .Unknown;
}

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

        if member.type != type_info(Behavior_Component) {
            continue;
        }

        return true;
    }

    return false;
}

