use std::io;

struct Employee {
    employee_name: String,
    employee_id: i32,
    email: String,
    age: i32,
    phone_number: String,
}

impl Employee {
    fn new(name: String, id: i32, email: String, age: i32, phone: String) -> Employee {
        Employee {
            employee_name: name,
            employee_id: id,
            email,
            age,
            phone_number: phone,
        }
    }
}

fn find_employee_by_id(employees: &[Employee], id: i32) -> Option<&Employee> {
    employees.iter().find(|e| e.employee_id == id)
}

fn find_employees_by_age(employees: &[Employee], age: i32) -> Vec<&Employee> {
    employees.iter().filter(|e| e.age == age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("1. Add Employee");
        println!("2. Find Employee by ID");
        println!("3. Find Employees by Age");
        println!("4. Exit");
        println!("Enter your choice: ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let choice: i32 = input.trim().parse().expect("Invalid input");

        match choice {
            1 => {
                println!("Enter Employee Name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                println!("Enter Employee ID: ");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                let id: i32 = id_str.trim().parse().expect("Invalid input");

                println!("Enter Employee Email: ");
                let mut email = String::new();
                io::stdin().read_line(&mut email).expect("Failed to read line");

                println!("Enter Employee Age: ");
                let mut age_str = String::new();
                io::stdin().read_line(&mut age_str).expect("Failed to read line");
                let age: i32 = age_str.trim().parse().expect("Invalid input");

                println!("Enter Employee Phone Number: ");
                let mut phone = String::new();
                io::stdin().read_line(&mut phone).expect("Failed to read line");

                employees.push(Employee::new(name, id, email, age, phone));
                println!("Employee added successfully!");
            }
            2 => {
                println!("Enter Employee ID to search: ");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                let id: i32 = id_str.trim().parse().expect("Invalid input");

                match find_employee_by_id(&employees, id) {
                    Some(employee) => {
                        println!("Employee found:");
                        println!("Employee Name: {}", employee.employee_name);
                        println!("Employee ID: {}", employee.employee_id);
                        println!("Employee Email: {}", employee.email);
                        println!("Employee Age: {}", employee.age);
                        println!("Employee Phone Number: {}", employee.phone_number);
                    }
                    None => println!("Employee not found."),
                }
            }
            3 => {
                println!("Enter Age to find employees with the same age: ");
                let mut age_str = String::new();
                io::stdin().read_line(&mut age_str).expect("Failed to read line");
                let age: i32 = age_str.trim().parse().expect("Invalid input");

                let matching_employees = find_employees_by_age(&employees, age);
                if matching_employees.is_empty() {
                    println!("No employees found with age {}.", age);
                } else {
                    println!("Employees with age {}:", age);
                    for employee in matching_employees {
                        println!("Employee Name: {}", employee.employee_name);
                        println!("Employee ID: {}", employee.employee_id);
                        println!("Employee Email: {}", employee.email);
                        println!("Employee Phone Number: {}", employee.phone_number);
                    }
                }
            }
            4 => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
