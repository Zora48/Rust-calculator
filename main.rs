use std::io;

fn main() {
    println!("Rust Calculator");
    println!("===============");

    // Get first number
    println!("Enter first number:");
    let mut num1_input = String::new();
    io::stdin().read_line(&mut num1_input).expect("Failed to read line");
    let num1: f64 = match num1_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    // Get second number
    println!("Enter second number:");
    let mut num2_input = String::new();
    io::stdin().read_line(&mut num2_input).expect("Failed to read line");
    let num2: f64 = match num2_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    // Get operation
    println!("Enter operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    // Calculate result
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    println!("Result: {}", result);
}

