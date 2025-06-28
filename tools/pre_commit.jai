#import "Basic";
#import "Compiler";
#import "File";
#import "File_Utilities";

#run {
    set_build_options_dc(.{
        do_output = false,
    });

    if !file_exists("../.git") {
        print("Not a git repository. Aborting\n", to_standard_error = true);
    }

    write_entire_file("../.git/hooks/pre-commit", PRE_COMMIT);
}

PRE_COMMIT :: #string BASH
#!/bin/sh

cspell src tests examples
BASH
