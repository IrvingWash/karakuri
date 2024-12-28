package vector2

import "core:math"

Vector2 :: struct {
	x, y: f64,
}

ZERO :: Vector2 {
	x = 0,
	y = 0,
}

UNIT :: Vector2 {
	x = 1,
	y = 1,
}

add :: proc {
	add_in_place,
	to_added,
	add_in_place_with_other,
	to_added_with_other,
}

subtract :: proc {
	subtract_in_place,
	to_subtracted,
	subtract_in_place_with_other,
	to_subtracted_with_other,
}

scale :: proc {
	scale_in_place,
	to_scaled,
	scale_in_place_by_other,
	to_scaled_by_other,
}

divide :: proc {
	divide_in_place,
	to_divided,
	divide_in_place_by_other,
	to_divided_by_other,
}

set :: proc(v2: ^Vector2, other: Vector2) {
	v2.x = other.x
	v2.y = other.y
}

reset :: proc(v2: ^Vector2) {
	v2.x = 0
	v2.y = 0
}

squared_magnitude :: proc(v2: Vector2) -> f64 {
	using math

	return pow(v2.x, 2) + pow(v2.y, 2)
}

magnitude :: proc(v2: Vector2) -> f64 {
	return math.sqrt(squared_magnitude(v2))
}

dot :: proc(v2: Vector2, other: Vector2) -> f64 {
	return v2.x * other.x + v2.y * other.y
}

cross :: proc(v2: Vector2, other: Vector2) -> f64 {
	return v2.x * other.y - v2.y * other.x
}

normalize :: proc {
	normalize_in_place,
	to_normalized,
}

create_perpendicular :: proc(v2: Vector2) -> Vector2 {
	result := Vector2 {
		x = v2.y,
		y = -v2.x,
	}

	normalize(&result)

	return result
}

rotate :: proc {
	rotate_in_place,
	to_rotated,
	rotate_at_in_place,
	to_rotated_at,
}

move_towards :: proc {
	move_towards_in_place,
	to_moved_towards,
}

// ==============================
// Private
// ==============================

// Addition
@(private = "file")
add_in_place :: proc(lhs: ^Vector2, rhs: f64) {
	lhs.x += rhs
	lhs.y += rhs
}

@(private = "file")
@(require_results)
to_added :: proc(lhs: Vector2, rhs: f64) -> Vector2 {
	return Vector2{x = lhs.x + rhs, y = lhs.y + rhs}
}

@(private = "file")
add_in_place_with_other :: proc(lhs: ^Vector2, rhs: Vector2) {
	lhs.x += rhs.x
	lhs.y += rhs.y
}

@(private = "file")
@(require_results)
to_added_with_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	return Vector2{x = lhs.x + rhs.x, y = lhs.y + rhs.y}
}

// Subtration
@(private = "file")
subtract_in_place :: proc(lhs: ^Vector2, rhs: f64) {
	lhs.x -= rhs
	lhs.y -= rhs
}

@(private = "file")
@(require_results)
to_subtracted :: proc(lhs: Vector2, rhs: f64) -> Vector2 {
	return Vector2{x = lhs.x - rhs, y = lhs.y - rhs}
}

@(private = "file")
subtract_in_place_with_other :: proc(lhs: ^Vector2, rhs: Vector2) {
	lhs.x -= rhs.x
	lhs.y -= rhs.y
}

@(private = "file")
@(require_results)
to_subtracted_with_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	return Vector2{x = lhs.x - rhs.x, y = lhs.y - rhs.y}
}

// Multiplication
@(private = "file")
scale_in_place :: proc(lhs: ^Vector2, rhs: f64) {
	lhs.x *= rhs
	lhs.y *= rhs
}

@(private = "file")
@(require_results)
to_scaled :: proc(lhs: Vector2, rhs: f64) -> Vector2 {
	return Vector2{x = lhs.x * rhs, y = lhs.y * rhs}
}

@(private = "file")
scale_in_place_by_other :: proc(lhs: ^Vector2, rhs: Vector2) {
	lhs.x *= rhs.x
	lhs.y *= rhs.y
}

@(private = "file")
@(require_results)
to_scaled_by_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	return Vector2{x = lhs.x * rhs.x, y = lhs.y * rhs.y}
}

// Division
@(private = "file")
divide_in_place :: proc(lhs: ^Vector2, rhs: f64) {
	if rhs == 0 {
		return
	}

	lhs.x /= rhs
	lhs.y /= rhs
}

@(private = "file")
@(require_results)
to_divided :: proc(lhs: Vector2, rhs: f64) -> Vector2 {
	if rhs == 0 {
		return Vector2{x = lhs.x, y = lhs.y}
	}

	return Vector2{x = lhs.x / rhs, y = lhs.y / rhs}
}

@(private = "file")
divide_in_place_by_other :: proc(lhs: ^Vector2, rhs: Vector2) {
	if rhs.x == 0 || rhs.y == 0 {
		return
	}

	lhs.x /= rhs.x
	lhs.y /= rhs.y
}

@(private = "file")
@(require_results)
to_divided_by_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	if rhs.x == 0 || rhs.y == 0 {
		return Vector2{x = lhs.x, y = lhs.y}
	}

	return Vector2{x = lhs.x / rhs.x, y = lhs.y / rhs.y}
}

// Normalization
@(private = "file")
normalize_in_place :: proc(v2: ^Vector2) {
	magnitude := magnitude(v2^)

	divide(v2, magnitude)
}

@(private = "file")
@(require_results)
to_normalized :: proc(v2: Vector2) -> Vector2 {
	result := Vector2 {
		x = v2.x,
		y = v2.y,
	}

	normalize(&result)

	return result
}

// Rotation
@(private = "file")
rotate_in_place :: proc(v2: ^Vector2, radians: f64) {
	cos := math.cos(radians)
	sin := math.sin(radians)

	x := v2.x * cos - v2.y * sin
	y := v2.x * sin + v2.y * cos

	v2.x = x
	v2.y = y
}

@(private = "file")
@(require_results)
to_rotated :: proc(v2: Vector2, radians: f64) -> Vector2 {
	result := Vector2 {
		x = v2.x,
		y = v2.y,
	}

	rotate(&result, radians)

	return result
}

@(private = "file")
rotate_at_in_place :: proc(v2: ^Vector2, radians: f64, pivot: Vector2) {
	x := v2.x - pivot.x
	y := v2.y - pivot.y

	temporary_vector := Vector2{x, y}

	rotate(&temporary_vector, radians)
	add(&temporary_vector, pivot)

	set(v2, temporary_vector)
}

@(private = "file")
to_rotated_at :: proc(v2: Vector2, radians: f64, pivot: Vector2) -> Vector2 {
	result := Vector2 {
		x = v2.x,
		y = v2.y,
	}

	rotate_at_in_place(&result, radians, pivot)

	return result
}

// Moving
@(private = "file")
move_towards_in_place :: proc(
	v2: ^Vector2,
	other: Vector2,
	max_distance: f64,
) {
	disposition := subtract(other, v2^)

	magnitude := magnitude(disposition)

	if magnitude <= max_distance || magnitude == 0.0 {
		return
	}

	divide(&disposition, magnitude)
	scale(&disposition, max_distance)
	add(&disposition, v2^)

	set(v2, disposition)
}

@(private = "file")
@(require_results)
to_moved_towards :: proc(
	v2: Vector2,
	other: Vector2,
	max_distance: f64,
) -> Vector2 {
	result := Vector2 {
		x = v2.x,
		y = v2.y,
	}

	move_towards_in_place(&result, other, max_distance)

	return result
}
