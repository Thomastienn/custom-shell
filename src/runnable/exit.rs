pub struct Exit;

impl super::Runnable for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn run(&self, _args: &[&str]) -> i32 {
        std::process::exit(0);
    }
}
