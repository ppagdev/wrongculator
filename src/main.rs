mod constants;

//use rand::Rng;
use std::io;

fn evaluate(input: &str) {
    println!("{input} is ");
    for c in input.chars() {
        if c.is_numeric() {
            println!("a number.");
        }
        else {
            println!("an operand.");
        }
    }
}

fn parse(expression: &str) -> String {
    let mut math_expression = String::new();
    for c in expression.chars() {
        if c.is_numeric() || constants::OPERANDS.contains(c) {
            math_expression.push(c);
        }
        else {
            math_expression = constants::ERROR_MESSAGE.to_string();
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
    println!("WRONGCULATOR IS NOW ACTIVE!!!");

    loop {
        let input = get_input();

        println!("You entered: {input}");
        if input == constants::ERROR_MESSAGE {
            continue;
        }

        evaluate(&input);

        let result = 2;
        println!("Result = {result}");
    };
}
