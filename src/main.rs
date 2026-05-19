#[allow(unused_imports)]
use std::io::{self, Write};

mod runnable;

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let commands = runnable::get_commands();
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts = input.trim().split_whitespace().collect::<Vec<&str>>();
        let command = parts[0];
        let args = &parts[1..];

        runnable::dispatch(&commands, command, args);
    }
}
