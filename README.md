# lozgrep   
#### Simple grep and find command line tool written in Rust.   
- The complied executable is in the main branch.   
- Have not tested it on Windows, only Linux.
## Options:     
``--help          -h``       Prints the help menu.      
``--version       -ver``     Prints the current version.      
``--verbose       -v``       Prints output statements while the process is running.        
``--query         -q``       The term you are searching for follows this option.       
``--path          -p``       The path you are searching follows this option.        
``--simple-grep   -sg``      Searches the contents of a file.       
``--simple-find   -sf``      Searches for a file or directory name.          
## Syntax rules:        
There is no default mode, the options have to be sprecified.     
The options can come in any order.     
The long option (--) or short option (-) can be used interchangeably.     
## Examples:
lozgrep -sg -p /home/user/file -q wordiamlookingfor    
lozgrep --help -ver --query filename --simple-find -p /root       
## Escape character rules:    
The escape character is: /        
The escape character can only be used on the value you want to query.          
#### Escape character examples:           
If you need to query for phrase that happens to also be an option        
lozgrep -sg -q /--help -p /home/user/file       
The escape character is needed in this example because without it lozgrep will read --help as an option and not an item to look for.         
Therefore if you need to query for / you need to escape it, otherwise it will be stripped and the query will be empty.          
lozgrep -sg -q // -p /home/user/file     
## Contact: contact@gistyr.dev    
