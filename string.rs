struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    // Adding some sample student details to the vector
    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phno: String::from("123-456-7890"),
        id: 1,
    });

    students.push(Student {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phno: String::from("987-654-3210"),
        id: 2,
    });

    students.push(Student {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        phno: String::from("555-555-5555"),
        id: 3,
    });

    students.push(Student {
        name: String::from("David"),
        email: String::from("david@example.com"),
        phno: String::from("111-222-3333"),
        id: 4,
    });

    students.push(Student {
        name: String::from("Eve"),
        email: String::from("eve@example.com"),
        phno: String::from("444-444-4444"),
        id: 5,
    });

    // Accessing students by index with error handling
    let index = 3; // Change this index to test different scenarios

    if let Some(student) = students.get(index) {
        println!("Student Details:");
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
        println!("Phone Number: {}", student.phno);
        println!("ID: {}", student.id);
    } else {
        eprintln!("Error: Student at index {} does not exist.", index);
    }
}