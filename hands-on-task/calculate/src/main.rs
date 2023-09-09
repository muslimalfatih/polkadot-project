use std::io;
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}
fn main() {
    // Create a mutable string to store user input
    let mut input = String::new();
    println!("Enter the first number:");

    // Read user input for the first number
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid number");

    // Clear the input string for reuse
    input.clear();
    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => panic!("Invalid operation"),
    };
    input.clear();
    println!("Enter the second number:");
    
    // Read user input for the second number
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Read user input for the second number
    let second_number: f64 = input.trim().parse().expect("Invalid number");

    // Create an instance of the Operation enum with the parsed values
    let operation_instance = operation(first_number, second_number);

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation_instance);

    // Print the result to the console
    println!("Result: {}", result);
}
