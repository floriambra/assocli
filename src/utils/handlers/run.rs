use crate::{
    shared::global::PROJECT_PATH,
    utils::common::{logger::*,status_cargo::check_toml_project},
};
use std::thread;

pub fn handler_run(name_project: &str) {
    run_cargo_command("run", None, name_project.to_string());
}

fn run_cargo_command(arg: &str, optional_arg: Option<&str>, name_project: String) {
    let path = PROJECT_PATH.as_deref();

    if let Some(_path) = path {
        let path_project = _path.to_path_buf().join(&name_project);
        let cargo_toml = path_project.join("Cargo.toml");

        
        check_toml_project(&cargo_toml);

        thread::sleep(std::time::Duration::from_secs(1));

        let mut output = std::process::Command::new("cargo");

        output.arg(arg);

        if let Some(arg) = optional_arg {
            output.arg(arg);
        }

        let command_result = output.current_dir(&path_project).status();

        if let Ok(_status) = command_result {
            if !_status.success() {
                logger_warning(_status.to_string());
            }
        } else {
            logger_error("Error trying to start the process".to_string());
        }
    } else {
        logger_error("Error searching for project path".to_string());
    }
}
