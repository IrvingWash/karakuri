package karakuri_components

import "kwindow:renderer"

Sprite_Component :: struct {
	using sprite:  renderer.Sprite,
	sprite_name:   string,
	sorting_layer: uint,
}

