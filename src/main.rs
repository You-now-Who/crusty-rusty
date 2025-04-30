//Including the library here
// This is called a prelude
use std::io;

fn main() {
	println!("Guess the number!");
	println!("Please input your guess");
	let mut guess = String::new();
	io::stdin()
      .read_line(&mut guess) // The & refers to a reference and the mut is added so that the
                             // reference/pointer is actually mutable because rust treats
                             // everything as being immutable.
      .expect("Failed to read line"); // Need to read up on the enums explained here but
                                      // essentially, Rust returns a enumerated Result (is it a
                                      // type?)
	println!("You guessed: {}", guess);

  let x = 2;
  let y = 10;

  println!("x = {x} and y + 2 = {}", y+2);
}
