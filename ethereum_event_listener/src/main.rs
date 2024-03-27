mod web_3;

use std::io;

fn main() {
    println!("Point72 Blockchain Menu: \n(1) listener\n");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();

            if input == "1" {
                let _ = web_3::read_block_data();
            } else {
                eprintln!("Error: choose 1 or 2 ");
            }
        }
        Err(error) => {
            // If an error occurred, print the error message
            eprintln!("Error reading input: {}", error);
        }
    }
}
