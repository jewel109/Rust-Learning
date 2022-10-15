use rand::Rng;
use std::cmp::Ordering;
use std::io ;
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        println!("The secret number is : {}", secret_number);

        println!("please inpput your  guess");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed : {}", guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num ) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Too big".yellow()),
            Ordering::Equal => {
                println!("{}","You win".green());
                break;
            }
        };
    }
}
