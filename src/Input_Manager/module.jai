// A simple and probaby very naive implemention of an input manager

#import "Input";

// Holds all the events for the frame
Events :: struct {
    window_resizes: []Window_Resize_Record;
    quit: bool;
}

// Reads all the events for the frame and returns the them
get_events :: () -> Events {
    events: Events;

    update_window_events();

    for events_this_frame {
        if it.type == {
            case .QUIT;
                events.quit = true;
        }
    }

    events.window_resizes = get_window_resizes();

    return events;
}

#scope_file

Input_Manager :: struct {}

manager: Input_Manager = ---;
