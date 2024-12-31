package canary

import "core:fmt"
import "karakuri:karakuri"
import comp "karakuri:karakuri/components"
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

	level_1 := karakuri.scene_new(
		{
			// Player
			comp.Component_Bundle {
				transform = comp.new_transform_component(
					position = v2.Vector2{0, 0},
				),
				shape = comp.Shape_Component {
					width = 100,
					height = 100,
					color = ku.ColorBlue,
				},
				behavior = comp.Behavior_Component {
					on_start = on_player_start,
					on_update = on_player_update,
					on_destroy = on_player_destroy,
				},
			},
		},
	)

	karakuri.start_scene(&game, &level_1)

	karakuri.scene_destroy(level_1)
	karakuri.destroy_game(game)
}

on_player_start: comp.On_Start_Proc : proc(ctx: comp.Behavior_Context) {
	fmt.println("Player started with dt ", ctx.dt)
}

on_player_update: comp.On_Update_Proc : proc(ctx: comp.Behavior_Context) {
	fmt.println("Player updated wth dt ", ctx.dt)
}

on_player_destroy: comp.On_Destroy_Proc : proc(ctx: comp.Behavior_Context) {
	fmt.println("Player destroyed")
}
