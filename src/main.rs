use std::io; 

fn main() {
    println!("Guess the number!");

    println!("Please input your guess: ");

    let mut guess = String::new(); // new empty string -> same as calling a static method on the String class

    io::stdin()
        .read_line(&mut guess) // read_line returns a Result type
        .expect("Failed to read line"); // expect is a method on Result types

    println!("You guessed: {}", guess);   
}
