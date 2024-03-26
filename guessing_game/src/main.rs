use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number was: {}", secret_number);

    loop {
        println!("Please input the number");
        let mut guess: String = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("The Number you guessed was: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "The number you guessed is lower than the secret number".red()),
            Ordering::Greater => println!("{}","The number you guessed is greater than the secret number.red()"),
            Ordering::Equal =>  {
                println!("{}", "You Win! Congratulations!!!".green()); 
        break
    },

        }
    }
}
