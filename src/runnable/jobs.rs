use std::process::{Child, Command};

use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub struct JobInfo {
    pub job_id: usize,
    pub command: String,
    pub child: Child,
}

pub struct Jobs;

impl Jobs {
    pub fn run_background(&self, args: &Vec<String>, ctx: CommandContext) -> i32 {
        let cmd = &ctx.parsed_command.command;
        let p_out = &ctx.parsed_command.stdout;
        let p_err = &ctx.parsed_command.stderr;
        let job_list = ctx.job_list;

        let stdout = match output::output_to_stdio(p_out) {
            Ok(stdout) => stdout,
            Err(e) => {
                eprintln!("Error setting up stdout: {}", e);
                return 1;
            }
        };

        let stderr = match output::output_to_stdio(p_err) {
            Ok(stderr) => stderr,
            Err(e) => {
                eprintln!("Error setting up stderr: {}", e);
                return 1;
            }
        };

        let child = Command::new(cmd)
            .args(args)
            .stdout(stdout)
            .stderr(stderr)
            .spawn();

        if let Err(e) = child {
            eprintln!("Error executing background command {}: {}", cmd, e);
            return 1;
        }
        let cnt_bg = ctx.cnt_bg;
        let child_process = child.unwrap();
        let pid = child_process.id();
        let job_info = JobInfo {
            job_id: cnt_bg,
            command: format!("{} {}", cmd, args.join(" ")),
            child: child_process,
        };
        job_list.push(job_info);

        return output::write(format!("[{}] {}", cnt_bg, pid).as_str(), p_out);
    }
}

impl Runnable for Jobs {
    fn name(&self) -> String {
        "jobs".to_string()
    }

    fn run(&self, _args: &Vec<String>, ctx: CommandContext) -> i32 {
        let mut err_code = 0;
        ctx.job_list.retain_mut(|job| {
            let latest = match ctx.cnt_bg {
                id if id == job.job_id => "+",
                id if id - 1 == job.job_id => "-",
                _ => "",
            };

            let status = match job.child.try_wait() {
                Ok(Some(_)) => "Done",
                Ok(None) => "Running",
                Err(_) => "Error",
            };

            let trailing_background = if status == "Running" { " &" } else { "" };
            let content = format!(
                "[{}]{}  {:<24}{}{}",
                job.job_id, latest, status, job.command, trailing_background
            );
            let output_code = output::write(content.as_str(), &ctx.parsed_command.stdout);
            if output_code != 0 {
                eprintln!(
                    "Error outputting job status for job {}: {}",
                    job.job_id, content
                );
                err_code = output_code;
            }

            status == "Running"
        });

        err_code
    }
}
