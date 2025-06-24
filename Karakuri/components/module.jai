#import "Math";

Transform :: struct {
    position: Vector3;
    scale: Vector3 = .{1, 1, 1};
    rotation: Quaternion;
}

Sprite :: struct {
    color: Vector4;
}
