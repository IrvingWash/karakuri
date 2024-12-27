package vector2_tests

import v2 "../../../karakuri/kmath/vector2"
import "core:testing"

X1 :: 2
Y1 :: -34.4
X2 :: 432
Y2 :: 0

@(test)
test_add :: proc(t: ^testing.T) {
	using testing
	using v2

	v := Vector2{X1, Y1}
	other := Vector2{X2, Y2}

	add(&v, other)

	expect(t, v.x == X1 + X2)
	expect(t, v.y == Y1 + Y2)
}
