Key :: enum {
    A :: #run #char "A";
    D :: #run #char "D";
}

is_key_pressed :: (key: Key) -> bool {
    return (input.events.input_button_states[key] & (.DOWN | .START | .END)) != 0;
}

is_key_down :: (key : Key) -> bool {
    return (input.events.input_button_states[key] & .DOWN) != 0;
}

is_key_start :: (key: Key) -> bool {
    return (input.events.input_button_states[key] & .START) != 0;
}

is_key_end :: (key: Key) -> bool {
    return (input.events.input_button_states[key] & .END) != 0;
}

is_key_up :: (key: Key) -> bool {
    return (input.events.input_button_states[key] & .NONE) != 0;
}

#scope_module

input_set_events :: (events: Events) {
    input.events = events;
}

#scope_file

Input :: struct {
    events: Events;
}

input: Input;
