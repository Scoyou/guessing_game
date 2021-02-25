use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        println!("Input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // matches if guess is a number
        // otherwise it just asks for the number again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match compares the argument (&secret_number)
        // to the variable it is called on (guess)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
