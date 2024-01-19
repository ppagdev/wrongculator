//use rand::Rng;
use std::io;

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
        println!("Your input: {input}");
        println!("Your expression: {current_expression}");
    };
}
