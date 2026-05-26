use std::process::{Child, Command};

use crate::runnable::{ExecContext, JobList, RunResult, Runnable};
use crate::structures::dll::HasId;
use crate::utils::io;

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
    // Too few to use heap
    pub fn get_new_id(job_list: &JobList) -> usize {
        let mut id = 1;
        while job_list.get_node(&id).is_some() {
            id += 1;
        }
        id
    }
    pub fn run_background(&self, ctx: ExecContext) -> RunResult {
        let cmd = &ctx.own_parsed_command.command;
        let p_out = &ctx.own_parsed_command.stdout;
        let p_err = &ctx.own_parsed_command.stderr;
        let job_list = &mut *ctx.shell_ctx.job_list;
        let args = &ctx.own_parsed_command.args;

        let stdout = match io::output_to_stdio(p_out) {
            Ok(stdout) => stdout,
            Err(e) => {
                eprintln!("Error setting up stdout: {}", e);
                return RunResult::exit(1);
            }
        };

        let stderr = match io::output_to_stdio(p_err) {
            Ok(stderr) => stderr,
            Err(e) => {
                eprintln!("Error setting up stderr: {}", e);
                return RunResult::exit(1);
            }
        };

        let child = Command::new(cmd)
            .args(args)
            .stdout(stdout)
            .stderr(stderr)
            .spawn();

        if let Err(e) = child {
            eprintln!("Error executing background command {}: {}", cmd, e);
            return RunResult::exit(1);
        }
        let child_process = child.unwrap();
        let pid = child_process.id();
        let job_id = Self::get_new_id(job_list);
        let job_info = JobInfo {
            job_id: job_id,
            command: format!("{} {}", cmd, args.join(" ")),
            child: child_process,
        };
        job_list.push_back(job_info);

        io::write(format!("[{}] {}", job_id, pid).as_str(), p_out)
    }

    pub fn get_latest_job_id(job_list: &JobList) -> Option<usize> {
        job_list
            .tail
            .and_then(|idx| job_list.get_node(&idx))
            .map(|node| node.value.job_id)
    }

    pub fn get_second_latest_job_id(job_list: &JobList) -> Option<usize> {
        job_list
            .tail
            .and_then(|idx| job_list.get_node(&idx))
            .and_then(|node| node.prev)
            .and_then(|prev_idx| job_list.get_node(&prev_idx))
            .map(|node| node.value.job_id)
    }

    pub fn get_job_display(
        job: &mut JobInfo,
        latest_job_id: Option<usize>,
        second_latest_job_id: Option<usize>,
    ) -> String {
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

        content
    }

    pub fn reap_jobs(job_list: &mut JobList) -> Vec<String> {
        let mut removed_contents = Vec::new();
        let latest_job_id = Jobs::get_latest_job_id(job_list);
        let second_latest_job_id = Jobs::get_second_latest_job_id(job_list);
        
        for idx in job_list.ids() {
            let cur_node = job_list.get_node_mut(&idx).unwrap();
            let job = &mut cur_node.value;

            if let Ok(Some(_)) = job.child.try_wait() {
                let content = Self::get_job_display(job, latest_job_id, second_latest_job_id);
                removed_contents.push(content);
                job_list.remove(&idx);
            }
        }

        removed_contents
    }
}

impl Runnable for Jobs {
    fn name(&self) -> String {
        "jobs".to_string()
    }

    fn run(&self, ctx: ExecContext) -> RunResult {
        let ll_jobs = &mut *ctx.shell_ctx.job_list;
        let stdout = &ctx.own_parsed_command.stdout;

        let latest_job_id = Jobs::get_latest_job_id(ll_jobs);
        let second_latest_job_id = Jobs::get_second_latest_job_id(ll_jobs);

        for idx in ll_jobs.ids() {
            let cur_node = ll_jobs.get_node_mut(&idx).unwrap();
            let job = &mut cur_node.value;

            let content = Jobs::get_job_display(job, latest_job_id, second_latest_job_id);

            let err_code = io::write(content.as_str(), stdout);
            if err_code.exit_code != 0 {
                return err_code;
            }
        }
        Self::reap_jobs(ll_jobs);
        RunResult::exit(0)
    }
}
