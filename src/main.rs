use std::io::{self, Read};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("DO U WANT TO GUESS MY NUMBER? (Y/N):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Type Y (Yes) or N (NO)");

    if choice.trim() == "Y"{
        let secret_num = rand::thread_rng().gen_range(1..11);
        
        loop {
            println!("Please, enter your number:");
            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("Failed to read line ;-(");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed {guess}");
            
            match guess.cmp(&secret_num) {
                Ordering::Less => println!("My number is bigger."),
                Ordering::Greater => println!("My number is smaller."),
                Ordering::Equal => {
                    println!("You guessed!");
                    break;
                }
            }

        }

        
    }else {
        println!("BYE!")
    }

}
