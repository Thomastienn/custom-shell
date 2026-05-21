#[allow(unused_imports)]
use std::io::{self, Write};
use runnable::CommandContext;
use tokenizer::Tokenizer;
use parser::parse;

mod runnable;
mod utils;
mod tokenizer;
mod parser;

fn main() {
    let commands = runnable::get_commands();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let ctx = CommandContext {
            commands: &commands,
            parsed_command: &parse(tokens).unwrap(),
        };

        runnable::dispatch(ctx);
    }
}
