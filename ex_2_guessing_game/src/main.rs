extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_num);

    loop {
        println!("Guess the number!");
        println!("Put in your guess: ");

        let mut guess = String::new(); //

        let str_len = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        println!("String len: {}", str_len);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        } ;
            //.expect("Please insert the number!");

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("To small!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
            Ordering::Greater => println!("To big!"),
        }
    }
}
