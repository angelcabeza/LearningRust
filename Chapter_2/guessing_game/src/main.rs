use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // With thread_rng we indicate that we want to use a generator
    // that is local to the current thread of execution and seeded
    // by the OS and with gen_range we genearte a random number within
    // a range
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}",secret_number);

    loop {
        println!("Please input your guess.");

        // With let we create a variable (with mut we declare it mutable)
        // and bound to a new, empty instance of a String
        let mut guess = String::new();

        // We call the stdin function
        io::stdin()
            // We call the read_line method which takes the user input and append into a string (without overwritting)
            // We have to put mut because a reference is unmutable by default
            .read_line(&mut guess)
            // expect is a mehtod to handle errors
            .expect("Failed to read line");

        // In this line, we create a variable named guess (we are shadowing the guess variable that already existed)
        // and we delete spaces at the beggning and ending of the string, then we convert it to a 32 bits integer with parse,
        // and we check if we could do this operation with expect.
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
