package registry_tests

import "../../karakuri/kec"
import "core:testing"

@(test)
test_create_entity :: proc(t: ^testing.T) {
	using testing
	using kec

	r := new_registry()

	sonic := create_entity(&r)
	tails := create_entity(&r)
	knuckles := create_entity(&r)

	expect(t, sonic == 0)
	expect(t, tails == 1)
	expect(t, knuckles == 2)

	expect(t, r.next_entity == 3)
}
