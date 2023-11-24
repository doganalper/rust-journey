use std::io; // Here we say we will be using standart library input/output
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // loop will run it's scope until it matches a break
    loop {
        // firstly we are calling `thread_rng` function and it returns a number generater
        // secondly we call `gen_range` to generate number, this function takes a range expression
        // `start..=end` is a range expression and it includes both start and end values
        let generated_number = rand::thread_rng().gen_range(1..=100);

        // `String::new()` will return new instance of String.
        // :: syntax indicates that new is an associated function. It is a function that's implemented on a type.
        let mut guess = String::new();

        // We are using standart in function and calling readline function of it.
        // `read_line` functin takes a parameter and it is about on which string it should store user entered text.
        // note: `read_line` function appends entered text to given string and not overwrite its contents.
        // `read_line` function takes a string reference and that means it takes a reference of variable's placement on memory
        // and saying `&mut` means that we pass mutable reference of guess variable.
        // Potential Failure Handling: on second line function `read_line` returns Result value. This Result value is an enumeration (enum)
        // by calling `expect` function we are saying if result of `read_line` is Err it should print this message and crash the program
        // If we comment `expect` function program will compile but still it will give us a warning about possible danger of not handling error.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // We already have a variable named guess but rust allows us to shadow the previous value with new one.
        // This is called `Shadowing`
        // here first we trim whitespaces on both and end of the original guess variable
        // later we tell rust to parse it's type and type of it should be `u32` because it is the type annotation we give
        // we add a error handling on situations where parsing might fail. (example: entering a char)
        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        // this is another way of handling Result values with pattern matching
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // this will run if Result is handled without an error
            Err(_) => {
                println!("Invalid value is given. Guess again!");
                continue;
            }, // this will run if Result has an error, here it will skip current loop and start from start
        };

        // We can use variables inside strings using curly brackets. Anything between curly brackets will be printed.
        println!("You guessed: {guess}");

        match guess.cmp(&generated_number) {
            Ordering::Less => println!("Too small"), // these are called arms of the enum
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break; // this will break loop 
            },
        }

        println!("Generated number was: {generated_number}");
    }
}

// VARIABLES
// `let apples = 5;` this creates a new variable named "apples" and assign 5 as value to it.
// In Rust, variables are immutable by default. Meaning once given a value to variable, value won't change.
// To change this behave we must mark variable as mutable by adding `mut` after let keyword.
// "let mut bananas = 5" will crete a mutable variable named bananas.
