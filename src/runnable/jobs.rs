use std::process::{Child, Command};

use crate::runnable::{CommandContext, Runnable};
use crate::structures::dll::HasId;
use crate::utils::output;

pub struct JobInfo {
    pub job_id: usize,
    pub command: String,
    pub child: Child,
}

impl HasId for JobInfo {
    type Id = usize;

    fn id(&self) -> Self::Id {
        self.job_id
    }
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
        job_list.push_back(job_info);

        return output::write(format!("[{}] {}", cnt_bg, pid).as_str(), p_out);
    }
}

impl Runnable for Jobs {
    fn name(&self) -> String {
        "jobs".to_string()
    }

    fn run(&self, _args: &Vec<String>, ctx: CommandContext) -> i32 {
        let ll_jobs = ctx.job_list;
        let latest_job_id = ll_jobs.tail
            .and_then(|idx| ll_jobs.get_node(&idx))
            .map(|node| node.value.job_id);
        let second_latest_job_id = ll_jobs
            .tail
            .and_then(|idx| ll_jobs.get_node(&idx))
            .and_then(|node| node.prev)
            .and_then(|prev_idx| ll_jobs.get_node(&prev_idx))
            .map(|node| node.value.job_id);

        let mut head = ll_jobs.head;

        while let Some(idx) = head {
            let cur_node = ll_jobs.get_node_mut(&idx).unwrap();
            let next_idx = cur_node.next;
            let job = &mut cur_node.value;

            let status = match job.child.try_wait() {
                Ok(Some(_)) => "Done",
                Ok(None) => "Running",
                Err(_) => "Error",
            };

            let mut latest = "";
            if let Some(latest_job_id) = latest_job_id {
                if latest_job_id == job.job_id {
                    latest = "+";
                }
                if let Some(second_latest_job_id) = second_latest_job_id {
                    if second_latest_job_id == job.job_id {
                        latest = "-";
                    }
                }
            }

            let trailing_background = if status == "Running" { " &" } else { "" };
            let content = format!(
                "[{}]{}  {:<24}{}{}",
                job.job_id, latest, status, job.command, trailing_background
            );
            let err_code = output::write(content.as_str(), &ctx.parsed_command.stdout);
            if err_code != 0 {
                return err_code;
            }

            if status != "Running" {
                ll_jobs.remove(&idx);
            }

            head = next_idx;
        }
        0
    }
}
