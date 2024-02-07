use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    const LEN:i32 = 100;
    let mut guess_counter_array = [0; LEN as usize];
    let mut index = 0;

    while index < LEN {
        guess_counter_array[index as usize ] = guess_the_number();
        index+=1;
    }
    let sum:i32 = guess_counter_array.iter().sum();
    let average = sum / LEN;
    print!("average guesses need to find secret number is: {average}, based on average of {LEN } tries.")
}

fn guess_the_number() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..100);

    let mut guess = 50;
    let mut upper_limit = 100;
    let mut lower_limit = 0;
    let mut counter = 0;

    loop {
        counter += 1;
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
    return counter;
} 
