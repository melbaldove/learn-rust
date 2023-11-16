use regex::Regex;
use std::{collections::HashMap, io};

fn main() {
    let mut command = String::new();
    let mut employees_map = HashMap::new();
    let add_re = Regex::new(r"Add (.*) to (.*)").unwrap();
    let fetch_all_by_dep_re = Regex::new(r"Get (.*)").unwrap();
    let fetch_all_re = Regex::new(r"Get all").unwrap();
    loop {
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        if command.trim().eq("exit") {
            return;
        }
        execute_command(
            &command[..],
            &mut employees_map,
            &add_re,
            &fetch_all_by_dep_re,
            &fetch_all_re,
        );
        command.clear();
    }
}

fn execute_command(
    command: &str,
    employees_map: &mut HashMap<String, Vec<String>>,
    add_re: &Regex,
    fetch_all_by_dep_re: &Regex,
    fetch_all: &Regex,
) {
    let command: Command = match command {
        command if add_re.is_match(command) => {
            let caps = add_re.captures(command).unwrap();
            Command::Add(String::from(&caps[1]), String::from(&caps[2]))
        }
        command if fetch_all.is_match(command) => Command::GetAll(),
        command if fetch_all_by_dep_re.is_match(command) => {
            let caps = fetch_all_by_dep_re.captures(command).unwrap();
            Command::GetAllByDepartment(String::from(&caps[1]))
        }
        _ => Command::Invalid,
    };
    match command {
        Command::Add(name, department) => {
            println!("{} is from {}", name, department);
            let employee_list = employees_map.entry(department).or_insert(Vec::new());
            (*employee_list).push(name);
        }
        Command::GetAllByDepartment(department) => {
            println!("Employees of {}:", department);
            let employee_list = employees_map.entry(department).or_insert(Vec::new());
            employee_list.sort();
            for employee in employee_list {
                println!("{}", employee);
            }
        }
        Command::GetAll() => {
            println!("All employees:");
            let mut all_employees: Vec<(&String, &String)> = Vec::new();
            for (department, department_employees) in employees_map {
                for employee in department_employees {
                    all_employees.push((employee, department));
                }
            }
            all_employees.sort_unstable_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(b.1)));
            let mut all_employees_print = String::new();
            for (employee, department) in all_employees {
                all_employees_print.push_str(employee);
                all_employees_print.push_str(", ");
                all_employees_print.push_str(department);
                all_employees_print.push_str("\n");
            }
            print!("{}", all_employees_print);
        }
        Command::Invalid => println!("Invalid command"),
    }
}

enum Command {
    Add(String, String),
    GetAllByDepartment(String),
    GetAll(),
    Invalid,
}
