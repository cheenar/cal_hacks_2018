extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Please input your guess.");
    
    let mut guess = String::new();

    let rand_num = rand::thread_rng().gen_range(1, 101);

    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    let guess: u32 = guess.trim().parse().expect("Please type a number1");

    match guess.cmp(&rand_num) {
        Ordering::Less => println!("Less motheruccker"),
        Ordering::Greater => {
            let fun = 4;
            println!("Greatrer");
        },
        Ordering::Equal => println!("Equal"),
    }

}