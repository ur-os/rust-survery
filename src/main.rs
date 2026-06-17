use std::io;
fn main() {
    println!("Hello, world!");

    let first_number = &mut 0;
    let second_number = &mut 0;
    let operation = &mut ArithmeticOperations::Add;

    let result = read_numbers(first_number, second_number, operation);
    if result.is_err() {
        println!("{}", result.err().unwrap().as_str());
        return
    }


    println!("Result: {}", calculate(*first_number, *second_number, operation))
}

enum ArithmeticOperations {
    Add,
    Subtract,
    Multiply,
    Divide,
}
fn calculate(a: i32, b :i32, operation : &mut ArithmeticOperations) -> i64 {
    match operation {
        ArithmeticOperations::Add => { a as i64 + b as i64 },
        ArithmeticOperations::Subtract => { a as i64 - b as i64 },
        ArithmeticOperations::Multiply => { a as i64 * b as i64 },
        ArithmeticOperations::Divide => { a as i64 / b as i64 },
    }
}

fn read_numbers(left_number: &mut i32,
                right_number: &mut i32,
                operation: &mut ArithmeticOperations) -> Result<(), String> {
    println!("Enter a first number you want to operate:");
    let left_number_raw= &mut String::new();
    io::stdin()
        .read_line(left_number_raw)
        .expect("Failed to read first number");
    *left_number = left_number_raw.trim().parse().expect("Passed incorrect number, supports only i32");


    println!("Enter a second number you want to operate:");
    let right_number_raw = &mut String::new();
    io::stdin()
        .read_line(right_number_raw)
        .expect("Failed to read second number");
    *right_number = right_number_raw.trim().parse().expect("Passed incorrect number, supports only i32");


    let mut raw_operation= String::new();
    println!("Enter a second number you want to operate (+, -, *, /):");
    io::stdin()
        .read_line(&mut raw_operation)
        .expect("Failed to read arithmetic operation");

    match raw_operation.trim() {
        "+" => { *operation = ArithmeticOperations::Add; },
        "-" => { *operation = ArithmeticOperations::Subtract; },
        "*" => { *operation = ArithmeticOperations::Multiply; },
        "/" => { *operation = ArithmeticOperations::Divide; },
        _ => return Err("Incorrect input. Make sure you pass one of [+, -, *, /] without any \
        different symbols".to_string())

    }

    Ok(())
}

