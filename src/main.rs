use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    let mut guess = 50;
    let mut upper_limit = 100;
    let mut lower_limit = 0;

    loop {

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{guess} is too small! because secret number is {secret_number}");
                lower_limit = guess;
                guess = (upper_limit - lower_limit) / 2 + guess;
            },
            Ordering::Greater => {
                println!("{guess} is too Big! because secret number is {secret_number}");
                upper_limit = guess;
                guess = guess - (upper_limit - lower_limit) / 2;
            },
            Ordering::Equal => {
                println!("YOU WIN!!!");
                break;
            }
        }
    }
}
