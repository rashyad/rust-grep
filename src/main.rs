use std::{fs, env};
use regex::Regex;
use colored::Colorize;
use std::path::Path;


// Maybe to make this program even better if we can add move and copy functionality
// i guess that's what i will do next time i stream. See you!

fn main() {

    // ideally, we should make both path and grep optional values
    // if user does not supply any args, we can just return the current dir
    // let's first read the grep arg and path

    let args: Vec<String> = env::args().collect();

    // we should only loop if args.len() > 1
    let mut grep_value = "";
    let mut path_value = "";
    if args.len() > 1 {
        let mut i = 0;

        while i < args.len() {
            

            if &args[i] == "grep" {

                grep_value = &args[i + 1];

            } else if &args[i] == "path" {

                path_value= &args[i + 1];

            }
            i += 1;
        }								
    }

    let current_dir = env::current_dir()
    .unwrap_or_else(|err| {
        panic!("Errir reading directory: {:?}", err)
    });

    let dir_to_read = if path_value.is_empty() { current_dir } else  { Path::new(path_value).to_path_buf() };

    let directory = fs::read_dir(dir_to_read)
    .unwrap_or_else(|err| {
        panic!("Errir reading directory: {:?}", err)
    });

    
    let re = if grep_value.is_empty() { 
        Regex::new(r"cargo").unwrap() 
    } else  { 
        let x = format!(r"{}", grep_value);
        // guess that works?
        Regex::new(&x).unwrap() 
    };

    for entry in directory {
        if let Ok(entry) = entry {

            // let's clean up
            let entry_path = entry.path();

            let file_name = entry.file_name().into_string()
            .unwrap_or_else(|err| {
                panic!("Errir reading directory: {:?}", err)
            });

            // we shouldnt do any regex pattern test if grep is not passed in
            
            if grep_value.is_empty() {

                if entry_path.is_dir() {

                    println!("{}", String::from(&file_name).green());
                } else {
                    println!("{}", String::from(&file_name).blue());
                } 

            } else {
                
                let name_to_test = &file_name.to_lowercase();

                let test_regex_name =  re.captures(name_to_test);
    
                if test_regex_name.is_some() {
                    if entry_path.is_dir() { 
                        println!("{}", String::from(&file_name).green());
                    }else {
                        println!("{}", String::from(&file_name).blue());
                    }
                }
            }

        }
    }

}