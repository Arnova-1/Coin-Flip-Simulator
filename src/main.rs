use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    println!("───────────────────────\n│ COIN FLIP SIMULATOR │\n───────────────────────\nSelect an action: \n[1] Flip the Coin \n[0] Exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(input) => input,
            Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
            },
        };
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        match input {
            0 => break,
            1 => pick_side(),
            _ => {
                println!("Enter a number between 0 and 1.");
                continue;
            },
        };
    }

    fn pick_side() {
        let random_side = random_side();
        println!("Side: {:?}", random_side);
    }
}

#[derive(Debug)]
enum Side {
    Head,
    Tail,
}

fn random_side() -> Side {
    match rand::rng().random_range(0..=100) {
        1..50 => Side::Head,
        _ => Side::Tail,
    }
}
