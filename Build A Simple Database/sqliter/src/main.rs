use std::io::{self, Write};

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

fn main() {
    let mut command = String::new();

    println!("SQLiter version 0.1.0");

    loop {
        print_prompt();

        io::stdin().read_line(&mut command).expect("Failed to read line");

        match command.trim() {
            ".exit" => break,
            _ => print!("Unrecognized command: \'{}\' ", command),
        }
    }
}
