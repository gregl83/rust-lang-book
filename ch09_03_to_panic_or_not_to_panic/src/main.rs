use std::io;
use std::cmp::Ordering;
use std::net::IpAddr;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // programmer knows hard-coded IP is valid, thus unwrap is warranted
    let _home: IpAddr = "127.0.0.1".parse().unwrap();

    // WORDS OF GOLD (Rust Book):
    //
    // "It’s advisable to have your code panic when it’s possible that your code could end up in a
    // bad state. In this context, a bad state is when some assumption, guarantee, contract,
    // or invariant has been broken, such as when invalid values, contradictory values,
    // or missing values are passed to your code—plus one or more of the following:
    //
    // - The bad state is not something that’s expected to happen occasionally.
    // - Your code after this point needs to rely on not being in this bad state.
    // - There’s not a good way to encode this information in the types you use."

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}
