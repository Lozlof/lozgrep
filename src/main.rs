use lozgrep::parse_and_build_arguments::build_running_configuration;
use lozgrep::print_to_terminal::{print_version, print_help};
use lozgrep::execute_main_operations::simple_grep;

fn main() {
    let passed_options: lozgrep::parse_and_build_arguments::Options = build_running_configuration();

    if passed_options.version == true { print_version() }
    if passed_options.help == true { print_help() }

    if passed_options.simple_grep == true { 
        /*if simple_grep(&passed_options.query_item, &passed_options.path_item) == true { // If simple_grep returns true, then exit.

            if passed_options.query_item != "null" {println!("Query item: {}", passed_options.query_item)};
            if passed_options.path_item != "null" {println!("Path item: {}", passed_options.path_item)};
            std::process::exit(1);
        }*/

        // simple_search(&passed_options.query_item, &passed_options.file_content);

        simple_grep(&passed_options.query_item, &passed_options.path_item);
    }

    
    
    
    
    
    if passed_options.verbose {println!("Verbose: true")};
    if passed_options.query {println!("Query: true")};
    if passed_options.path {println!("Path: true")};
    
    if passed_options.simple_find {println!("Simple-find: true")};

}

fn simple_search<'a>(search_query: &String, search_contents: &'a String) {
    let mut search_results: Vec<&str> = Vec::new();
    
    for line in search_contents.lines() { 
        if line.contains(search_query) { 
            search_results.push(line);
        }
    }

    if !&search_results.is_empty() { // If &search_results in not empty.
        let search_results_status: String = format!("lozgrep found lines in the given file that matched the query parameters. Found items not written to log to reduce clutter");
        // lozgrep::write_to_log_file(&search_results_status); // Writes a log entry saying that lozgrep found a match.
        println!("{}", search_results_status);

    } else {
        let search_results_status: String = format!("lozgrep did not find any lines in the given file that matched the query parameters");
        // lozgrep::write_to_log_file(&search_results_status); // Writes a log entry saying that lozgrep did not find a match.
        println!("{}", search_results_status);
    }

    simple_print_and_exit(&search_results);
}

fn simple_print_and_exit(found_lines: &Vec<&str>) {
    
    for item in found_lines {
        println!("{}", item);
    }

    std::process::exit(1); // End of process.
}