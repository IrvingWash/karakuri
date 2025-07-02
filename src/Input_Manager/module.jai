// A simple and probably very naive implementation of an input manager

#import "Input";

init :: () {
    manager = .{};
}

// Holds all the events for the frame
Events :: struct {
    window_resizes: []Window_Resize_Record;
    quit: bool;
    input_button_states: []Key_Current_State;
}

Key :: enum {
    Space :: #run #char " ";
    A :: #run #char "A";
    D :: #run #char "D";
    P :: #run #char "P";
    S :: #run #char "S";
    W :: #run #char "W";
    Arrow_Up :: 128;
    Arrow_Down :: 129;
    Arrow_Left :: 130;
    Arrow_Right :: 131;
}

is_key_pressed :: (key: Key) -> bool {
    return (manager.events.input_button_states[key] & ~.NONE) != 0;
}

is_key_down :: (key : Key) -> bool {
    return (manager.events.input_button_states[key] & .DOWN) != 0;
}

is_key_start :: (key: Key) -> bool {
    return (manager.events.input_button_states[key] & .START) != 0;
}

is_key_end :: (key: Key) -> bool {
    return (manager.events.input_button_states[key] & .END) != 0;
}

is_key_up :: (key: Key) -> bool {
    return (manager.events.input_button_states[key] & .NONE) != 0;
}

// Reads all the events for the frame and returns the them
get_events :: () -> Events {
    update_window_events();
    manager.events.input_button_states = input_button_states;

    for events_this_frame {
        if it.type == {
            case .QUIT;
                manager.events.quit = true;
        }
    }

    manager.events.window_resizes = get_window_resizes();

    return manager.events;
}

#scope_file

Input_Manager :: struct {
    events: Events;
}

manager: Input_Manager = ---;
