//use rand::Rng;
use std::io;

fn parse(expression: &str) -> String {
    let operands = "+-*/";
    let mut math_expression = String::new();
    for c in expression.chars() {
        if c.is_numeric() || operands.contains(c) {
            math_expression.push(c);
        }
    }
    return math_expression;
}


fn treat_input(mut input: String) -> String {
    input = input.trim().to_string();
    input.truncate(1);
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
    let mut current_expression = String::new();

    loop {
        let input = get_input();
        current_expression = format!("{current_expression}{input}");
        let math_expression = parse(&current_expression);
        println!("Your input: {input}");
        println!("Your expression: {current_expression}");
        println!("Your math expression: {math_expression}");
    };
}
