use crate::internal::Status;

use std::process::Command;


pub fn run_command(command: &str, args: &[&str]) -> Status {
    let result = Command::new(command).args(args).status();

    return match result {
        Ok(_) => Status::Success,
        Err(_) => Status::Failure("error".to_string()),
    };
}

