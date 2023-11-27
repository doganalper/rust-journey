use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("There was an error with your imput.");

    let input: u32 = input.trim().parse().expect("Invalid number");

    let final_result = fibonacci(input);

    println!("Result: {final_result}")
}

fn fibonacci(num: u32) -> u32 {
    match num {
        0 => return 0,
        1 | 2 => return 1,
        x => return fibonacci(x - 1) + fibonacci(x - 2) 
    }
}
