use std::io; 
use std::cmp::Ordering; // Ordering is an enum
use rand::Rng; // Rng is a trait
use colored::*; // Colored is a trait

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101); // gen_range is a method on the Rng trait
    println!("{}", ["The secret is: ", &secret_number.to_string()].join(" ").black());

    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");
    
        let mut guess = String::new(); // new empty string -> same as calling a static method on the String class
    
        // Error messages
        let input_error_msg = "Failed to read line!".red();
        let type_error_msg = "Please enter a number!".red();

        io::stdin()
            .read_line(&mut guess) // read_line returns a Result type
            .expect(&input_error_msg); // expect is a method on Result types
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}",type_error_msg);
                continue;
            },
        };
        
        //compare guess to secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".yellow()),
            Ordering::Greater => println!("{}","Too big!".yellow()),
            Ordering::Equal => {
                // Anonymus function
                println!("{}","You win!".green());
                break;
            }
        }
    }


}
