use std::io;
use std::ops::Add;

fn main() {
    println!("Hello, world!");
}

enum ArithmeticOperations {
    Add,
    Subtract,
    Multiply,
    Divide,
}
fn calculate(a: i32, b :i32, operation :ArithmeticOperations) -> i64 {
    match operation {
        ArithmeticOperations::Add => { a as i64 + b as i64 },
        ArithmeticOperations::Subtract => { a as i64 - b as i64 },
        ArithmeticOperations::Multiply => { a as i64 * b as i64 },
        ArithmeticOperations::Divide => { a as i64 / b as i64 },
    }
}

fn read_numbers(left_number: &mut String,
                right_number: &mut String,
                operation: &mut ArithmeticOperations) -> Result<(), String> {
    println!("Enter a first number you want to operate:");
    io::stdin()
        .read_line(left_number)
        .expect("Failed to read first number");

    println!("Enter a second number you want to operate:");
    io::stdin()
        .read_line(right_number)
        .expect("Failed to read second number");



    let mut raw_operation= String::new();
    println!("Enter a second number you want to operate (+, -, *, /):");
    io::stdin()
        .read_line(&mut raw_operation)
        .expect("Failed to read arithmetic operation");

    match raw_operation.as_str() {
        "+" => { *operation = ArithmeticOperations::Add; },
        "-" => { *operation = ArithmeticOperations::Subtract; },
        "*" => { *operation = ArithmeticOperations::Multiply; },
        "/" => { *operation = ArithmeticOperations::Divide; },
        _ => return Err("Incorrect input. Make sure you pass one of [+, -, *, /] without any \
        different symbols".to_string())

    }

    return Ok(())
}

