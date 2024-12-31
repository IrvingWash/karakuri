package canary

import "core:fmt"
import "karakuri:karakuri"
import c "karakuri:karakuri/components"
import v2 "karakuri:kmath/vector2"
import ku "karakuri:kutils"

main :: proc() {
	game := karakuri.new_game(
		title = "Sonic The Hedgehog",
		width = 800,
		height = 600,
		clear_color = ku.ColorWhite,
		fullscreen = false,
		vsync = true,
	)

	level_1 := karakuri.new_scene(
		{
			// Player
			c.Component_Bundle {
				transform = c.new_transform_component(
					position = v2.Vector2{0, 0},
				),
				shape = c.Shape_Component {
					width = 100,
					height = 100,
					color = ku.ColorBlue,
				},
				behavior = c.Behavior_Component {
					on_start = on_player_start,
					on_update = on_player_update,
				},
			},
		},
	)

	karakuri.start_scene(&game, &level_1)

	karakuri.destroy_scene(level_1)
	karakuri.destroy_game(game)
}

on_player_start: c.On_Start_Proc : proc(ctx: c.Behavior_Context) {
	fmt.println("Player started with dt ", ctx.dt)
}

on_player_update: c.On_Update_Proc : proc(ctx: c.Behavior_Context) {
}
