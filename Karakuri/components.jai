#import "Math";

Transform_Component :: struct {
    position: Vector3;
    scale: Vector3 = .{1, 1, 1};
    rotation: Quaternion;
}
