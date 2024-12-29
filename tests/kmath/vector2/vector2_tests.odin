package vector2_tests

import v2 "../../../src/kmath/vector2"
import u "../../utils"
import "core:math"
import "core:testing"

X1: f64 : 2
Y1: f64 : -34.4
X2: f64 : 432
Y2: f64 : 0

@(test)
test_add :: proc(t: ^testing.T) {
	using testing
	using v2

	{
		v := Vector2{X1, Y1}
		other := Vector2{X2, Y2}

		add(&v, other)

		expect(t, v.x == X1 + X2)
		expect(t, v.y == Y1 + Y2)
	}

	{
		v := Vector2{X1, Y1}
		other := Vector2{X2, Y2}

		result := add(v, other)

		expect(t, result.x == X1 + X2)
		expect(t, result.y == Y1 + Y2)
	}

	{
		v := Vector2{X1, Y1}

		add(&v, X2)

		expect(t, v.x == X1 + X2)
		expect(t, v.y == Y1 + X2)
	}

	{
		v := Vector2{X1, Y1}

		result := add(v, X2)

		expect(t, result.x == X1 + X2)
		expect(t, result.y == Y1 + X2)
	}
}

@(test)
test_subtract :: proc(t: ^testing.T) {
	using testing
	using v2

	{
		v := Vector2{X1, Y1}
		other := Vector2{X2, Y2}

		subtract(&v, other)

		expect(t, v.x == X1 - X2)
		expect(t, v.y == Y1 - Y2)
	}

	{
		v := Vector2{X1, Y1}
		other := Vector2{X2, Y2}

		result := subtract(v, other)

		expect(t, result.x == X1 - X2)
		expect(t, result.y == Y1 - Y2)
	}

	{
		v := Vector2{X1, Y1}

		subtract(&v, X2)

		expect(t, v.x == X1 - X2)
		expect(t, v.y == Y1 - X2)
	}

	{
		v := Vector2{X1, Y1}

		result := subtract(v, X2)

		expect(t, result.x == X1 - X2)
		expect(t, result.y == Y1 - X2)
	}
}

@(test)
test_scale :: proc(t: ^testing.T) {
	using testing
	using v2

	{
		v := Vector2{X1, Y1}

		scale(&v, X2)

		expect(t, v.x == X1 * X2)
		expect(t, v.y == Y1 * X2)
	}

	{
		v := Vector2{X1, Y1}

		result := scale(v, X2)

		expect(t, result.x == X1 * X2)
		expect(t, result.y == Y1 * X2)
	}

	{
		v := Vector2{X1, Y1}

		scale(&v, Vector2{X2, Y2})

		expect(t, v.x == X1 * X2)
		expect(t, v.y == Y1 * Y2)
	}

	{
		v := Vector2{X1, Y1}

		result := scale(v, Vector2{X2, Y2})

		expect(t, result.x == X1 * X2)
		expect(t, result.y == Y1 * Y2)
	}
}

@(test)
test_divide :: proc(t: ^testing.T) {
	using testing
	using v2

	{
		v := Vector2{X1, Y1}

		divide(&v, X2)

		expect(t, u.are_equal_floats(v.x, X1 / X2))
		expect(t, u.are_equal_floats(v.y, Y1 / X2))

		divide(&v, 0.0)

		expect(t, u.are_equal_floats(v.x, X1 / X2))
		expect(t, u.are_equal_floats(v.y, Y1 / X2))
	}

	{
		v := Vector2{X1, Y1}

		result := divide(v, X2)

		expect(t, u.are_equal_floats(result.x, X1 / X2))
		expect(t, u.are_equal_floats(result.y, Y1 / X2))

		zero_result := divide(v, 0.0)

		expect(t, u.are_equal_floats(zero_result.x, X1))
		expect(t, u.are_equal_floats(zero_result.y, Y1))
	}

	{
		v := Vector2{X1, Y1}

		divide(&v, Vector2{X2, X2})

		expect(t, v.x == X1 / X2)
		expect(t, v.y == Y1 / X2)
	}

	{
		v := Vector2{X1, Y1}

		result := divide(v, Vector2{X2, X2})

		expect(t, result.x == X1 / X2)
		expect(t, result.y == Y1 / X2)
	}
}

@(test)
test_move_towards :: proc(t: ^testing.T) {
	using testing
	using v2

	{
		a := Vector2{X1, Y1}
		b := Vector2{X2, Y2}

		move_towards(&a, b, 10.0)

		expect(t, a.x == 11.96815278536125)
		expect(t, a.y == -33.6025477771711)
	}

	{
		a := Vector2{X1, Y1}
		b := Vector2{X2, Y2}

		result := move_towards(a, b, 10.0)

		expect(t, result.x == 11.96815278536125)
		expect(t, result.y == -33.6025477771711)
	}
}

@(test)
test_set :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{X1, Y1}

	set(&v, Vector2{X2, Y2})

	expect(t, v.x == X2)
	expect(t, v.y == Y2)
}

@(test)
test_rest :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{X1, Y1}

	reset(&v)

	expect(t, v.x == 0)
	expect(t, v.y == 0)
}

@(test)
test_squared_magnitude :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{X1, Y1}

	expect(t, squared_magnitude(v) == 1187.36)
}

@(test)
test_magnitude :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{X1, Y1}

	expect(t, magnitude(v) == 34.458090486850836)
}

@(test)
test_dot :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{X2, Y2}
	other := Vector2{-32., -99.}

	dot_product := dot(v, other)

	expect(t, dot_product == -13824.)
}

@(test)
test_cross :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{432.0, 0}
	other := Vector2{-32., -99}

	cross_product := cross(v, other)

	expect(t, cross_product == -42768)
}

@(test)
test_normalize :: proc(t: ^testing.T) {
	using testing
	using v2

	{
		v := Vector2{-234, 309}

		normalize(&v)

		expect(t, v.x == -0.6037086604052452)
		expect(t, v.y == 0.7972050259197468)
	}

	{
		v := Vector2{-234, 309}

		result := normalize(v)

		expect(t, result.x == -0.6037086604052452)
		expect(t, result.y == 0.7972050259197468)
	}
}

@(test)
test_create_perpendicular :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{-102.23, 34}

	perpendicular := create_perpendicular(v)

	expect(t, perpendicular.x == 0.3155872375021863)
	expect(t, perpendicular.y == 0.9488965673484855)
}

@(test)
test_rotate :: proc(t: ^testing.T) {
	using testing
	using v2

	{
		v := Vector2{34.343, -27}

		rotate(&v, math.to_radians(f64(90)))

		expect(t, v.x == 27.000000000000004)
		expect(t, u.are_equal_floats(v.y, 34.343))
	}

	{
		v := Vector2{34.343, -27}

		result := rotate(v, math.to_radians(f64(90)))

		expect(t, result.x == 27.000000000000004)
		expect(t, u.are_equal_floats(result.y, 34.343))
	}

	{
		v := Vector2{34., -27.}
		pivot := Vector2{34. * 0.5, -27. * 0.5}

		rotate(&v, math.to_radians(f64(90)), pivot)

		expect(t, v.x == 30.5)
		expect(t, v.y == 3.5)
	}

	{
		v := Vector2{34., -27.}
		pivot := Vector2{34. * 0.5, -27. * 0.5}

		result := rotate(v, math.to_radians(f64(90)), pivot)

		expect(t, result.x == 30.5)
		expect(t, result.y == 3.5)
	}
}
