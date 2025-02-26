enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

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

struct UserInput {
    number1: f64,
    number2: f64,
    text: String,
}

fn handle_user_input(input: &UserInput) -> Result<f64, String> {
    match input.text.trim().to_lowercase().as_str() {
        // create instances with the parsed number values
        "add" => Operation::Add(input.number1, input.number2).calculate(),
        "subtract" => Operation::Subtract(input.number1, input.number2).calculate(),
        "multiply" => Operation::Multiply(input.number1, input.number2).calculate(),
        "divide" => Operation::Divide(input.number1, input.number2).calculate(),
        _ => Err("Invalid operation".to_string()),
    }
}
fn main() {
    let mut input: UserInput = UserInput {
        number1: 0.0,
        number2: 0.0,
        text: String::new(),
    };
    // Prompt the user for the first number
    println!("Please enter the first number:");
    let mut number1: String = String::new();
    std::io::stdin()
        .read_line(&mut number1)
        .expect("Failed to read line");
    input.number1 = number1
        .trim()
        .parse::<f64>() // parse value
        .expect("Please enter a valid number");

    // Prompt the user for the second number
    println!("Please enter the second number:");
    let mut number2: String = String::new();
    std::io::stdin()
        .read_line(&mut number2)
        .expect("Failed to read line");
    input.number2 = number2
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number");

    // Prompt the user for a string
    println!("Please enter a string:");
    std::io::stdin()
        .read_line(&mut input.text)
        .expect("Failed to read line");

    let handle_input: Result<f64, String> = handle_user_input(&input); // we can move the input ownership to the function if we want or borrow input if we are going to use the variable latter
    if handle_input.is_ok() {
        println!("Result: {:.1}", handle_input.unwrap());
    } else {
        println!("Error: {}", handle_input.unwrap_err());
    }
}
