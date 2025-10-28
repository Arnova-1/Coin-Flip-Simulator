use std::io;
use std::io::Write;

fn main() {
    println!("───────────────────────\n│ COIN FLIP SIMULATOR │\n───────────────────────\nSelect an action: \n[1] Flip the Coin \n[0] Exit");

    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
