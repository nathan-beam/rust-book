fn main() {
    for i in 1..13 {
        let day = get_day(i);
        println!("On the {day} day of Christmas my true love gave to me, ");
        print_gifts(i);
    }
}

fn get_day(i: u32) -> &'static str {
    match i {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        _ => "twelfth",
    }
}

fn print_gifts(i: u32) {
    if i >= 12 {
        println!("Twelve drummers drumming,");
    }
    if i >= 11 {
        println!("Eleven pipers piping,");
    }
    if i >= 10 {
        println!("Ten lords a-leaping,");
    }
    if i >= 9 {
        println!("Nine ladies dancing,");
    }
    if i >= 8 {
        println!("Eight maids a-milking,");
    }
    if i >= 7 {
        println!("Seven swans a-swimming,");
    }
    if i >= 6 {
        println!("Six geese a-laying,");
    }
    if i >= 5 {
        println!("!! Five golden rings !!");
    }
    if i >= 4 {
        println!("Four calling birds,");
    }
    if i >= 3 {
        println!("Three French hens,");
    }
    if i >= 2 {
        println!("Two turtle doves,");
        println!("And a partridge in a pear tree.");
    }
    if i == 1 {
        println!("A partridge in a pear tree.");
    }
    println!();
}
