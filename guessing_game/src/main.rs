use rand::Rng;
use std::io;

fn main() {
    // let secret_number = rand::rng().random_range(1..=100);

    // loop {
    //     println!("Please input your guess.");
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please enter a valid number!");
    //             continue;
    //         }
    //     };
    //     if guess < secret_number {
    //         println!("Too small!");
    //     } else if guess > secret_number {
    //         println!("Too big!");
    //     } else {
    //         println!("You win!");
    //         break;
    //     }
    // }

    let fruits = ["apple", "banana", "cherry", "date", "elderberry"];

    loop {
        let n = rand::rng().random_range(0..=fruits.len() - 1);
        let result = fruits[n];

        println!("Random fruit: {}", result);

        println!("Please Enter fruit name:");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "-1" {
                    println!("Exiting the game. Goodbye!");
                    break;
                }

                let selected_fruit = input.trim().to_lowercase();

                println!("You entered: {}", selected_fruit);


                if selected_fruit == result {
                    println!("You guessed it right!");
                    break;
                } else {
                    println!("Wrong guess! Try Again for exit enter (-1).");
                }
            }
            Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
            },
            
        }
    }
}
