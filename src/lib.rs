pub mod parse_and_build_arguments {
    use std::env;
    use std::process;

    pub fn build_running_configuration() {
        let collected_arguments: Vec<String> = env::args().skip(1).collect(); // Will collect passed arguments and put them into a vector. Does not collect the first passed argument, because it is not needed.
        let possible_options: [&str; 14] = ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"]; // These are all the valid options.
        verify_argument_length(&collected_arguments); // Checks if zero arguments are passed, checks if too many arguments are passed, error in either senario.
        verify_options_are_valid(&collected_arguments, &possible_options); // Filters and collects all options (--, -) from the arguments. Compares the filtered options to possible_options to verify the given options. Creates errors if bad options are present.
    }

    fn verify_argument_length(borrow_collected_arguments: &Vec<String>) {
        if borrow_collected_arguments.len() == 0 { // If no arguments are passed, it is an error.
            println!("Invalid syntax. Zero arguments were passed. Use \"--help\" or \"-h\" to see options and syntax.");
            process::exit(1);
        }
    
        if borrow_collected_arguments.len() > 20 { // If too many arguments are passed, it is an error.
            println!("Invalid syntax. Too many arguments were passed. Use \"--help\" or \"-h\" to see options and syntax."); // TODO: Make this number more specific to what the actual max is..
            process::exit(1);
        }
    }

    fn verify_options_are_valid (borrow_collected_arguments: &Vec<String>, borrow_possible_options: &[&str; 14]) {
        let filtered_options: Vec<String> = borrow_collected_arguments // Parses through all the collected arguments and pulls out any options (-- -).
        .iter() // creates an iterator.
        .filter(|option| option.starts_with("--") || option.starts_with("-")) // .filter(...) is used to retain only items that satisfy a given condition. |option| is a closure (anonymous function) parameter representing each item passed from the iterator. Checks if the String starts with -- or -.
        .cloned() // Is used to convert &String (a reference) into an owned String. This is necessary because we want to create a new vector with owned String values, rather than references to the original vector’s items
        .collect(); //  Takes the filtered and cloned items from the iterator and collects them into a new Vec<String>. This newly created vector is then assigned to filtered_options.
        
        let bad_options: Vec<String> = filtered_options// Parses through all the filtered options and checks if they are actual valid options.
        .iter()
        .filter(|option| !borrow_possible_options.contains(&option.as_str())) // Filters by options that are not contained inside of borrow_all_possible_options.
        .cloned()
        .collect();

        if !bad_options.is_empty() { // If bad_options is not empty, then it means that bad option were passed.
            let print_bad_options: String = bad_options.join(", "); // Turns the values of &bad_options into a string so a clear error message can be printed.
            
            if bad_options.len() == 1 { // Different error messages depending on the situation.
                println!("Invalid syntax. An unknown option was passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", &print_bad_options);
                process::exit(1);
    
            } else {
                println!("Invalid syntax. Unknown options were passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", &print_bad_options);
                process::exit(1);
            }
        }
    }
}