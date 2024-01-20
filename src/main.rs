//use rand::Rng;
use std::io;

fn parse(expression: &str) -> String {
    let operands = "+-*/";
    let mut math_expression = String::new();
    for c in expression.chars() {
        if c.is_numeric() || operands.contains(c) {
            math_expression.push(c);
        }
        else {
            math_expression = "Invalid Character".to_string();
        }
    }
    return math_expression;
}


fn treat_input(mut input: String) -> String {
    // lose the newlines and whitespace
    input = input.trim().to_string();

    // only keep first character of string
    input.truncate(1);

    input = parse(&input);

    return input;
}

fn get_input() -> String {
    println!("Waiting on input...");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = treat_input(input);

    return input;
}


fn main() {
    println!("Wrongculator is now active!");

    loop {
        let input = get_input();
        //println!("Your input: {input}");
        //println!("Your expression: {current_expression}");
        println!("Your math expression: {input}");
        let result = 2;
        println!("Result = {result}");
    };
}
