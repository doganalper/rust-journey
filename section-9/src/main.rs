/*
Rust has two major categories for errors: `recoverable` and `unrecoverable` errors
- RECOVERABLE ERRORS
* For this errors we most likely just want to report the problem to the user.
* Example: Trying to access not existing file (file not found) Here we might create if it does not exist and
* continue the program

- UNRECOVERABLE ERRORS
* They are symptoms of bugs
* Example: Trying to access a location beyond the end of an array. Here it is better if we quit program.

Rust does not have exceptions. It has type `Result<T, E>` for recoverable errors and `panic!` macro that stops execution for unrecoverable errors.
*/

use core::panic;
use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // unrecoverable_errors();
    // recoverable_errors();
    // For better understanding when to and not to panic your program please read https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
}

fn recoverable_errors() {
    /*
    Most errors are not serious enought to stop program entirely. `Result` enum has two variants, `Ok` and `Err`:
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
    */

    let file_name = "hello.txt";
    let greeting_file_result = File::open(file_name);
    // Return type of `File::open` is `Result<T,E>`.
    // If `open` function succeeds, the value in the variable will be an instance of `Ok` and it will contain a file handle
    // If it fails, variable will be an instance of Err and it contains more about kind of error.
    // Both of these scenarios can be caught by using `match`
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file {:?}", error), // Catch-all type of handling errors
        Err(error) => match error.kind() {
            // If typeof error when opening file is not found, try creating new file
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };

    // More cleaner version of previous code
    // Here if Result is successful (which is `Ok` enum variant) it will return it directly
    // if it failed (Err enum variant) it will run given function
    let greeting_file = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name)
                .unwrap_or_else(|err| panic!("Problem creating the file: {:?}", err))
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    // Anoter version of handling Result
    // `unwrap` will return the value inside if result is `Ok`
    // if result is `Err`, `unwrap` will call `panic!` macro
    let greeting_file = File::open(file_name).unwrap();

    // Panicing with custom message on fail
    // `expect` will take a string slice that will be displayed if result is `Err` otherwise it will return value inside
    // this is more frequently used since developers can give more information about why program failed to run.
    let greeting_file = File::open(file_name)
        .expect(format!("{} should be included in this project.", file_name).as_str());

    // PROPAGATING ERRORS
    // this return type says it may return String if calculations inside are Ok and may return an error if it is not Ok
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e), // this return is for function itself
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e), // since match is not an expression it will return from function
        }
    }

    // since called function returns an Result it is up to consumer of this function to handle error.
    let username_from_file = read_username_from_file();

    // `?` OPERATOR
    /*
    `?` operator placed after a `Result` value is defined to work almost same as `match` expression.
    If result is `Ok` it will return value inside and if it is `Err` it will return from the function not to variable
    `?` operator can only be used in functions with return type is compatible with the value `?` operator is used on.
    If we try to use `?` with a variable with `Option` and our function returns a `Result` it would not be compatible. 
    */
    fn read_username_from_file_shorter() -> Result<String, io::Error> {
        // ONE WAY TO SHORTEN IT.
        // let mut username_file = File::open("hello.txt")?;
        // let mut username = String::new();

        // username_file.read_to_string(&mut username)?;
        // Ok(username)

        // ANOTHER WAY TO SHORTEN IT.
        // let mut username = String::new();
        // Here if it fails on first ? operator it will return error from it and if it fails on second ? operator it will return error from it
        // If no fails happens it will return Ok with username
        // File::open("hello.txt")?.read_to_string(&mut username)?;
        // Ok(username)

        // ANOTHER WAY TO MORE SHORTEN IT.
        // Reading from file is so common that standard library provides a function that takes care of what we are trying to do.
        fs::read_to_string("hello.txt")
    }
}

fn unrecoverable_errors() {
    /*
    There are two ways of causing panic in practice:
    * Taking an action that will definitely cause panic (accessing an array past the end, knowingly)
    * By explicitly calling the `panic!` macro.
    By default, both of these panics will print a failure message, unwind, clean up the stack and quit.
    NOTE: Unwind means when panic occurs, Rust walks back up the stack and cleans up the data from each function it run. This is a lot of work. Another behavior is `abort` which means Rust does not clean and leave it to OS.
    */

    // panic!("crash and burn")
    /*
    Panic error message contains:
    * First line is where our code paniced. Which file and line it paniced. (src/main.rs:2:5 means line 2 char 5 in main.rs file)
    * On occasions where panic occurs on a code that we call from external crate, it will display file, line information on external code and we can backtrace it to where we call that code.
    */

    env::set_var("RUST_BACKTRACE", "1"); // for backtracing, works only on debugging (no `--release` flag)

    let v = vec![1, 2, 3];

    v[99]; // this will panic since we cannot access item on given index because it does not exist on vector
}
