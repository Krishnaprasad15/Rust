use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("secret number is {secret_number}");
    
    loop {
        println!("Guess a number: ");   
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        // print!("Num is {guess}");
        let guess: u32 = guess.trim().parse().expect("Failed to parse");

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too low\n"),
            Ordering::Greater => print!("Too high\n"),
            Ordering::Equal => {
                print!("Yayyyy! You guessed the right number\n");
                break;
            }
        }
    }
}
