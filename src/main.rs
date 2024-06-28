extern crate num;

use num::BigUint;
use std::io;
use std::str::FromStr;

fn factorial(number: BigUint) -> BigUint {
    let big_1 = BigUint::from(1u32);
    let big_2 = BigUint::from(2u32);
    if number < big_2 {
        big_1
    } else {
        let prev_factorial = factorial(number.clone() - &big_1);
        number * prev_factorial
    }
}

fn main() {
    println!("Factorial Calculator");
    println!("Please enter a positive integer (or 'exit' to quit):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    if input.to_lowercase() == "exit" {
        println!("Exiting...");
        return;
    }

    match BigUint::from_str(input) {
        Ok(number) => {
            println!("{}", factorial(number));
        }
        Err(_) => {
            println!("Please enter a valid positive integer.");
        }
    }
}
