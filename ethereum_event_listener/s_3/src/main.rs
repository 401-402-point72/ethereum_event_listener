mod s_3;

use std::io;

fn main() {
    println!("Point72 Blockchain Menu: \n(2) Store Data in S3\n");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();

            // if input == "1" {
            //     let _ = web_3::read_block_data();
            /* } else */if input == "2" {
                if let Err(e) = s_3::main() {
                    eprintln!("Error running S3 main: {:?}", e);
                }
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
