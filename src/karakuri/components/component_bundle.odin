package components

Component_Bundle :: struct {
	transform: Maybe(Transform_Component),
	shape:     Maybe(Shape_Component),
	behavior:  Maybe(Behavior_Component),
}
