pub mod echo;
pub mod exit;
pub mod r#type;

use std::collections::HashMap;

type CommandMap = HashMap<&'static str, Box<dyn Runnable>>;

pub struct CommandContext<'a> {
    pub commands: &'a CommandMap,
}

pub trait Runnable {
    fn name(&self) -> &'static str;
    fn run(&self, args: &[&str], ctx: CommandContext) -> i32;
    fn is_builtin(&self) -> bool {
        true
    }
}

pub fn get_commands() -> CommandMap {
    HashMap::from([
        (echo::Echo.name(), Box::new(echo::Echo) as Box<dyn Runnable>),
        (exit::Exit.name(), Box::new(exit::Exit) as Box<dyn Runnable>),
        (r#type::Type.name(), Box::new(r#type::Type) as Box<dyn Runnable>),
    ])
}

pub fn dispatch(
    commands: &CommandMap,
    command: &str,
    args: &[&str]
) -> i32 {
    let ctx = CommandContext { commands };
    
    if let Some(cmd) = commands.get(command) {
        return cmd.run(args, ctx);
    }

    println!("{}: command not found", command);
    127
}
