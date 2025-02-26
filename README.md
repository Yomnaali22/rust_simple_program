# Simple Calculator in Rust

## Overview
This is a simple command-line calculator implemented in Rust. It utilizes an `enum` called `Operation` to represent four basic arithmetic operations: addition, subtraction, multiplication, and division. The program takes user input, parses it into numbers and an operation type, performs the calculation using pattern matching, and displays the result.

## Features
- Supports addition, subtraction, multiplication, and division.
- Uses Rust's `enum` and pattern matching for clean and structured code.
- Handles user input parsing and validation.
- Provides error handling for invalid operations.

## Implementation Details

### Enum Definition
The `Operation` enum is defined with four variants, each holding two `f64` values:
```rust
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
```

### Calculate Function
The `calculate` function is implemented using pattern matching to perform the appropriate arithmetic operation:
```rust
impl Operation {
    fn calculate(self) -> Result<f64, String> {
        match self {
            Self::Add(a, b) => Ok(a + b),
            Self::Subtract(a, b) => Ok(a - b),
            Self::Multiply(a, b) => Ok(a * b),
            Self::Divide(a, b) => {
                if b == 0.0 {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}
```

### Handling User Input
A `UserInput` struct is used to store the user inputs:
```rust
struct UserInput {
    number1: f64,
    number2: f64,
    text: String,
}
```
A function `handle_user_input` is implemented to convert user input into an `Operation` enum instance and call the `calculate` function.

### Main Function Workflow
1. Prompts the user for the first number.
2. Prompts the user for the second number.
3. Prompts the user for an operation (`add`, `subtract`, `multiply`, `divide`).
4. Parses the inputs and creates an `Operation` enum instance.
5. Calls the `calculate` function and displays the result.
6. Handles errors for invalid input or operations.

## How to Run the Program
### Prerequisites
- Rust installed on your system. If not, install it from [Rust official website](https://www.rust-lang.org/).

### Steps
1. Clone this repository or copy the code into a new Rust project.
2. Navigate to the project directory.
3. Compile and run the program using the following command:
   ```sh
   cargo run
   ```
4. Follow the on-screen prompts to enter numbers and an operation.

## Example Usage
```
Please enter the first number:
5
Please enter the second number:
3
Please enter a string:
add
Result: 8.0
```

## Error Handling
- If the user enters an invalid number, the program will display an error message and exit.
- If an unsupported operation is entered, an error message will be shown.
- Division by zero is handled gracefully with an appropriate error message.

## Future Improvements
- Add more operations (e.g., exponentiation, modulus).
- Implement a loop to allow multiple calculations without restarting the program.
- Improve input handling with a more robust error recovery mechanism.

## Conclusion
This simple calculator demonstrates Rust's `enum`, pattern matching, and user input handling. It provides a solid foundation for building more complex CLI applications in Rust.
