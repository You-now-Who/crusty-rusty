//Including the library here
// This is called a prelude
use std::io;
use std::cmp::Ordering; // This was neccesary because this exists only in standard io files
use rand::Rng;

fn main() {
	println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

println!("The secret number is: {secret_number}");

	println!("Please input your guess");
	let mut guess = String::new(); 
	io::stdin()
      .read_line(&mut guess) // The & refers to a reference and the mut is added so that the
                             // reference/pointer is actually mutable because rust treats
                             // everything as being immutable.
      .expect("Failed to read line"); // Need to read up on the enums explained here but
                                      // essentially, Rust returns a enumerated Result (is it a
// type?)

  let guess: u32 = guess.trim()
      .parse()
      .expect("Failed to convert to int. Please type a number!"); // So essentially, here we are
                                                                  // redefining the variable of
                                                                  // guess to be an unsigned
                                                                  // integer, and parsing the
                                                                  // String to int so that we are
                                                                  // able to compare ints with ints
  
	println!("You guessed: {}", guess);
  
  match guess.cmp(&secret_number) { // so cmp is a method really, that can be used to compare
                                    // values of the same type. Do they undergo a typechange?
                                    // This really feels so weird, so they expect a String but....
                                    // Order as int? Or is it order by ASCII?
      Ordering::Less => println!("Too small!"), // So this Ordering::Less is called an arm and
                                               // is... like a type? Or a specific value?
      Ordering::Greater => println!("Too big!!"),
      Ordering::Equal => println!("You win! You guessed the correct number"),
  }
}
