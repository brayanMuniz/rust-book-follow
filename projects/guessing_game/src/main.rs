use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Guess a random number
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut tries_left: u32 = 10;
    while tries_left > 0 {
        println!("Enter your guess!");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read it :(");

        // convert to int
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_input.cmp(&secret_number) {
            Ordering::Less => println!("Your number was too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it");
                break;
            }
        }

        tries_left -= 1;
    }

    if tries_left > 0 {
        println!("You won with {tries_left} tries left");
    } else {
        println!("Sorry, you lost");
    }
}
