use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        match command {
            "exit" => break,
            _ => println!("{}: command not found", command),
        }
    }
}
