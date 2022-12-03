use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let target = rand::thread_rng().gen_range(1..=1000);

    println!("Welcome! Today we will guess a number...");

    loop {
        println!("Input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("{} is not a number. You idiot!", guess.trim());
                continue;
            }
        };

        match guess.cmp(&target) {
            Ordering::Less => println!("{guess} is too small. Think bigger!"),
            Ordering::Greater => println!("{guess} is too big. Think smaller!"),
            Ordering::Equal => {
                println!("{guess} is correct. You win!");
                break;
            }
        }
    }
}
