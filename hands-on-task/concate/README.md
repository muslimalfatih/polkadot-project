# Rust String Concatenation

This Rust program demonstrates the concepts of ownership, borrowing, and references by concatenating two strings.

## Code Explanation

- The `concatenate_strings` function takes two string slices (`&str`) as arguments, which are references to the input strings.

- Inside the `concatenate_strings` function, a new `String` called `result` is created and initialized with the content of the first input string (`s1`) using `String::from`. Then, the `push_str` method is used to append the contents of the second input string (`s2`) to `result`.

- The `result` string is returned from the function.

- In the `main` function, two `String` variables (`string1` and `string2`) are created and initialized with the desired string values.

- The `concatenate_strings` function is called with references to `string1` and `string2` as arguments using `&`. This allows us to borrow the strings without transferring ownership.

- The result of the concatenation is stored in the `concatenated_string` variable.

- Finally, the concatenated string is printed to the console using `println!`.

## Running the Program

Compile and run this program to see the output, which should be "Hello, world!". This program demonstrates how Rust's ownership and borrowing system allows you to work with strings without taking ownership of them and efficiently concatenate them.
