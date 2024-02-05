use std::io; 
use std::cmp::Ordering; // Ordering is an enum
use rand::Rng; // Rng is a trait


fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101); // gen_range is a method on the Rng trait
    println!("The secret is: {}", secret_number);

    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");
    
        let mut guess = String::new(); // new empty string -> same as calling a static method on the String class
    
        io::stdin()
            .read_line(&mut guess) // read_line returns a Result type
            .expect("Failed to read line"); // expect is a method on Result types
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // subscribing guess to a new variable with a integer type
        
        //compare guess to secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Anonymus function
                println!("You win!");
                break;
            }
        }
    }


}
