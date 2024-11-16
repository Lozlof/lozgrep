pub mod parse_and_build_arguments {
    use std::env;
    use std::process;
    use std::collections::HashSet;
    use std::fs;
    use std::io;
    use std::path::Path;

    pub fn build_running_configuration() -> Options {
        let collected_arguments: Vec<String> = env::args().skip(1).collect(); // Will collect passed arguments and put them into a vector. Does not collect the first passed argument, because it is not needed.
        let possible_options: [&str; 14] = ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"]; // These are all the valid options.

        verify_argument_length(&collected_arguments); // Checks if zero arguments are passed, checks if too many arguments are passed, error in either senario.

        let validated_options: Vec<String> = verify_options_are_valid(&collected_arguments, &possible_options); // Filters and collects all options (--, -) from the arguments. Compares the filtered options to possible_options to verify the given options. Creates errors if bad options are present. Calls on a function to check for exact duplicate options (-h -h), and creates an error if there are duplicate options. Calls on function to check for logically duplicate options (--help -h), and creates error if there are duplicates.
        let validated_values = verify_values_are_valid(&collected_arguments, &validated_options); // Parses out the non-option arguments. Verifies that if there are zero non-option arguments, then query and path are not present. Creates errors if there is only one value or more than two values. 
        
        if validated_values.len() == 0 { // If validated_vales.len() == 0 and validated_values gets passed into parse_path_and_query, it will cause an error. 
            let null_query: String = "null".to_string(); // Needed because check_if_the_given_options_work_together requires two Strings to be passed to it.
            let null_path: String = "null".to_string();

            check_if_the_given_options_work_together(&validated_options, &null_query, &null_path); // Will ignore null strings.

            let running_options: Options = Options::build_options(validated_options, null_query, null_path);

            return running_options;

        } else { // validated_values.len() != 0
            let (valid_query, valid_path) = parse_path_and_query(&collected_arguments, &validated_values); // Creates an error if a non-option value is passed as the first argument. Creates errors if a non-option value is passed behind an option that is not path or query. Parses which value is a query and which value is a path. Validates path. Checks for escape character on the query.
        
            check_if_the_given_options_work_together(&validated_options, &valid_query, &valid_path); // Checks if the passed options work together. May have to add more logic here, if errors appear.
            
            let running_options: Options = Options::build_options(validated_options, valid_query, valid_path);

            return running_options;
        }
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

    fn verify_values_are_valid(borrow_collected_arguments: &Vec<String>, borrow_validated_options: &Vec<String>) -> Vec<String>{ // Filters and collects everything else besides the options.
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

        return filtered_values;
    }

    fn parse_path_and_query(borrow_collected_arguments: &Vec<String>, borrow_validated_values: &Vec<String>) -> (String, String) { // Returns a tuple of query and path.
        let mut count: usize = 0;
        let mut query: String = String::new(); // Mutable empty string.
        let mut path: String = String::new();
        let mut error_occurred: usize = 0;

        if borrow_validated_values.contains(&borrow_collected_arguments[0]) { // If a non-option value is the first argument passed, it is an error because that has no meaning. An option has to come first.
            println!("Invalid syntax. An option has to be the first argument passed. Use \"--help\" or \"-h\" to see options and syntax.");
            process::exit(1);
        }

        while count < borrow_collected_arguments.len() { // This loop is structured like this for a reason. Logic errors were occuring when done the other way.
            if borrow_collected_arguments[count] == borrow_validated_values[0] { // If the current value of collected_arguments equals a filtered_values. 
                if borrow_collected_arguments[count -1] == "--query" || borrow_collected_arguments[count -1] == "-q" { // Look at the option that comes before the current value. If the option that comes before is query or path, you know that the current value is either the query or the path.
                    query = borrow_collected_arguments[count].clone(); // If the found value meets the parameters, update the string.

                } else if borrow_collected_arguments[count -1] == "--path" || borrow_collected_arguments[count -1] == "-p" {
                    path = borrow_collected_arguments[count].clone();

                } else {
                    error_occurred = error_occurred + 1; // If query or path cannot be assigned, increment the error counter.
                }
            }
            
            if borrow_collected_arguments[count] == borrow_validated_values [1] {
                if borrow_collected_arguments[count -1] == "--query" || borrow_collected_arguments[count -1] == "-q" {
                    query = borrow_collected_arguments[count].clone();

                } else if borrow_collected_arguments[count -1] == "--path" || borrow_collected_arguments[count -1] == "-p" {
                    path = borrow_collected_arguments[count].clone();

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

        if validate_path(&path) == false { // If validate_path returns false, process is exited.
            process::exit(1);
        }

        if query.starts_with("/") { // The escape character for queries is /, therefore if query starts with /, it must be removed. And if the user wants to search for / they have to type //.
            query = query.chars().skip(1).collect::<String>(); // Since query is mutable, just re-define it with the first char trimmed.
        }

        return (query, path);
    }

    fn validate_path(borrow_path: &String) -> bool { // Checks if the given path is valid, if not valid creates errors.
        match fs::metadata(borrow_path) { // Attempts to retrieve metadata about the file or directory. match Statement: Matches the result of fs::metadata(borrow_path) to handle both success and error cases.
            Ok(_) => { return true; } // Was able to retrieve metadata, therfore the path is valid, so true is returned. 
            Err(error) => match error.kind() { // Nested match error.kind(): Matches on the specific kind of I/O error to determine why the metadata retrieval failed.
                io::ErrorKind::NotFound => { // Path does not exist.
                    println!("Path error. The path given is not valid because the path cannot be found.");
                    return false;

                } io::ErrorKind::PermissionDenied => { // Current user doesn't have proper permissions.
                    println!("Path error. The path given is not valid because permission was denied.");
                    return false;

                } _ => { // Wildcard to catch everything else.
                    println!("Path error. The lozgrep cannot access the specified path"); 
                    return false;
                }
            }
        } 
    }

    fn check_if_the_given_options_work_together(borrow_validated_options: &Vec<String>, borrow_valid_query: &String, borrow_valid_path: &String) { // ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"] all the options for reference.
        if (borrow_validated_options.contains(&"--simple-grep".to_string()) || borrow_validated_options.contains(&"-sg".to_string())) && (borrow_validated_options.contains(&"--simple-find".to_string()) || borrow_validated_options.contains(&"-sf".to_string())) { // If simple-grep and simple-find are both passed. It is an error because thoes two options do not work together.
            println!("Invalid syntax. The simple-grep (--simple-grep, -sg) and simple-find (--simple-find, -sf) options cannot be used together. Those processes can only be ran one at a time. Use \"--help\" or \"-h\" to see options and syntax.");
            process::exit(1);
        }

        if (borrow_valid_query == "null" && borrow_valid_path == "null") && ((borrow_validated_options.contains(&"--simple-grep".to_string()) || borrow_validated_options.contains(&"-sg".to_string())) || (borrow_validated_options.contains(&"--simple-find".to_string()) || borrow_validated_options.contains(&"-sf".to_string()))) { // If the user does not pass a query and path, the simple-grep and simple-find processes cannot run.
            println!("Invalid syntax. The simple-grep (--simple-grep, -sg) and simple-find (--simple-find, -sf) options cannot be used if a query (--query, -q) and path (--path, -p) are not passed. Use \"--help\" or \"-h\" to see options and syntax.");
            process::exit(1);
        }    

        if borrow_validated_options.contains(&"--simple-grep".to_string()) || borrow_validated_options.contains(&"-sg".to_string()) { // If simple-grep is passed, the path must be a file.
            let check_path: &Path = Path::new(borrow_valid_path);

            if check_path.is_dir() { // If path is a directory, error.
                println!("Invalid syntax. When using simple-grep (--simple-grep, -sg) the path specified needs to be a file. simple-grep searches the contents of files. Use \"--help\" or \"-h\" to see options and syntax.");
                process::exit(1);
            }
        } 

        if borrow_validated_options.contains(&"--simple-find".to_string()) || borrow_validated_options.contains(&"-sf".to_string()) {
            let check_path: &Path = Path::new(borrow_valid_path);

            if check_path.is_file() {
                println!("Invalid syntax. When using simple-find (--simple-find, -sf) the path specified needs to be a directory. simple-find searches a directory for a file. Use \"--help\" or \"-h\" to see options and syntax."); 
                process::exit(1);
            } 
        }
    }

    #[derive(Debug)] // Instructs the compiler to automatically generate an implementation of the Debug trait for your struct. Has to do this in order for this line in main to work:  if passed_options.verbose == true { println!("Collected {:?}", &passed_options) }.
    pub struct Options {
        pub help: bool,
        pub version: bool,
        pub verbose: bool,
        pub query: bool,
        pub path: bool,
        pub simple_grep: bool,
        pub simple_find: bool,
        pub query_item: String,
        pub path_item: String,
    }

    impl Options { // ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"] all the options for reference.
        fn build_options(build_options: Vec<String>, build_query: String, build_path: String) -> Options { // Assign everything.
            let help: bool = if build_options.contains(&"--help".to_string()) || build_options.contains(&"-h".to_string()) { true } else { false };
            let version: bool = if build_options.contains(&"--version".to_string()) || build_options.contains(&"-ver".to_string()) { true } else { false };
            let verbose: bool = if build_options.contains(&"--verbose".to_string()) || build_options.contains(&"-v".to_string()) { true } else { false };
            let query: bool = if build_options.contains(&"--query".to_string()) || build_options.contains(&"-q".to_string()) { true } else { false };
            let path: bool = if build_options.contains(&"--path".to_string()) || build_options.contains(&"-p".to_string()) { true } else { false };
            let simple_grep: bool = if build_options.contains(&"--simple-grep".to_string()) || build_options.contains(&"-sg".to_string()) { true } else { false };
            let simple_find: bool = if build_options.contains(&"--simple-find".to_string()) || build_options.contains(&"-sf".to_string()) { true } else { false };
            let query_item: String = build_query;
            let path_item:String = build_path;

            return Options {help, version, verbose, query, path, simple_grep, simple_find, query_item, path_item}
        }
    }
}

pub mod print_to_terminal { // All print to terminal functions go here.
    pub fn print_version(borrow_passed_options_verbose: &bool) {
        println!("lozgrep version 0.0.2");
        
        if borrow_passed_options_verbose == &true { println!("VERBOSE: Printed version"); } 
    }

    pub fn print_help(borrow_passed_options_verbose: &bool) { // TODO: Make more descriptive.
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

        if borrow_passed_options_verbose == &true { println!("VERBOSE: Printed help menu"); }
    }
}

pub mod execute_main_operations {
    use std::io;
    use std::fs;
    use std::process;

    pub fn simple_grep(borrow_query_item: &String, borrow_path_item: &String, borrow_passed_options_verbose: &bool) {
        let contents_result: Result<String, io::Error> = fs::read_to_string(borrow_path_item); // fs::read_to_string takes the file_path, opens that file, and returns a value of type std::io::Result<String> that contains the file’s contents.
        if borrow_passed_options_verbose == &true { println!("VERBOSE: Attempt to read the contents of {} into a string", borrow_path_item); }

        let file_contents:String = match contents_result { // Begins a match expression to handle the two possible variants of the contents_result (Ok or Err). Declares contents as a String to store the file contents if reading is successful.
            Ok(file) => file, // If no error, the contents of the file are passed into file_contents.
            Err(error_one) => { // If error, print error message and exit.
            println!("Error. Problem reading the file contents of the given path: {}", error_one);
            process::exit(1); 
            }
        };
        if borrow_passed_options_verbose == &true { println!("VERBOSE: Successfully read the contents of {} into a string", borrow_path_item); }

        let mut results_that_match_query: Vec<&str> = Vec::new(); // Mutable vector of &str to hold the found lines.

        if borrow_passed_options_verbose == &true { println!("VERBOSE: Attempt to check if {} contains {}", borrow_path_item, borrow_query_item); }
        for line in file_contents.lines() { // For every line in file_contents.
            if line.contains(borrow_query_item) {  // If query_item is in the line.
                results_that_match_query.push(line); // Push that line into the vector.
                if borrow_passed_options_verbose == &true { println!("VERBOSE: Found a match"); }
            }
        }

        if results_that_match_query.len() == 0 { // If results_that_match_query is empty. Print message and exit.
            println!("No matches found.");

            if borrow_passed_options_verbose == &true { println!("VERBOSE: End of process, now exiting"); }
            process::exit(1);

        } else { // If results_that_match_query is not empty. Print the contents and exit.
            if borrow_passed_options_verbose == &true { println!("VERBOSE: Will now print the matches"); }
            for item in results_that_match_query { println!("{}", item); }

            if borrow_passed_options_verbose == &true { println!("VERBOSE: End of process, now exiting"); }
            process::exit(1);
        }
    }
}