#![allow(non_snake_case)]
use std::io;

enum Operation{
    Add(f64, f64), 
    Subtract(f64, f64), 
    Multiply(f64, f64),
    Divide(f64, f64)
}

fn main() {
    println!("Enter the first number:");
    let mut first_num = String::new();
    io::stdin().read_line(&mut first_num).expect("Failed to read input");
    let first_num: f64 = first_num.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

    println!("Enter the second number:");
    let mut second_num = String::new();
    io::stdin().read_line(&mut second_num).expect("Failed to read input");
    let second_num: f64 = second_num.trim().parse().expect("Invalid input. Please enter a number.");

    let op = match operation {
        "Add" => Operation::Add(first_num, second_num),
        "Subtract" => Operation::Subtract(first_num, second_num),
        "Multiply" => Operation::Multiply(first_num, second_num),
        "Divide" => Operation::Divide(first_num, second_num),
        _ => {
            println!("Invalid operation. Please choose from Add, Subtract, Multiply, or Divide.");
            return;
        }
    };

    let result = calculate(op);

    println!("Result: {}", result);
}

fn calculate(x: Operation) -> f64{
    match x {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(c, d) => c - d,
        Operation::Multiply(e, f) => e * f,
        Operation::Divide(q, w) => q / w
    }
}