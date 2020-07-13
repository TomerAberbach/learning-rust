use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn get_guess() -> u8 {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("I guess you hate guessing...");

        let guess = guess.trim().parse().ok().and_then(|integer: u8| {
            if (1..101).contains(&integer) {
                Some(integer)
            } else {
                None
            }
        });

        match guess {
            None => {
                print!("That's not an integer between 1 and 100! Try again: ");
                io::stdout().flush().unwrap();
                continue;
            }
            Some(integer) => return integer,
        };
    }
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I'm thinking of an integer between 1 and 100!");

    loop {
        print!("What do you think it is? ");
        io::stdout().flush().unwrap();

        match get_guess().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
