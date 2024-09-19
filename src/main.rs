use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        // println!("The secret number is: {}", secret_number);
        println!("Please input your guess.");

        // read_line returns Result enum with Ok and Err variants
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:i32 = match guess.trim().parse() {
            Ok(secret_number) => secret_number,
            Err(err) => {
                println!("The error is: {}", err);
                println!();

                let exit_cmd: String = match guess.trim().parse() {
                    Ok(res) => res,
                    Err(_) => {
                        continue;
                    }
                };

                if exit_cmd == "quit" {
                    println!();
                    println!("Finishing the program.....");
                    break;
                }

                println!("Please, try again by specifying a number value!");
                continue;
            }
        };

        println!("Your value is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                println!();
            },
            Ordering::Greater => {
                println!("Too big!");
                println!();
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }



}
