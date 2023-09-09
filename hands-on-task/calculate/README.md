## Calculator

This is a simple calculator program implemented in Rust that uses enums and pattern matching to perform arithmetic operations.

By following these steps and running the program, the user can input two numbers and an operation, and the program will perform the arithmetic operation using enums and pattern matching. The result will be displayed on the console.

This simple calculator program demonstrates the use of enums and pattern matching in Rust to perform different actions based on the variant of an enum. It can be expanded upon to include additional operations or error handling as needed.

### Steps

1. The `Operation` enum is defined with four variants: `Add`, `Subtract`, `Multiply`, and `Divide`. Each variant holds two `f64` values, representing the operands for the arithmetic operation.

2. The `calculate` function is implemented with the signature `fn calculate(operation: Operation) -> f64`. This function takes an `Operation` enum as an argument and returns an `f64` result.

3. Inside the `calculate` function, pattern matching is used to match each variant of the `Operation` enum. Depending on the variant, the appropriate arithmetic operation is performed and the result is returned.

4. In the `main` function, the user is prompted to input the first number, the operation to be performed, and the second number. The `println!` and `read_line` functions from the `std::io` module are used for this purpose.

5. The user input is parsed into appropriate variables. The first number is stored in `first_number`, the operation is stored in `operation`, and the second number is stored in `second_number`.

6. An instance of the `Operation` enum is created using the parsed input values. The appropriate variant is chosen based on the `operation` variable.

7. The `calculate` function is called with the created `Operation` enum instance. The result of the calculation is stored in the `result` variable.

8. The result is printed to the console using the `println!` macro.

9. The program can be compiled and run using the `cargo build` and `cargo run` commands.