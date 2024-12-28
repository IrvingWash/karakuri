package canary

import c "karakuri:kcore/components"
import "karakuri:kcore/game"
import "karakuri:kec"
import ku "karakuri:kutils"
import im "karakuri:kwindow/input_manager"

main :: proc() {
	// Init the game
	my_game := game.new_game(
		"Sonic",
		clear_color = ku.ColorWhite,
		fullscreen = false,
	)
	defer game.destroy_game()

	// Create the player and add components
	player := kec.create_entity(&my_game.registry)
	kec.add_component(
		&my_game.registry,
		player,
		c.new_transform_component({100, 100}, {2, 2}, 0),
	)
	kec.add_component(
		&my_game.registry,
		player,
		c.Shape_Component{10, 20, ku.ColorBlue},
	)
	kec.add_component(
		&my_game.registry,
		player,
		c.Behavior_Component{on_update = on_player_update},
	)

	// Start the game
	game.start(&my_game)
}

on_player_update: c.On_Update_Fn : proc(ctx: c.Behavior_Context) {
	SPEED :: 500

	transform := kec.get_component(
		ctx.registry,
		ctx.entity,
		c.Transform_Component,
	)

	if ctx.is_key_down(im.Key.W) {
		transform.position.y -= SPEED * ctx.dt
	}
	if ctx.is_key_down(im.Key.S) {
		transform.position.y += SPEED * ctx.dt
	}
	if ctx.is_key_down(im.Key.A) {
		transform.position.x -= SPEED * ctx.dt
	}
	if ctx.is_key_down(im.Key.D) {
		transform.position.x += SPEED * ctx.dt
	}
}
