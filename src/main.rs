//Including the library here
// This is called a prelude
use std::io;
use std::cmp::Ordering; // This was neccesary because this exists only in standard io files
use rand::Rng;

fn main() {
	println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  // println!("The secret number is: {secret_number}");

  loop{

    println!("Please input your guess");
    let mut guess = String::new(); 
    io::stdin()
        .read_line(&mut guess) // The & refers to a reference and the mut is added so that the
        .expect("Failed to read line"); // Need to read up on the enums explained here but

    let guess: u32 = guess.trim()
        .parse()
        .expect("Failed to convert to int. Please type a number!"); // So essentially, here we are
    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) { // so cmp is a method really, that can be used to compare
        Ordering::Less => println!("Too small!"), // So this Ordering::Less is called an arm and
        Ordering::Greater => println!("Too big!!"),
        Ordering::Equal => {
            println!("You win! You guessed the correct number");
            break;
        },
    }
  }
}
