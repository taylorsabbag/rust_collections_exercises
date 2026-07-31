use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter, Display, EnumString)]
pub enum Department {
    #[strum(ascii_case_insensitive)]
    Engineering,

    #[strum(serialize = "Human Resources", ascii_case_insensitive)]
    HR,

    #[strum(ascii_case_insensitive)]
    Sales,
}

pub struct Directory {
    users: HashMap<Department, Vec<String>>,
}

impl Default for Directory {
    fn default() -> Self {
        let mut users = HashMap::new();
        for dept in Department::iter() {
            users.insert(dept, Vec::new());
        }
        Directory { users }
    }
}

impl Directory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, user: String, department: Department) {
        println!("Successfully added {user} to {department}.");
        if let Some(names) = self.users.get_mut(&department) {
            names.push(user);
        }
    }

    pub fn retrieve(&self, target_dept: Option<Department>) {
        if let Some(dept) = target_dept {
            match self.users.get(&dept) {
                Some(names) => self.print_dept(&dept, names),
                None => println!("{dept} does not exist in the directory."),
            }
        } else {
            let total_users: usize = self.users.values().map(|v| v.len()).sum();
            if total_users == 0 {
                println!("The directory is currently entirely empty.");
                return;
            }

            for dept in Department::iter() {
                if let Some(names) = self.users.get(&dept) {
                    self.print_dept(&dept, names);
                }
            }
        }
    }

    fn print_dept(&self, dept: &Department, names: &[String]) {
        println!("{dept}:");
        if names.is_empty() {
            println!("  (empty)");
        } else {
            let mut sorted_names = names.to_vec();
            sorted_names.sort();
            for name in &sorted_names {
                println!(" - {name}");
            }
        }
    }
}

pub enum Command {
    AddUser(String, Department),
    RetrieveUsers(Option<Department>),
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        match tokens.as_slice() {
            ["add", name, "to", dept_str @ ..] => {
                if dept_str.is_empty() {
                    return Err(Error::ParseError);
                }
                let dept_combined = dept_str.join(" ");
                let department = dept_combined
                    .parse()
                    .map_err(|_| Error::InvalidDepartment)?;
                Ok(Command::AddUser(name.to_string(), department))
            }
            ["add", ..] => Err(Error::ParseError),
            ["retrieve"] => Ok(Command::RetrieveUsers(None)),
            ["retrieve", dept_str @ ..] => {
                let dept_combined = dept_str.join(" ");
                let department = dept_combined
                    .parse()
                    .map_err(|_| Error::InvalidDepartment)?;
                Ok(Command::RetrieveUsers(Some(department)))
            }
            _ => Err(Error::InvalidCommand),
        }
    }
}

#[derive(Debug, Display)]
pub enum Error {
    #[strum(serialize = "Command format must be 'add [name] to [dept]'")]
    ParseError,

    #[strum(serialize = "Unknown command. Try 'add', 'retrieve', or 'exit'")]
    InvalidCommand,

    #[strum(serialize = "Department must be Engineering, Sales, or Human Resources.")]
    InvalidDepartment,
}

pub fn run() {
    let mut directory = Directory::new();
    println!("--- Company Directory Interface ---");
    println!("Commands: 'add [name] to [dept]', 'retrieve', 'exit'");

    loop {
        print!("\nInput command: ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error: Unable to process input.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        match trimmed.parse::<Command>() {
            Ok(Command::AddUser(user, department)) => directory.add(user, department),
            Ok(Command::RetrieveUsers(dept)) => directory.retrieve(dept),
            Err(e) => println!("Error: {e}"),
        }
    }
}
