package canary

import "core:log"
import "karakuri:karakuri/components"
import karakuri "karakuri:karakuri/game"
import "karakuri:kmath"
import "karakuri:kutils"

main :: proc() {
	game := karakuri.new_game(
		title = "Sonic The Hedgehog",
		width = 800,
		height = 600,
		clear_color = kutils.ColorWhite,
		fullscreen = false,
		vsync = true,
	)
	defer karakuri.destroy_game(game)

	level_1 := karakuri.create_scene(
	{
		// Sonic
		components.Component_Bundle {
			transform = components.new_transform_component(
				position = kmath.Vector2{0, 0},
			),
			shape = components.Shape_Component {
				size = kmath.Vector2{100, 100},
				color = kutils.ColorBlue,
			},
			behavior = components.Behavior_Component {
				on_start = on_player_start,
				on_update = on_player_update,
				on_destroy = on_player_destroy,
			},
		},
		// Tails
		components.Component_Bundle {
			transform = components.new_transform_component(
				position = kmath.Vector2{0, -100},
			),
			shape = components.Shape_Component {
				size = kmath.Vector2{100, 100},
				color = kutils.ColorYellow,
			},
		},
	},
	)

	karakuri.start_scene(&game, &level_1)
}

on_player_start: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	log.log(log.Level.Info, "Player started with dt ", ctx.dt)
}

on_player_update: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	if ctx.input.is_key_pressed(kutils.Key.SPACE) {
		ctx.spawner.add_entity(
			ctx.spawner,
			components.Component_Bundle {
				shape = components.Shape_Component {
					size = kmath.Vector2{10, 50},
					color = kutils.ColorRed,
				},
				transform = components.new_transform_component(
					position = kmath.Vector2{0, -200},
				),
			},
		)
	}

	if ctx.input.is_key_pressed(kutils.Key.X) {
		ctx.spawner.remove_entity(ctx.spawner, ctx.entity)
	}
}

on_player_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	log.log(log.Level.Info, "Player destroyed")
}

