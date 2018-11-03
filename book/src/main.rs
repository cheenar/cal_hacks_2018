extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
          
            Err(_) => continue
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Less motheruccker"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }

}