use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // ..= means inclusive range
    let mut guesses_list: Vec<u32> = Vec::new();

    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            // If an error occurs, the program will crash and display the message passed to expect.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        guesses_list.push(guess);

        // match is like switch, but more powerful.
        // It returns a Ordering enum, which has three variants: Less, Greater, and Equal.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win, with {} guesses!", guesses_list.len());
                break;
            }
        }

        println!("Your guesses: {:?}", guesses_list);
    }
}
