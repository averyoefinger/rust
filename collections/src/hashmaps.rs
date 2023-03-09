/*
Using a hash map and vectors, create a text interface to allow a user to add 
employee names to a department in a company. For example, “Add Sally to 
Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all 
people in a department or all people in the company by department, sorted 
alphabetically.
*/
use std::io;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
enum Request {
    Add,
    Remove,
    List,
}

fn request_from_string(s: &str) -> Option<Request> {
    match s.to_lowercase().as_str() {
        "add" =>    Some(Request::Add),
        "remove" => Some(Request::Remove),
        "list" =>   Some(Request::List),
        _ => None,
    }
}

#[derive(Debug)]
struct Input {
    request: Request,
    name: String,
    department: String,
}

pub fn menu() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Please enter '<{:?}/{:?}> <name> to <department>' or\n'<{:?}> <department>' for all names or\n'q' to quit:", Request::Add, Request::Remove, Request::List);
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => continue,
        };
        input = input.trim().to_string();
        if input.eq("q") {
            break;
        }
        let formatted_input: Input;
        match format(&input) {
            Ok(n) => formatted_input = n,
            Err(_) => continue,
        }
        match formatted_input.request {
            Request::Add => {
                company.entry(formatted_input.department.clone()).or_insert_with(|| vec![]).push(formatted_input.name.clone());
                println!("Added {} to {}", formatted_input.name, formatted_input.department);
            },
            Request::Remove => {
                let list = company.entry(formatted_input.department.clone()).or_insert_with(|| vec![]);
                let position = list.iter().position(|x| x == &formatted_input.name);
                match position {
                    Some(p) => {
                        list.remove(p);
                    },
                    None => {
                        println!("There is no record of a {} in the {} department.", formatted_input.name, formatted_input.department);
                        continue;
                    },
                }
                println!("Removed {} from {}", formatted_input.name, formatted_input.department);
            },
            Request::List => {
                match company.get(&formatted_input.department) {
                    Some(names) => {
                        let mut names = names.clone();
                        names.sort();
                        for name in names {
                            println!("{}", name);
                        }
                    },
                    None => {
                        println!("There are no names in this department: {}", formatted_input.department)
                    },
                }
            },
        }
    }
}

fn format(s: &str) -> Result<Input, String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    let request = request_from_string(words.get(0).unwrap_or(&"")).ok_or_else(|| "Bad request")?;
    let formatted_input = Input {
        request,
        name: match request {
            Request::Add | Request::Remove => words.get(1).unwrap_or(&"").to_string(),
            Request::List => "".to_string(),
        },
        department: match request {
            Request::Add | Request::Remove => words.get(3).unwrap_or(&"").to_string(),
            Request::List => words.get(1).unwrap_or(&"").to_string(),
        },
    };
    return Ok(formatted_input);
}
