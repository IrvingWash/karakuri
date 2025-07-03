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
    pixels_per_unit: u32 = 100;
}

Box_Collider_Component :: struct {
    size: Vector2 = .{1, 1};
    offset: Vector2;
}

Camera_Component :: struct {
    size: float = 1;

    front :: Vector3.{0, 0, -1};
    up :: Vector3.{0, 1, 0};
    right :: Vector3.{1, 1, 0};
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
    Box_Collider;
    Camera;
    Behavior;
}

make_view_matrix :: (transform: Transform_Component, camera: Camera_Component) -> Matrix4 {
    position := Vector3.{
        transform.position.x * PIXELS_PER_UNIT,
        transform.position.y * PIXELS_PER_UNIT,
        0,
    };

    return make_look_at_matrix(position, position + camera.front, camera.up, false);
}

component_to_component_type :: inline ($$Component: Type) -> Component_Type {
    if Component == {
        case Transform_Component;
            return .Transform;
        case Sprite_Component;
            return .Sprite;
        case Box_Collider_Component;
            return .Box_Collider;
        case Camera_Component;
            return .Camera;
        case Behavior_Component;
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

