use std::process::Command;

use crate::runnable::{CommandContext, Runnable};
use crate::utils::output;

pub enum JobStatus {
    Running,
    Stopped,
    Done,
}

pub struct JobInfo {
    pub job_id: usize,
    pub status: JobStatus,
    pub command: String,
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
        let pid = child.unwrap().id();
        let job_info = JobInfo {
            job_id: cnt_bg,
            status: JobStatus::Running,
            command: format!("{} {} &", cmd, args.join(" ")),
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
        for job in ctx.job_list.iter() {
            let latest = if ctx.cnt_bg == job.job_id {
                "+"
            } else {
                ""
            };
            let status = match job.status {
                JobStatus::Running => "Running",
                JobStatus::Stopped => "Stopped",
                JobStatus::Done => "Done",
            };
            let content = format!("[{}]{}  {:<24}{}", job.job_id, latest, status, job.command);
            output::write(content.as_str(), &ctx.parsed_command.stdout);
        }

        0
    }
}
