package kwindow_renderer

import rl "vendor:raylib"
import "kutils:color"
import v2 "kmath:vector2"

Texture :: rl.Texture2D

Sprite :: struct {
	texture:       Texture,
	clip_position: v2.Vector2,
	clip_size:     Maybe(v2.Vector2),
	origin:        Maybe(v2.Vector2), // TODO
	flip:          struct {
		x: bool,
		y: bool,
	},
	tint:          Maybe(color.Color),
}

