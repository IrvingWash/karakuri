package example_canary

import "core:fmt"
import "karakuri:entity_manager"

Player :: struct {
	using entity: entity_manager.Entity,
	hp:           uint,
}

main :: proc() {
	emi := entity_manager.init()
	defer entity_manager.deinit(emi)

	player := new(Player)
	player^ = Player {
		transform = {position = {300, 50}},
		hp = 100,
	}

	player_id := entity_manager.create_entity(&emi, player)

	player_gotten := entity_manager.get_entity(emi, player_id, Player)

	if p, ok := player_gotten.?; ok {
		fmt.println(p.transform.position)
		fmt.println(p.hp)
	}
}

