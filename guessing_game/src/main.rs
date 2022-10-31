use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let system_guess = generate_random();

    println!("Please input your guess.");

    //loops can be labeled and controlled with label used in conjunction with break or continue
    'game_loop: loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // println!("The system guessed: {}", system_guess);

        match guess.cmp(&system_guess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The system guessed {} as well!", system_guess);
                //break loop with the label
                break 'game_loop;
            }
        }
    }
}

fn generate_random() -> u32 {
    return rand::thread_rng().gen_range(1..=100)
}
