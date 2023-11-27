use std::io::stdin;

fn main() {
    println!("Enter temperature: ");

    let mut temperature = String::new();

    stdin()
        .read_line(&mut temperature)
        .expect("There was an error with your input.");

    let temperature: f64 = temperature.trim().parse().expect("Please enter number.");

    let mut degree = String::new();

    println!("Enter degree: ");

    stdin()
        .read_line(&mut degree)
        .expect("There was an error with your input.");

    match degree.trim().to_uppercase().as_str() {
        "F" => println!("It is {}°C", (temperature - 32.0) * 5.0 / 9.0),
        "C" => println!("It is {}°F", (temperature * (9.0 / 5.0)) + 32.0),
        _ => println!("Invalid degree type please enter C or F"),
    }
}
