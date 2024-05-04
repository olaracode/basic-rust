use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

/**
* This is a simple game of guessing.
* Where the user will have the ability to choose a number
* IF it matches the secret number randomly generated
* Then the user wins
* If not we will hint the user to weather the number is
* A: To Small
* B: To Big
*/
fn guessing_game(){
	println!("Guess the number!");

	let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
	let mut attempt = 1;
	// println!("The secret number is: {}", secret_number);

	loop {
		if attempt > 1 {
			println!("{} {attempt}", "This is your attempt nr:".blue());
		}
		println!("Please input your guess");
		let mut guess: String = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		println!("You guessed: {}", guess);
		match guess.cmp(&secret_number){
			Ordering::Less => println!("{}", "To Small".red()),
			Ordering::Greater => println!("{}", "Too big!".red()),
			Ordering::Equal => {
				println!("{}", "You win!".green());
				break;
			},
		};
		attempt += 1;
	}

}
