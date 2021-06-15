use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(0..101);
    println!("Hello, user!");
    println!("Guess a number between 0 and 100!");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Hmmm... that didnt work...");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small, try again!"),
            Ordering::Greater => println!("Too big, try again!"),
            Ordering::Equal => {
                println!("Well played!");
                break;
            }
        }
    }
}
