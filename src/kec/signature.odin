#+private

package kec

// A `bit_set` that can store up to 128 values.
// Used to efficiently store info about which components are attached to an entity.
Signature :: distinct bit_set[0 ..= 127;u128]
