use std::process::Command;

use crate::utils::common::logger::*;

pub fn add_dependency(arg: &str, features: Option<&str>, path: &str) {
    logger_debug(format!("  Adding dependencies {arg}....."));
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut command = Command::new("cargo");

    command.arg("add").arg(arg);
    if let Some(value) = features {
        command.arg("--features").arg(value);
    }

    command.current_dir(path);

    let status = command.status();

    match status {
        Ok(status) if status.success() => {
            logger_info(format!("  {arg} successfully added!"));
        }
        Ok(_) => {
            logger_error(format!(
                "The installation failed {arg}. ¿you have installed 'cargo-edit'?"
            ));
        }
        Err(err) => {
            logger_error(format!("  Error executing 'cargo add {arg}': {err}"));
        }
    }
}
