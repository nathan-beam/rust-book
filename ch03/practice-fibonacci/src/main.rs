use std::io;
use std::process::exit;

fn main() {
    println!("Welcome to the Rust fibonacci generator!");
    println!("Please enter the number of fibonacci numbers you would like to generate.");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number.");
            exit(1);
        }
    };

    println!("Generating {} fibonacci numbers.", n);

    let mut prev: u128 = 0;
    let mut curr: u128 = 1;
    print!("0");

    for _ in 1..n {
        print!(", {}", curr);
        match prev.checked_add(curr) {
            Some(v) => {
                prev = curr;
                curr = v;
            }
            None => {
                println!();
                println!("Whoops! Looks like we've overflowed the u128 type. Exiting.");
                exit(1);
            }
        };
    }
    println!();
}
