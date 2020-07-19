use std::io::{self, Write};

fn main() {
    println!("Guess what... We are still basically copy-pasting the documentation.");

    print!("Please input your guess: ");

    // print doesn't flush on it's own.
    // unwrap() is used to get T from an Result<T,E> or panic.
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

        println!("You guessed: {}", guess);
}
