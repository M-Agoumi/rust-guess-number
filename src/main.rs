use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_validated_guess(attempt: u8) -> u32 {
    // read and validate guess
    loop {
        println!("Please input your guess, you have {attempt} attempts left");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Filed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter valid number");
                continue;
            }
        };

        if guess < 1 || guess > 99 {
            println!("number must be >= 1 and <= 99");
            continue;
        }

        return guess;
    }
}

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempt: u8 = 6;

    loop {
        if attempt == 0 {
            println!("you lost, correct number is {secret_number}");
            break;
        }

       let guess: u32 = get_validated_guess(attempt);

        println!("You guessed {guess}");
        attempt -= 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won in {} attempts!", 6 - attempt);
                break;
            }
        }
    }
}
