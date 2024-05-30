use std::collections::HashMap;
use std::io;

fn main() {
    let mut department_hash : HashMap<String, Vec<String>> = HashMap::new();
    print_intro();

    'main: loop {
        let mut user_input = String::new();
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
                    name = name.trim_end().to_string();  
                    if let Some(department) = words.next() {
                        department_hash.entry(department.to_string()).or_insert(Vec::new()).push(name.clone());
                        println!("Successfully added {name} to {department}");
                    } else {
                        println!("Must specify department name!");
                    }
                    break;
                }
            }
        }
        else if action.to_lowercase() == "remove" {
            println!("Remove who?");
        } 
        else if action.to_lowercase() == "list" {
            if let Some(department) = words.next() {
                if let Some(department_names) = department_hash.get(department) {
                    let mut sorted_names = department_names.clone();
                    sorted_names.sort_unstable();
                    println!("{:?}", sorted_names);
                } else {
                    println!("Department name not found!");
                }
            } else {
                println!("Must specify department name!");
            }
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
