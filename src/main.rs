use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 99;
const MAX_ATTEMPTS: u8 = 6;

fn get_validated_guess(attempts_left: u8) -> u32 {
    loop {
        println!("Please input your guess, you have {attempts_left} attempts left");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        if guess < MIN_NUMBER || guess > MAX_NUMBER {
            println!("Number must be >= {MIN_NUMBER} and <= {MAX_NUMBER}");
            continue;
        }

        return guess;
    }
}

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);
    let mut attempts_left: u8 = MAX_ATTEMPTS;

    loop {
        if attempts_left == 0 {
            println!("You lost, the correct number is {secret_number}");
            break;
        }

        let guess: u32 = get_validated_guess(attempts_left);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won with {attempts_left} attempts remaining!");
                break;
            }
        }

        attempts_left -= 1;
    }
}
