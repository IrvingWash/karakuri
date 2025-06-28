#import "Basic";
#import "Compiler";
#import "File";
#import "File_Utilities";
#import "Process";

#run {
    #if OS != .LINUX && OS != .MACOS {
        print("Sorry, Windows is not supported\n", to_standard_error = true);
        exit(1);
    }

    set_build_options_dc(.{
        do_output = false,
    });

    if !file_exists("../.git") {
        print("Not a git repository. Aborting\n", to_standard_error = true);
        exit(1);
    }

    write_entire_file("../.git/hooks/pre-commit", PRE_COMMIT);

    run_command(
        "chmod",
        "u+x",
        "../.git/hooks/pre-commit",
    );
}

PRE_COMMIT :: #string BASH
#!/bin/sh

cspell lint ../../ --gitignore
BASH
