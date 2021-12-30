pub use std::{io, cmp::Ordering};
use rand::Rng;
use colored::*;

fn main() {
    let number_to_guess = rand::thread_rng().gen_range(1..100);
    
    loop {
        println!("Guess my number!");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&number_to_guess) {
            Ordering::Equal => {
                println!("{}", r#"You won!"#.green());
                println!("It was indeed indeed {}", number_to_guess);
            break;  
            }
            Ordering::Greater => println!("{}", "Too High!".red()),
            Ordering::Less => println!("{}", "Too Low!".red()),
        }
    }
    
}
