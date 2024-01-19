use rand::Rng;
use std::io;

fn get_input() -> String {
    println!("Waiting on input...");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}


fn main() {
    let input = get_input();
    println!("Your input: {input}");
}
