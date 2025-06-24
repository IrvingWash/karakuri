#import "Basic";
#import "Compiler";
#import "File";

BUILD_PATH_BASE :: "build";
ENTRY_POINT_PATH_BASE :: "examples";

Config :: struct {
    example: string;
    mode: Mode = .Debug;
}

Mode :: enum {
    Debug;
    Release;
}

#run {
    set_build_options_dc(.{
        do_output = false,
        write_added_strings = false,
    });

    build_options_dc := get_build_options();

    config := make_config_from_args(build_options_dc.compile_time_command_line);

    w := compiler_create_workspace(config.example);

    build(w, config, build_options_dc);

    reset_temporary_storage();
}

make_config_from_args :: (args: []string) -> Config {
    config: Config;

    for arg: args {
        if arg == {
            case "--example";
                if args.count >= it_index + 2 {
                    config.example = args[it_index + 1];
                }
            case "--release";
                config.mode = .Release;
        }
    }

    return config;
}

build :: (w: int, config: Config, build_options_dc: Build_Options) {
    if config.example == "" {
        print("No example name provided. Aborting\n", to_standard_error = true);
        return;
    }

    build_dir := tprint(
        "%/%/%",
        BUILD_PATH_BASE,
        config.example,
        config.mode
    );

    make_directory_if_it_does_not_exist(build_dir, recursive = true);

    build_options := get_build_options(w);
    copy_commonly_propagated_fields(build_options_dc, *build_options);
    build_options.output_type = .EXECUTABLE;
    build_options.output_executable_name = config.example;
    build_options.output_path, build_options.intermediate_path = build_dir;
    build_options.import_path = make_import_paths(build_options.import_path);
    set_optimization(
        *build_options,
        mode_to_optimization_type(config.mode)
    );

    set_build_options(build_options, w);

    add_build_file(tprint("%/%/main.jai", ENTRY_POINT_PATH_BASE, config.example), w);
}

make_import_paths :: (import_paths: []string) -> []string {
    result: [..]string;
    result.allocator = temp;

    for path: import_paths {
        array_add(*result, path);
    }

    array_add(*result, "src");

    return result;
}

mode_to_optimization_type :: (mode: Mode) -> Optimization_Type {
    if #complete mode == {
        case .Debug;
            return .DEBUG;
        case .Release;
            return .VERY_OPTIMIZED;
    }
}
