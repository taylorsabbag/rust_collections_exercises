use std::collections::HashMap;
use std::io::{self, Write};

type Users = HashMap<Department, Vec<String>>;

pub fn run() {
    let mut users: Users = HashMap::new();

    println!("--- Company Directory Interface ---");
    println!("Commands: 'add [name] to [dept]', 'retrieve', 'exit'");

    loop {
        print!("\nInput command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to process input");

        let trimmed = input.trim();
        if trimmed.to_ascii_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        match eval(trimmed) {
            Ok(Command::AddUser(user, department)) => {
                add_user_to_department(user, department, &mut users);
            }
            Ok(Command::RetrieveUsers(department)) => retrieve_users(&users, department),
            Err(e) => match e {
                Error::ParseError => {
                    println!("Error: Command format must be 'add [name] to [dept]'")
                }
                Error::InvalidCommandError => {
                    println!("Error: Unknown command. Try 'add', 'retrieve', or 'exit'")
                }
                Error::InvalidDepartmentError => {
                    println!("Error: Department must be Engineering, Sales, or HR.")
                }
            },
        }
    }
}

enum Command {
    AddUser(String, Department),
    RetrieveUsers(Option<Department>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Department {
    Engineering,
    HR,
    Sales,
}

enum Error {
    ParseError,
    InvalidCommandError,
    InvalidDepartmentError,
}

fn eval(input: &str) -> Result<Command, Error> {
    let input_as_vec: Vec<&str> = input.split_whitespace().collect();

    match input_as_vec.get(0) {
        Some(&"add") => {
            if input_as_vec.len() != 4 || input_as_vec[2] != "to" {
                return Err(Error::ParseError);
            }
            let user = input_as_vec[1].to_string();
            let department = match input_as_vec[3].to_ascii_lowercase().as_str() {
                "engineering" => Department::Engineering,
                "sales" => Department::Sales,
                "hr" => Department::HR,
                _ => return Err(Error::InvalidDepartmentError),
            };
            Ok(Command::AddUser(user, department))
        }
        Some(&"retrieve") => {
            if input_as_vec.len() == 1 {
                Ok(Command::RetrieveUsers(None))
            } else if input_as_vec.len() == 2 {
                let department = match input_as_vec[1].to_ascii_lowercase().as_str() {
                    "engineering" => Department::Engineering,
                    "sales" => Department::Sales,
                    "hr" => Department::HR,
                    _ => return Err(Error::InvalidDepartmentError),
                };
                Ok(Command::RetrieveUsers(Some(department)))
            } else {
                Err(Error::ParseError)
            }
        }

        _ => Err(Error::InvalidCommandError),
    }
}

fn add_user_to_department(user: String, department: Department, users: &mut Users) {
    users
        .entry(department.clone())
        .or_default()
        .push(user.clone());
    println!("Successfully added {user} to {department:?}.");
}

fn retrieve_users(users: &Users, target_dept: Option<Department>) {
    let print_dept_list = |dept: &Department, names: &Vec<String>| {
        println!("{dept:?}:");
        let mut sorted_names = names.clone();
        sorted_names.sort();
        sorted_names.iter().for_each(|name| println!("  - {name}"));
    };

    match target_dept {
        Some(dept) => match users.get(&dept) {
            Some(names) => print_dept_list(&dept, names),
            None => println!("{dept:?} does not have any employees yet."),
        },
        None => {
            if users.is_empty() {
                println!("The directory is currently empty.");
                return;
            }
            users
                .iter()
                .for_each(|(dept, names)| print_dept_list(dept, names));
        }
    }
}
