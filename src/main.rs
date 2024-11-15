use lozgrep::parse_and_build_arguments::build_running_configuration;

fn main() {
    let passed_options: lozgrep::parse_and_build_arguments::Options = build_running_configuration();

    if passed_options.help {println!("Help: true")};
    if passed_options.version {println!("Version: true")};
    if passed_options.verbose {println!("Verbose: true")};
    if passed_options.query {println!("Query: true")};
    if passed_options.path {println!("Path: true")};
    if passed_options.simple_grep {println!("Simple-grep: true")};
    if passed_options.simple_find {println!("Simple-find: true")};
    if passed_options.query_item != "null" {println!("Query item: {}", passed_options.query_item)};
    if passed_options.path_item != "null" {println!("Path item: {}", passed_options.path_item)};
}

/* Options:
help: bool,
version: bool,
verbose: bool,
query: bool,
path: bool,
simple_grep: bool,
simple_find: bool,
query_item: String,
path_item: String, */
