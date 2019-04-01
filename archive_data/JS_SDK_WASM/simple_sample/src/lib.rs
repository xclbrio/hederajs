/*
 *
 *	Digital Devices Environment Corporation
 *	D1mk4_22
 *
 */


// Rust code for using in JS, conversioned to WebAssembly
extern crate wasm_bindgen;

use std::io;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn transfer_to(hidden_value: &str) -> u32 {
    println!("Guess a number!");
    
    //let secret_number = rand::thread_rng().gen_range(1, 101);
    
    let _secret_number: u32 = hidden_value.trim().parse().expect("Error!");
    
    //println!("Hidden number: {}", secret_number);
    
    loop {
        
        println!("Please, enter guess");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("String is not available! Try again!");
        
        //let guess: u32 = guess.trim().parse().expect("Please, enter a number");
        
        println!("Please, enter a number:");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your attempt: {}", guess);
        
        return guess;
        
    }
}



//extern crate rand;


//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;


// Rust code, remove brackets and test this code on your local machine
//#[wasm_bindgen]
/*
fn main() {
    
    println!("Guess a number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    //println!("Hidden number: {}", secret_number);
    
    loop {
        
        println!("Please, enter guess");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("String is not available! Try again!");
        
        //let guess: u32 = guess.trim().parse().expect("Please, enter a number");
        
        println!("Please, enter a number:");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your attempt: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal   => {
                println!("Congratulations! Your guessed!");
                break;
            }
        }
        
    }
    
}
*/