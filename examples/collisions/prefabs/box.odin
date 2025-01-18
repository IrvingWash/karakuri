package example_collisions_prefabs

import "core:log"
import "kutils:color"
import v2 "kmath:vector2"
import "karakuri:world"
import "karakuri:input_manager"
import "karakuri:components"

Box_Kind :: enum {
	Static,
	Dynamic,
}

box_prefab :: proc(kind: Box_Kind) -> world.Entity_Payload {
	behavior := new(Box_Behavior)
	behavior^ = Box_Behavior {
		on_update    = on_update,
		on_collision = on_collision,
		kind         = kind,
	}

	return world.Entity_Payload {
		behavior = behavior,
		transform = components.Transform_Component {
			position = kind == .Dynamic ? {-100, 0} : {100, 50},
			scale = kind == .Dynamic ? {0.75, 1} : {1.5, 0.5},
		},
		sprite = components.Sprite_Component {
			sprite_name = "square",
			tint = kind == .Dynamic ? color.White : color.Yellow,
			origin = kind == .Dynamic ? v2.Vector2{64 * 0.75 / 2, 64} : nil,
		},
		box_collider = components.Box_Collider_Component{offset = {10, 10}},
	}
}

@(private = "file")
Box_Behavior :: struct {
	using behavior: world.Behavior,
	kind:           Box_Kind,
}

@(private = "file")
on_update: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	behavior := world.get_behavior(ctx.self^, Box_Behavior).?

	if sprite, ok := &ctx.self.sprite.?; ok {
		if sprite.tint == color.Red {
			sprite.tint = behavior.kind == .Static ? color.Yellow : color.White
		}
	}

	if behavior.kind == .Static {
		return
	}

	ctx.self.transform.position = input_manager.get_cursor_position()
}

@(private = "file")
on_collision: world.On_Collision_Proc : proc(
	ctx: world.Behavior_Context,
	other: ^world.Entity,
) {
	log.info("Collision!")
	sprite := &ctx.self.sprite.?
	sprite.tint = color.Red
}

