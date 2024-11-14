pub mod parse_and_build_arguments {
    use std::env;
    use std::process;
    use std::collections::HashSet;
    use std::fs;
    use std::io;
    use std::path::Path;

    pub fn build_running_configuration() {
        let collected_arguments: Vec<String> = env::args().skip(1).collect(); // Will collect passed arguments and put them into a vector. Does not collect the first passed argument, because it is not needed.
        let possible_options: [&str; 14] = ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"]; // These are all the valid options.
        verify_argument_length(&collected_arguments); // Checks if zero arguments are passed, checks if too many arguments are passed, error in either senario.
        let validated_options: Vec<String> = verify_options_are_valid(&collected_arguments, &possible_options); // Filters and collects all options (--, -) from the arguments. Compares the filtered options to possible_options to verify the given options. Creates errors if bad options are present. Calls on a function to check for exact duplicate options (-h -h), and creates an error if there are duplicate options. Calls on function to check for logically duplicate options (--help -h), and creates error if there are duplicates.
        verify_values_are_valid(&collected_arguments, &validated_options); //
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

    fn verify_options_are_valid (borrow_collected_arguments: &Vec<String>, borrow_possible_options: &[&str; 14]) -> Vec<String> {
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

        if check_for_exact_duplicate_options(&filtered_options) == true { // Function will return true if there are eacxt duplicated options (-p, -p, or --help, --help), which is an error.
            let print_filtered_options: String = filtered_options.join(", ");
            
            println!("Invalid syntax. Duplicated options were passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", &print_filtered_options);
            process::exit(1);
        }

        check_for_logically_duplicate_options(&filtered_options); // If this function finds logically duplicate options (--help -h --path -p), error will be created and process will end.

        return filtered_options; // If there is no issues with the passed options, then filtered_options will be returned to build_running_configuration.
    }

    fn check_for_exact_duplicate_options<T: Eq + std::hash::Hash>(borrow_filtered_options: &[T]) -> bool { // Generics (<T>): Allows the function to operate on slices of any data type. Eq Trait: Ensures that the elements can be compared for equality. Hash Trait: Allows the elements to be hashed, which is necessary for inserting them into a HashSet.
        if borrow_filtered_options.len() != borrow_filtered_options // if borrow_filtered_options.len() != borrow_filtered_options.iter()..collect::<HashSet<_>>().len() {}.
        .iter() // Creates an iterator over references to the elements (&T).
        .collect::<HashSet<_>>()// Transforms the iterator into a collection, in this case, a HashSet. Using ::<HashSet<_>> explicitly tells Rust to collect into a HashSet, and the underscore _ lets the compiler infer the specific type.
        .len() {
            return true; // There are repeated options.
            
        } else {
            return false; // There are not repeated options.
        }
    }

    fn check_for_logically_duplicate_options(borrow_filtered_options: &Vec<String>) {
        let double_tack: Vec<String> = borrow_filtered_options // Creates a vector of all the options that start with --.
        .iter() 
        .filter(|option| option.starts_with("--")) 
        .cloned()
        .collect();

        let mut build_error_message: String = String::new(); // Creates a mutable string, text is appended to it if there is an error.

        for option in double_tack { // ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"].
            if option == "--help" { // If the double tacked option is present, it is an error if the single tacked option is present.
                if borrow_filtered_options.contains(&"-h".to_string()) {
                    build_error_message.push_str("--help -h ");
                }

            } else if option =="--version" {
                if borrow_filtered_options.contains(&"-ver".to_string()) {
                    build_error_message.push_str("--version -ver ");
                }

            } else if option =="--verbose" {
                if borrow_filtered_options.contains(&"-v".to_string()) {
                    build_error_message.push_str("--verbose -v ");
                }

            } else if option =="--query" {
                if borrow_filtered_options.contains(&"-q".to_string()) {
                    build_error_message.push_str("--query -q ");
                }

            } else if option =="--path" {
                if borrow_filtered_options.contains(&"-p".to_string()) {
                    build_error_message.push_str("--path -p ");
                }

            } else if option =="--simple-grep" {
                if borrow_filtered_options.contains(&"-sg".to_string()) {
                    build_error_message.push_str("--simple-grep -sg ");
                }

            } else if option =="--simple-find" {
                if borrow_filtered_options.contains(&"-sf".to_string()) {
                    build_error_message.push_str("--simple-find -sf ");
                }
            }
        }

        if !build_error_message.is_empty() { // If the build_error_message string is not empty, that means there are doubled options, and therefore is an error.
            let error_message = format!("Invalid syntax. Duplicate options were passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", build_error_message);
            println!("{}", error_message);
            process::exit(1);
        }
    }
    // Parses out the non-option arguments. Verifies that if there are zero non-option arguments, then query and path are not present. Creates errors if there is only one value or more than two values. Calls on a function that seperates the query and path, creates an error if the query or path is blank.
    fn verify_values_are_valid(borrow_collected_arguments: &Vec<String>, borrow_validated_options: &Vec<String>) { // Filters and collects everything else besides the options.
        let filtered_values: Vec<String> = borrow_collected_arguments
        .iter()
        .filter(|value| !value.starts_with("--") && !value.starts_with("-"))// Will filter out all other passed arguments that are not options (--, -).
        .cloned()
        .collect();

        let query_present: bool = borrow_validated_options.contains(&"--query".to_string()) || borrow_validated_options.contains(&"-q".to_string()); // query and path_present will == true if they contain query or path options. 
        let path_present: bool = borrow_validated_options.contains(&"--path".to_string()) || borrow_validated_options.contains(&"-p".to_string());

        if filtered_values.len() == 0 && query_present == true || filtered_values.len() == 0 && path_present == true { // Since query and path require values, it is an error if there are xero values and query or path is present.
            if query_present == true && path_present == true { // Different error messages depending on the situation.
                println!("Invalid syntax. The query (--query, -q) and path (--path, -p) options require a non-option value to follow it. Use \"--help\" or \"-h\" to see options and syntax.");
                process::exit(1);
    
            } else if query_present == true {
                println!("Invalid syntax. The query (--query, -q) option requires a non-option value to follow it. Use \"--help\" or \"-h\" to see options and syntax.");
                process::exit(1);
    
            } else if path_present == true {
                println!("Invalid syntax. The path (--path, -p) option requires a non-option value to follow it. Use \"--help\" or \"-h\" to see options and syntax.");
                process::exit(1);
            }
        }

        if filtered_values.len() == 1 || filtered_values.len() > 2 { // There should only be two non-option arguments, one for query, one for path.
            if filtered_values.len() == 1 { // Different error messages depending on the situation.
                let print_bad_arguments: String = filtered_values.join(" ");
                println!("Invalid syntax. Too few non-option values were passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", print_bad_arguments);
                process::exit(1);
    
            } else { // filtered_argument.len() > 2.
                let print_bad_arguments: String = filtered_values.join(", ");
                println!("Invalid syntax. Too many non-option values were passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", print_bad_arguments);
                process::exit(1);
            }
        }

        parse_path_and_query(&borrow_collected_arguments, &filtered_values); // Creates an error if a non-option value is passed as the first argument. Creates errors if a non-option value is passed behind an option that is not path or query. Parses which value is a query and which value is a path.


        /*if filtered_values[0].starts_with("`") || filtered_values[1].starts_with("`") {
            check_for_escape_characters(&filtered_values);
        }*/
    }

    fn parse_path_and_query(borrow_borrow_collected_arguments: &Vec<String>, borrow_filtered_values: &Vec<String>) {
        let mut count: usize = 0;
        let mut query: String = String::new(); // Mutable empth string.
        let mut path: String = String::new();
        let mut error_occurred: usize = 0;

        if borrow_filtered_values.contains(&borrow_borrow_collected_arguments[0]) { // If a non-option value is the first argument passed, it is an error because that has no meaning. An option has to come first.
            println!("Invalid syntax. An option has to be the first argument passed. Use \"--help\" or \"-h\" to see options and syntax.");
            process::exit(1);
        }

        while count < borrow_borrow_collected_arguments.len() { // This loop is structured like this for a reason. Logic errors were occuring when done the other way.
            if borrow_borrow_collected_arguments[count] == borrow_filtered_values[0] { // If the current value of collected_arguments equals a filtered_values. 
                if borrow_borrow_collected_arguments[count -1] == "--query" || borrow_borrow_collected_arguments[count -1] == "-q" { // Look at the option that comes before the current value. If the option that comes before is query or path, you know that the current value is either the query or the path.
                    query = borrow_borrow_collected_arguments[count].clone(); // If the found value meets the parameters, update the string.

                } else if borrow_borrow_collected_arguments[count -1] == "--path" || borrow_borrow_collected_arguments[count -1] == "-p" {
                    path = borrow_borrow_collected_arguments[count].clone();

                } else {
                    error_occurred = error_occurred + 1; // If query or path cannot be assigned, increment the error counter.
                }
            }
            
            if borrow_borrow_collected_arguments[count] == borrow_filtered_values [1] {
                if borrow_borrow_collected_arguments[count -1] == "--query" || borrow_borrow_collected_arguments[count -1] == "-q" {
                    query = borrow_borrow_collected_arguments[count].clone();

                } else if borrow_borrow_collected_arguments[count -1] == "--path" || borrow_borrow_collected_arguments[count -1] == "-p" {
                    path = borrow_borrow_collected_arguments[count].clone();

                } else {
                    error_occurred = error_occurred + 1;
                }
            }

            count += 1;
        }

        if error_occurred != 0 {
            if query.is_empty() && path.is_empty() {
                println!("Invalid syntax. The query option (--query, -q) and path option (--path, -p) are not followed by a non-option value. Use \"--help\" or \"-h\" to see options and syntax.");
                process::exit(1);

            } else if query.is_empty() {
                println!("Invalid syntax. The query option (--query, -q) is not followed by a non-option value. Use \"--help\" or \"-h\" to see options and syntax.");
                process::exit(1);

            } else { // path.is_empty.
                println!("Invalid syntax. The path option (--path, -p) is not followed by a non-option value. Use \"--help\" or \"-h\" to see options and syntax.");
                process::exit(1);
            }
        }

        validate_path(&path);
    }

    fn validate_path(borrow_path: &String) {
        match fs::metadata(path) {
        
        } 
        
        
        
        /*let check_path: bool = Path::new(borrow_path).exists(); // Checks if the path is valid and returns a boolean value.
        
        if check_path == false { // TODO: Check and see what happens when you give a file path that the current user does not have access to.
            println!("Error Path");
        }*/
    }

    /*fn check_for_escape_characters(borrow_filtered_values: &Vec<String>) {

    }*/
}