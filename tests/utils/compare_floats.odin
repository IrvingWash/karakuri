package test_utils

import "core:math"

are_equal_floats :: proc(a, b: f64) -> bool {
	using math

	return abs(a - b) < F64_EPSILON
}
