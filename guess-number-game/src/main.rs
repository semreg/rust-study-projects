extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut counter: usize = 0;

    println!("Guess the number!\n");

    let random_number = rand::thread_rng()
        .gen_range(1, 101);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => {
                counter += 1;
                println!("Too small!\n");
            },

            Ordering::Greater => {
                counter += 1;
                println!("Too big!\n");
            },

            Ordering::Equal => {
                counter += 1;
                println!("Congrats! You win!\nIt took {} guessing attempt(s)!", counter);
                break;
            }
        }
    }
}
