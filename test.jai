#import "Compiler";
#import "Basic";

TESTS_SRC_DEFAULT_ENTRY :: "tests/tests.jai";
TEST_NOTE :: "test";

RUNNER_CODE :: #string DONE
#run {
    print("Running %\n");
    %();
}
DONE

test_procs: [..] *Code_Procedure_Header;

#run {
    set_build_options_dc(.{
        do_output=false,
        write_added_strings = false,
    });

    w := compiler_create_workspace("Tests");

    options := get_build_options(w);
    copy_commonly_propagated_fields(get_build_options(), *options);
    options.output_type = .NO_OUTPUT;
    set_build_options(options, w);

    compiler_begin_intercept(w, .SKIP_EXPRESSIONS_WITHOUT_NOTES);

    add_build_file(get_entry_point(), w);

    message_loop();

    compiler_end_intercept(w);
}

message_loop :: () {
    while true {
        message := compiler_wait_for_message();

        if message.kind == {
            case .TYPECHECKED;
                typechecked := cast(*Message_Typechecked) message;

                for tc: typechecked.procedure_headers {
                    header := tc.expression;
                    if has_note(header, TEST_NOTE) {
                        array_add(*test_procs, header);
                    }
                }

            case .PHASE;
                phase := cast(*Message_Phase) message;

                if phase.phase == .TYPECHECKED_ALL_WE_CAN {
                    for test_procs {
                        s := tprint(RUNNER_CODE, it.name, it.name);
                        add_build_string(s, message.workspace);
                    }

                    array_reset(*test_procs);
                }

            case .COMPLETE;
            break;
        }
    }
}

has_note :: (header: *Code_Procedure_Header, note: string) -> bool {
    for header.notes  if it.text == note  return true;
    return false;
}

get_entry_point :: () -> string {
    args := get_build_options().compile_time_command_line;

    return ifx args.count > 0 {
        args[0];
    } else {
        TESTS_SRC_DEFAULT_ENTRY;
    };
}
