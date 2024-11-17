use lozgrep::parse_and_build_arguments::build_running_configuration;
use lozgrep::print_to_terminal::{print_help, print_version};
use lozgrep::execute_main_operations::{simple_grep, simple_find};

// TODO: Have two query items. Use find first to reduce down the files. And then use a grep query to search the found files.
fn main() { // As long as the options all pass through build_running_configuration
    let passed_options: lozgrep::parse_and_build_arguments::Options = build_running_configuration(); // Calls on build_running_configuration to parse, check, and organize all the arguments passed. If there are no errors, passed_options will be assigned a struct that holds the status of all the possible options and values.
    if passed_options.verbose == true { println!("VERBOSE: Collected {:?}", &passed_options); } // If verbose is true, the debug implementation of Options will be used to print all the values of passed_options.

    if passed_options.version == true { print_version(&passed_options.verbose) } // If version is true, will call on print_version to print the version. Verbose is also passed so print_version can print the verbose output if true.

    if passed_options.help == true { print_help(&passed_options.verbose) } // If help is true, will call on print_help to print the version. Verbose is also passed so print_help can print the verbose output if true.

    if passed_options.simple_grep == true { // If everything goes right, simple_grep will terminate the process, therefore no return is needed.
        if passed_options.verbose == true { println!("VERBOSE: Calling on the simple_grep function"); }
        simple_grep(&passed_options.query_item, &passed_options.path_item, &passed_options.verbose);
    }

    if passed_options.simple_find == true { // If everything goes right, simple_find will terminate the process, therefore no return is needed.
        if passed_options.verbose == true { println!("VERBOSE: Calling on the simple_find function"); }
        simple_find(&passed_options.query_item, &passed_options.path_item, &passed_options.verbose);
    }    

    if passed_options.verbose == true { println!("VERBOSE: End of process, now exiting"); }
    std::process::exit(1); // If the simple-grep or simple-find options are not used, the process should end here.
}