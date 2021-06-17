fn main() {
    println!(
        "5 degree celsius is {} fahrenheit",
        convert_celsuis_to_fahrenheit(5)
    );
    fibonacci(5);
    lyrics();
}

fn convert_celsuis_to_fahrenheit(c: i32) -> i32 {
    c + 32
}

fn fibonacci(n: i32) {
    let mut fib1 = 0;
    let mut fib2 = 1;
    let mut i = 0;
    while i < n {
        let result = fib1 + fib2;
        println!("fib(n) = {}", result);
        fib1 = fib2;
        fib2 = result;
        i += 1;
    }
}

fn lyrics() {
    let lyrics = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];
    for i in 0..lyrics.len() {
        println!("On the {} day of christmas, my true love sent to me", i + 1);
        let mut idx = lyrics.len() - 1 - i;
        while idx < lyrics.len() {
            println!("{}", lyrics[idx]);
            idx += 1;
        }
        println!("");
    }
}
