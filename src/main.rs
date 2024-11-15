use lozgrep::parse_and_build_arguments::build_running_configuration;
use lozgrep::print_to_terminal::{print_help, print_version};
use lozgrep::execute_main_operations::simple_grep;

fn main() {
    let passed_options: lozgrep::parse_and_build_arguments::Options = build_running_configuration();
    if passed_options.verbose == true { println!("VERBOSE: Collected {:?}", &passed_options); }

    if passed_options.version == true { print_version() }
    if passed_options.verbose == true { println!("VERBOSE: Printed version"); } 

    if passed_options.help == true { print_help() }
    if passed_options.verbose == true { println!("VERBOSE: Printed help menu"); }

    if passed_options.simple_grep == true { 
        simple_grep(&passed_options.query_item, &passed_options.path_item);
    }

    std::process::exit(1);
    
    
    /*if passed_options.verbose {println!("Verbose: true")};
    if passed_options.query {println!("Query: true")};
    if passed_options.path {println!("Path: true")};
    
    if passed_options.simple_find {println!("Simple-find: true")};*/

}