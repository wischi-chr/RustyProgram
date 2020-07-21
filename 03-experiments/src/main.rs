use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let reversed = s.chars().rev().collect::<String>();
    let words: Vec<&str> = s.split(' ').filter(|x| x.trim().len() > 1).collect();
    let word_count = words.len();

    test_string(&reversed);
    print_fibonacci(1000);

    println!("Debug: {:?}", (words, word_count));
    println!("Count vowels: {}", count_vowels(&s));
    println!("IsPalindrome: {}", is_palindrome_ascii(&s));
}

fn count_vowels(s: &str) -> i32 {
    s.chars().filter(is_vowel).count() as i32
}

fn is_palindrome_ascii(x: &str) -> bool {
    let bytes = x.as_bytes();
    let len = bytes.len() / 2;

    for i in 0..len {
        if bytes[i] != bytes[bytes.len() - 1 - i] {
            return false;
        }
    }

    return true;
}

fn is_vowel(x: &char) -> bool {
    let vowels = "aeiou";

    for v in vowels.chars() {
        if &v == x {
            return true;
        }
    }
    return false;
}

fn test_string(s: &str) {
    println!("{}", s);
}

fn print_fibonacci(limit: i32) {
    let mut a = 0;
    let mut b = 1;

    while a <= limit {
        println!("{}", a);
        let sum = a + b;
        a = b;
        b = sum;
    }
}
