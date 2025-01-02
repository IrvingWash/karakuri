package components

Component_Bundle :: struct {
	transform:  Maybe(Transform_Component),
	shape:      Maybe(Shape_Component),
	behavior:   Maybe(Behavior_Component),
	rigid_body: Maybe(Rigid_Body_Component),
	tag:        Maybe(Tag_Component),
}
