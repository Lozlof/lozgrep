use lozgrep::parse_and_build_arguments::build_running_configuration;

fn main() {
    let passed_options: lozgrep::parse_and_build_arguments::Options = build_running_configuration();

    if passed_options.version == true { print_version() }
    if passed_options.help == true { print_help() }
    
    
    
    
    if passed_options.verbose {println!("Verbose: true")};
    if passed_options.query {println!("Query: true")};
    if passed_options.path {println!("Path: true")};
    if passed_options.simple_grep {println!("Simple-grep: true")};
    if passed_options.simple_find {println!("Simple-find: true")};
    if passed_options.query_item != "null" {println!("Query item: {}", passed_options.query_item)};
    if passed_options.path_item != "null" {println!("Path item: {}", passed_options.path_item)};
}

fn print_version() {
    println!("lozgrep version 0.0.2");
}

fn print_help() { // TODO: Make more descriptive.
    println!("Options:");
    println!("--help          -h       Prints the help menu.");
    println!("--version       -ver     Prints the current version.");
    println!("--verbose       -v       Prints output statements while the process is running.");
    println!("--query         -q       The term you are searching for follows this option.");
    println!("--path          -p       The path you are searching follows this option.");
    println!("--simple-grep   -sg      Searches the contents of a file.");
    println!("--simple-find   -sf      Searches for a file or directory name.");
    println!("");
    println!("Syntax rules:");
    println!("The options can come in any order.");
    println!("The long option (--) or short option (-) can be used interchangeably.");
    println!("");
    println!("Examples:");
    println!("lozgrep -sg -p /home/user/file -q wordiamlookingfor");
    println!("lozgrep --help -ver --query filename --simple-find -p /root");
    println!("");
    println!("Escape character rules:");
    println!("The escape character is: /");
    println!("The escape character can only be used on the value you want to query.");
    println!("Escape character examples:");
    println!("If you need to query for phrase that happens to also be an option");
    println!("lozgrep -sg -q /--help -p /home/user/file");
    println!("The escape character is needed in this example because without it lozgrep will read --help as an option and not an item to look for.");
    println!("Therefore if you need to query for / you need to escape it, otherwise it will be stripped and the query will be empty.");
    println!("lozgrep -sg -q // -p /home/user/file");
}