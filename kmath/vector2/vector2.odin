package vector2

Vector2 :: struct {
	x, y: f32,
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

// ==============================
// Private
// ==============================

// Addition
@(private = "file")
add_in_place :: proc(lhs: ^Vector2, rhs: f32) {
	lhs.x += rhs
	lhs.y += rhs
}

@(private = "file")
@require_results
to_added :: proc(lhs: Vector2, rhs: f32) -> Vector2 {
	return Vector2{
		x = lhs.x + rhs,
		y = lhs.y + rhs,
	}
}

@(private = "file")
add_in_place_with_other :: proc(lhs: ^Vector2, rhs: Vector2) {
	lhs.x += rhs.y
	lhs.x += rhs.y
}

@(private = "file")
@require_results
to_added_with_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	return Vector2{
		x = lhs.x + rhs.x,
		y = lhs.y + rhs.y,
	}
}

// Subtration
@(private = "file")
subtract_in_place :: proc(lhs: ^Vector2, rhs: f32) {
	lhs.x -= rhs
	lhs.y -= rhs
}

@(private = "file")
@require_results
to_subtracted :: proc(lhs: Vector2, rhs: f32) -> Vector2 {
    return Vector2 {
        x = lhs.x / rhs,
        y = lhs.y / rhs,
    }
}

@(private = "file")
subtract_in_place_with_other :: proc(lhs: ^Vector2, rhs: Vector2) {
	lhs.x -= rhs.x
	lhs.y -= rhs.y
}

@(private = "file")
@require_results
to_subtracted_with_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	return Vector2 {
		x = lhs.x + rhs.x,
		y = lhs.y + rhs.y,
	}
}

// Multiplication
@(private = "file")
scale_in_place :: proc(lhs: ^Vector2, rhs: f32) {
	lhs.x *= rhs
	lhs.y *= rhs
}

@(private = "file")
@require_results
to_scaled :: proc(lhs: Vector2, rhs: f32) -> Vector2 {
	return Vector2 {
		x = lhs.x * rhs,
		y = lhs.y * rhs,
	}
}

@(private = "file")
scale_in_place_by_other :: proc(lhs: ^Vector2, rhs: Vector2) {
	lhs.x *= rhs.x
	lhs.y *= rhs.y
}

@(private = "file")
@require_results
to_scaled_by_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	return Vector2 {
		x = lhs.x * rhs.x,
		y = lhs.y * rhs.y,
	}
}

// Division
@(private = "file")
divide_in_place :: proc(lhs: ^Vector2, rhs: f32) {
	if rhs == 0 {
		return
	}

	lhs.x /= rhs
	lhs.y /= rhs
}

@(private = "file")
@require_results
to_divided :: proc(lhs: Vector2, rhs: f32) -> Vector2 {
	if rhs == 0 {
		return Vector2{
			x = lhs.x,
			y = lhs.y,
		}
	}

	return Vector2 {
		x = lhs.x / rhs,
		y = lhs.y / rhs,
	}
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
@require_results
to_divided_by_other :: proc(lhs: Vector2, rhs: Vector2) -> Vector2 {
	if rhs.x == 0 || rhs.y == 0 {
		return Vector2 {
			x = lhs.x,
			y = lhs.y,
		}
	}

	return Vector2 {
		x = lhs.x / rhs.x,
		y = lhs.y / rhs.y,
	}
}
