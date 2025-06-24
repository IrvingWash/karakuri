#import "Math";

// Represents transformations of an object
Transform :: struct {
    position: Vector3;
    scale: Vector3 = .{1, 1, 1};
    rotation: Quaternion;
}

// Represents a sprite
Sprite :: struct {
    color: Vector4;
}

// Represents a behavior where all the specific data can be placed
Behavior :: struct {}

#scope_module

Component_Type :: enum u64 {
    Transform;
    Sprite;
    Behavior;
}

component_to_component_type :: inline ($Component: Type) -> Component_Type
{
    #if Component == Transform {
        return .Transform;
    } else #if Component == Sprite {
        return .Sprite;
    } else #if Component == Behavior {
        return .Behavior;
    }
}

is_component :: ($T: Type) -> bool {
    return T == Transform || T == Sprite || T == Behavior;
}
