fn main() {
    demonstrate_if_else();
    demonstrate_if_else_if_else();
    demonstrate_if_let();
    demonstate_loop();
    demonstate_loop_labels();
    demonstrate_while();
    demonstrate_for();
}

fn demonstrate_if_else() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn demonstrate_if_else_if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn demonstrate_if_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn demonstate_loop() {
    let mut i = 0;

    let result = loop {
        println!("{} ", i);

        i += 1;
        if i >= 3 {
            break i * 2;
        }
    };

    println!("The result is {}", result);
}

fn demonstate_loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remainting = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End: count = {count}");
}

fn demonstrate_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn demonstrate_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
