#import "Math";

// Represents transformations of an object
Transform_Component :: struct {
    position: Vector3;
    scale: Vector3 = .{1, 1, 1};
    rotation: Quaternion;
}

// Represents a sprite
Sprite_Component :: struct {
    color: Vector4;
}

// Represents a behavior where all the specific data can be placed
Behavior_Component :: struct {}

#scope_module

Component_Type :: enum u64 {
    Unknown;
    Transform;
    Sprite;
    Behavior;
}

component_to_component_type :: inline ($Component: Type) -> Component_Type {
    #if Component == Transform_Component {
        return .Transform;
    } else #if Component == Sprite_Component {
        return .Sprite;
    } else #if Component == Behavior_Component {
        return .Behavior;
    }

    return .Unknown;
}
