use std::{collections::HashMap, io};

pub struct Database {
    db: HashMap<String, Vec<String>>,
}

impl Database {
    pub fn new() -> Database {
        Database { db: HashMap::new() }
    }

    pub fn run(&mut self) {
        loop {
            match read_command() {
                1 => self.add_employee(),
                2 => self.list_employees(),
                3 => break,
                _ => println!("Invalid command. Please try again."),
            }
        }
    }

    fn add_employee(&mut self) {
        let dept = read_department();
        let name = read_name();
        self.db
            .entry(String::from(&dept))
            .or_default()
            .push(name.clone());
        println!("{} has been added to the {} department.", name, dept);
    }

    fn list_employees(&mut self) {
        for department in self.db.keys() {
            println!("{} department:", department);
            let mut employees = self.db.get(department).unwrap().clone();
            employees.sort();
            for employee in employees {
                println!("\t{}", employee);
            }
        }
    }
}

fn prompt() {
    println!("Please select an option below:");
    println!("1. Add an employee");
    println!("2. List all employees");
    println!("3. Exit");
}

fn read_command() -> u32 {
    loop {
        prompt();
        let command = read_input();
        match command.trim().parse() {
            Ok(x) => return x,
            Err(_) => {
                println!("{} is not a number. You idiot!", command.trim());
                continue;
            }
        };
    }
}

fn read_department() -> String {
    println!("Please enter a department:");

    let dept = read_input();

    dept.trim().to_lowercase()
}

fn read_name() -> String {
    println!("Please enter the employee name:");

    let name = read_input();

    name.trim().to_lowercase()
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
