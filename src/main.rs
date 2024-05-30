use std::collections::HashMap;
use std::io;

fn main() {
    let mut department_hash : HashMap<String, Vec<String>> = HashMap::new();
    print_intro();

    let mut user_input = String::new();
   
    'main: loop {
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let trimmed_input = user_input.trim();

    let mut words = trimmed_input.split_whitespace();
    let action = words.next().unwrap();
    
    if action.to_lowercase() == "add" {
        let mut name = String::new();
        while let Some(name_part) = words.next() {
            if name_part != "to" {
                name += name_part;
                name += " ";
            } else {
                if let Some(department) = words.next() {
                    department_hash.entry(department.to_string()).or_insert(Vec::new()).push(name);
                } else {
                    println!("Must specify department name!");
                }
                break;
            }
        }
        println!("{:?}", department_hash);
    }
    else if action.to_lowercase() == "remove" {
        println!("Remove who?");
    } 
    else if action.to_lowercase() == "quit" || action.to_lowercase() == "q" {
        break 'main;
    } else {
        println!("Action Not Recognized!");
    }
    }
}

fn print_intro() {
    println!("Welcome to E-Corp!");
    println!("To Enter a new Employee into the system, follow the pattern: ");
    println!("Add |Employee Name| to |Department Name|");
}
