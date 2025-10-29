use std::io;
use std::io::Write;

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
            0 | 1 => break,
            _ => {
                println!("Enter a number between 0 and 1.");
                continue;
            },
        };
    }
}
