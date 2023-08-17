fn add(a: &f64, b: &f64) -> f64 {
    a + b
}

fn subtract(a: &f64, b: &f64) -> f64 {
    a - b
}

fn multiply(a: &f64, b: &f64) -> f64 {
    a * b
}

fn divide(a: &f64, b: &f64) -> Result<f64, &'static str> {
    if *b == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let num1 = 10.0;
    let num2 = 5.0;

    println!("Number 1: {}", num1);
    println!("Number 2: {}", num2);

    println!("Addition: {}", add(&num1, &num2));
    println!("Subtraction: {}", subtract(&num1, &num2));
    println!("Multiplication: {}", multiply(&num1, &num2));

    match divide(&num1, &num2) {
        Ok(result) => println!("Division: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}