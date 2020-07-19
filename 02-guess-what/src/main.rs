use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

// use "cargo fmt" to autoformat everything

fn main() {
    println!("Guess what... We are still basically copy-pasting the documentation.");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    // enable only for development
    // println!("The secret number is: {}", secret_num);

    loop {
        print!("Please input your guess: ");

        // print doesn't flush on it's own.
        // unwrap() is used to get T from an Result<T,E> or panic.
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 Congratulations!");
                break;
            }
        }
    }
}
