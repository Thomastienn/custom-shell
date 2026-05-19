pub struct Echo;

impl super::Runnable for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, args: &[&str]) -> i32 {
        println!("{}", args.join(" "));
        0
    }
}
