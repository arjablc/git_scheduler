mod command;
mod date_time;
mod status;

use std::io::Write;

use command::run_command;
use date_time::get_local_time;
use status::Status;

pub fn get_commit_message(message: &str) -> String {
    let time = get_local_time();
    format!("{message}: {time}")
}

pub fn check_git() -> bool {
    print!("Checking git on cwd\n");
    let res: Status = run_command("git", &["rev-parse", "--is-inside-work-tree"]);
    match res {
        Status::Success => true,
        Status::Failure(v) => {
            eprintln!("Git check failed: {v}\n");
            false
        }
    }
}

pub fn add_files() -> bool {
    print!("Adding files\n");
    let res: Status = run_command("git", &["add", "."]);
    match res {
        Status::Success => return true,
        Status::Failure(v) => {
            eprint!("Git add failed: {v}\n");
            return false;
        }
    }
}

pub fn git_commit(message: &str) -> bool {
    print!("Committing changes!!\n");
    let res: Status = run_command("git", &["commit", "-m", message]);
    match res {
        Status::Success => return true,
        Status::Failure(v) => {
            eprint!("Git commit failed: {v}\n");
            return false;
        }
    }
}
pub fn git_push(branch: Option<&str>) -> bool {
    use std::io::stdout;
    print!("Pushing changes!!\n");
    let _ = stdout().flush();
    let branch_arg: &str = match branch {
        Some(v) => v,
        None => "",
    };
    let res: Status = run_command("git", &["push", "origin", &branch_arg]);
    match res {
        Status::Success => return true,
        Status::Failure(v) => {
            eprint!("Git Push failed: {v}\n");
            return false;
        }
    }
}
