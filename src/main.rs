use std::process::exit;
mod internal;

use internal::{add_files, check_git, get_commit_message, git_commit, git_push};

fn main() {
    println!("Starting Script!!!\n");
    println!("Doing automated backup on current working directory(cwd)\n");
    let git_status = check_git();
    if git_status == false {
        eprintln!("cwd is not a git repo, exiting!!\n");
        exit(1)
    }
    add_files();
    let commit_message: String = get_commit_message("Vault Backup");
    git_commit(&commit_message);
    //WARN: Oh no pushing into main
    //TODO: get branch name from user,
    git_push(Some("main"));
}
