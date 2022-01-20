use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Lets get started with the guessing game");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is {}", secret_number);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse() {
            
        };

        println!("You guessed it as {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            },
        }
    }
}
