extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    let secret_num = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_num);_

    println!("Guess the number!");
    println!("Put in your guess: ");

    let mut guess = String::new(); //

    let str_len = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("String len: {}", str_len);

    println!("Your guess: {}", guess);

    match guess.cmp(&secret_num) {
        Ordering::Less => println("To small!"),
        Ordering::Equal =>  println("Just right!"),
        Ordering::Greater => println("To big!"),
    }
}
