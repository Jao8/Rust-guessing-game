use std::io; 
use rand::Rng; // Rng is a trait

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101); // gen_range is a method on the Rng trait
    println!("The secret is: {}", secret_number);

    println!("Guess the number!");

    println!("Please input your guess: ");

    let mut guess = String::new(); // new empty string -> same as calling a static method on the String class

    io::stdin()
        .read_line(&mut guess) // read_line returns a Result type
        .expect("Failed to read line"); // expect is a method on Result types

    


}
