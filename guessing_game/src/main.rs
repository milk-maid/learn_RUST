use rand::Rng; // for randon number generation
use std::cmp::Ordering; // to compare and oeder
use std::io; // for I/O operations
use colored::*; // used to add color to the final print statement

fn main() {
    // Start Game
    println!("Play A Guessing Game");
    let mut attempts: u8 = 5;
    // Use loop to enable game continuity
    while attempts > 0 {
        println!("Kindly note that you have {} attempts left!!!", attempts);
        // User To Input Guess
        println!("Input Your Guess btw 1 - 10");
        // Generate a secret random number using the random (rand) number generator
        let secret_number = rand::thread_rng().gen_range(1..11);
        //  Capture User's Input ..mutable string format
        let mut guess = String::new();
        // Read User's mutable Input on the buffer using the std I/O library with the .expect() msg in case of panic
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Print User's guerss to the std output
        print!("Your guess: {}", guess);
        // then print the secretnumber (the generated random number)
        println!("The secret Number is: {}", secret_number);
        // to enable comparison between the secret randomly generated number and user's guess they both should be of compatible type
        // user's guess is captured as a string WHILE secret_number is captured as a `u32`
        // we trim the guess on both ends then parse it into another type, with the .expect() msg  in case of Err
        // NOTE: annotate the variable to the required type... in this case `u32`

        // let guess: u32 = guess.trim().parse().expect("Enter a Number Please");

        // NOTE: we want to handle panics in case of wrong input. rem: parse() returns Ok or Err
        // we handle error cases by introducing match to help map btw the results and continue in case of ANY panic msgs
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // comparing our values using the `std::cmp` utilities for comparing and ordering   values
        // `cmp` return 3 results, then use match to map each Order to the expected result
        //  Using the `colored` dependency to add color to the Print statement respectively
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Guess Too Small!".red()),
            Ordering::Equal => {
                println!("{}", "Guess Correct! You Win!".green());
                // break out of the loop ONLY when gess is right
                break;
            }
            Ordering::Greater => println!("{}", "Guess Too Big!!!".red())
        }
        attempts -= 1;

        if attempts == 0 {
            println!("{} {}", "Sorry, You ran out of attempts & the secret number was: ".red(), secret_number)
        }
    }
}
