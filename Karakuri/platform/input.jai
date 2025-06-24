Events :: struct {
    window_resizes: []Input.Window_Resize_Record;
    quit: bool;
}

get_events :: () -> Events {
    events: Events;

    Input.update_window_events();

    for Input.events_this_frame {
        if it.type == {
            case .QUIT;
                events.quit = true;
        }
    }

    events.window_resizes = Input.get_window_resizes();

    return events;
}

#scope_file

Input :: #import "Input";
