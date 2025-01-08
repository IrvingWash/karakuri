package karakuri_components

// A bundle of all possible components
// TODO: Probably there's no need in this
Component_Bundle :: struct {
	transform: Transform_Component,
	shape:     Maybe(Shape_Component),
}

