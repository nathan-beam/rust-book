use std::io;
use std::process::exit;

fn main() {
    println!("Welcome to the Rust temperature converter!");
    println!(
        "Please enter F to convert farenheit to celsius or C to convert celsius to farenheit."
    );

    let mut starting_scale = String::new();

    io::stdin()
        .read_line(&mut starting_scale)
        .expect("Failed to read line");

    if starting_scale.trim().to_uppercase() == "F" {
        println!("Please enter the temperature in farenheit.");
    } else if starting_scale.trim().to_uppercase() == "C" {
        println!("Please enter the temperature in celsius.");
    } else {
        println!("Invalid input. Please enter F or C.");
        exit(1);
    }

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number.");
            exit(1);
        }
    };

    if starting_scale.trim().to_uppercase() == "F" {
        let celsius = farenheit_to_celsius(temperature);
        println!("The temperature in celsius is: {:.4}", celsius);
    } else if starting_scale.trim().to_uppercase() == "C" {
        let farenheit = celsius_to_farenheit(temperature);
        println!("The temperature in farenheit is: {:.4}", farenheit);
    }
}

fn farenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
