pub mod echo;
pub mod exit;

use self::{echo::Echo, exit::Exit};
use std::collections::HashMap;

pub trait Runnable {
    fn name(&self) -> &'static str;
    fn run(&self, args: &[&str]) -> i32;
}

pub fn dispatch(command: &str, args: &[&str]) -> i32 {
    let commands: HashMap<&str, Box<dyn Runnable>> = HashMap::from([
        (Echo.name(), Box::new(Echo) as Box<dyn Runnable>),
        (Exit.name(), Box::new(Exit) as Box<dyn Runnable>),
    ]);

    if let Some(cmd) = commands.get(command) {
        return cmd.run(args);
    }

    println!("{}: command not found", command);
    127
}
