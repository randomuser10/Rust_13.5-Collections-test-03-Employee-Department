//use standard libraries

use::std::collections::HashMap;
use::std::io;



fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\n Enter a command (example: Add Sally to Engineering, List Engineering, List All, or Quit):");


        //read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read message");
        let input = input.trim();// remove extra spaces

        //quit condition
        if input.eq_ignore_ascii_case("quit"){
            break;
        }

        //check if the user wants to add an Employee
        if input.to_lowercase().starts_with("add") {
            //Expected format : "Add Name to Department"
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.len() >= 4 &&  parts[2].eq_ignore_ascii_case("to") {
                let name = parts[1].to_string();
                let department = parts[3..].join(" ");

                company.entry(department.clone()).or_insert(Vec::new()).push(name);

                println!("Added employee to department: {} -> {:?}" ,department,company[&department]);
            } else {
                println!("Invalid format! use: Add NAME to DEPARTMENT");
            }

        } //check if user wants to list employees
            else if input.to_lowercase().starts_with("list") {
                let parts: Vec<&str> = input.split_whitespace().collect();

                if parts.len() == 2 && parts[1].eq_ignore_ascii_case("all"){
                    for (dept, employees) in &company {
                        let mut sorted_employees = employees.clone();
                        sorted_employees.sort();
                        println!("{}: {:?}", dept, sorted_employees);
                    }
                } else {
            let department = parts[1..].join(" ");
            match company.get(&department){
                Some(employees) =>{
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    println!("{}: {:?}", department, sorted_employees);
                    },
                None => println!("No such department"),
                }
            }
        } else {
            println!("No such command exist");
        }
    }

}
