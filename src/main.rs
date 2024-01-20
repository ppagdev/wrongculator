mod constants;

use shunting::MathContext;
use shunting::ShuntingParser;
//use rand::Rng;
use std::io;

fn parse(expression: &str) -> String {
    let mut math_expression = String::new();
    for c in expression.chars() {
        if c.is_numeric() || constants::OPERANDS.contains(c) {
            math_expression.push(c);
        }
        else {
            math_expression = constants::ERROR_MESSAGE.to_string();
            return math_expression;
        }
    }

    return math_expression;
}


fn treat_input(mut input: String) -> String {
    // lose the newlines and whitespace
    input = input.trim().to_string();

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

        let expression = ShuntingParser::parse_str(&input).unwrap();

        let result = MathContext::new().eval(&expression).unwrap();

        println!("{input} = {result}");
    };
}
