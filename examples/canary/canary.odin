package canary

import "core:fmt"
import "karakuri:karakuri"
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

	level_1 := karakuri.create_scene(
		{
			// Player
			karakuri.Component_Bundle {
				transform = karakuri.new_transform_component(
					position = v2.Vector2{0, 0},
				),
				shape = karakuri.Shape_Component {
					width = 100,
					height = 100,
					color = ku.ColorBlue,
				},
				behavior = karakuri.Behavior_Component {
					on_start = on_player_start,
					on_update = on_player_update,
					on_destroy = on_player_destroy,
				},
			},
		},
	)

	karakuri.start_scene(&game, &level_1)

	karakuri.destroy_game(game)
}

on_player_start: karakuri.On_Start_Proc : proc(
	ctx: karakuri.Behavior_Context,
) {
	fmt.println("Player started with dt ", ctx.dt)
}

on_player_update: karakuri.On_Update_Proc : proc(
	ctx: karakuri.Behavior_Context,
) {
	if ctx.input.is_key_pressed(ku.Key.SPACE) {
		ctx.spawner.add_entity(ctx.spawner, karakuri.Component_Bundle{
			shape = karakuri.Shape_Component{
				width = 10,
				height = 50,
				color = ku.ColorRed,
			},
			transform = karakuri.Transform_Component {
				position = v2.Vector2{0, -200},
			},
		})
	}

	if ctx.input.is_key_pressed(ku.Key.X) {
		ctx.spawner.remove_entity(ctx.spawner, ctx.entity)
	}
}

on_player_destroy: karakuri.On_Destroy_Proc : proc(
	ctx: karakuri.Behavior_Context,
) {
	fmt.println("Player destroyed")
}
