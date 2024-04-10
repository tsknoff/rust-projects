use std::io;

// mod ex_1;
// mod ex_2;
// mod ex_3;

fn main() {
    // Ex. 1 ================================================
    // let mut vector = vec![1, 2, 10, 3, 4, 1, 1];
    // vector.sort();
    // println!("{:?}", vector);
    //
    // println!("Average is: {:?}", ex_1::average(&vector));
    // println!("Median is : {}", ex_1::median(&vector));
    // println!("Mode is   : {}", ex_1::mode(&vector));
    // println!();
    // Ex. 1 ================================================

    // Ex. 2 ================================================
    // let s1 = String::from("first");
    // let s2 = String::from("apple");
    // println!("Mutated string: {}", ex_2::to_pig_latin(&s1));
    // println!("Mutated string: {}", ex_2::to_pig_latin(&s2));
    // Ex. 2 ================================================


    // Using a hash map and vectors,
    // create a text interface to allow a user to add employee names to a department in a company.
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department
    // or all people in the company by department, sorted alphabetically.

    // Ex. 3 ================================================

    use std::collections::HashMap;

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please input your command.");
        println!("1. Add employee to department");
        println!("2. List all employees in a department");
        println!("3. List all employees in the company");
        println!("4. Exit");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command: u32 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match command {
            1 => {
                println!("Enter department name: ");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                let department: String = match department.trim().to_string().parse() {
                    Ok(string) => string,
                    Err(_) => continue,
                };

                println!("Enter employee name: ");
                let mut employee_name = String::new();
                io::stdin()
                    .read_line(&mut employee_name)
                    .expect("Failed to read line");
                let employee_name: String = match employee_name.trim().to_string().parse() {
                    Ok(string) => string,
                    Err(_) => continue,
                };

                println!("{employee_name}");

                let employees_ref = company.entry(department).or_insert(vec![]);
                if !employees_ref.contains(&employee_name) {
                    employees_ref.push(employee_name);
                    employees_ref.sort();
                    println!("Employee added to the department");
                } else {
                    println!("Employee already exists in the department");
                }
            }
            2 => {
                println!("Enter department name: ");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                let department: String = match department.trim().to_string().parse() {
                    Ok(string) => string,
                    Err(_) => continue,
                };

                match company.get(&department) {
                    Some(employees) => {
                        for employee in employees {
                            println!("{employee}");
                        }
                    }
                    None => println!("Department does not exist"),
                }
            }
            3 => {
                let mut departments: Vec<String> = company.keys().cloned().collect();
                departments.sort();
                for department in departments {
                    println!("{department}");
                    match company.get(&department) {
                        Some(employees) => {
                            for employee in employees {
                                println!("{employee}");
                            }
                        }
                        None => println!("Department does not exist"),
                    }
                }
            }
            4 => break,
            _ => continue,
        }
    }
}
