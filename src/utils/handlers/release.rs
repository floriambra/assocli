use crate::shared::global::PROJECT_PATH;
use crate::utils::common::check_path::check_file_existing;
use crate::utils::common::{
    clear_terminal::clear_terminal, logger::*, status_cargo::execute_cargo,
};
use std::{fs, path::PathBuf, thread};

pub fn handler_release(name_project: &str) {
    let path_project = PROJECT_PATH.as_deref();

    if path_project.is_none() {
        logger_error("Error searching for project path".to_string());
    }

    let directory_project = path_project.unwrap().join(name_project);
    execute_cargo("build", Some("--release"), name_project.to_string());
    lift_release_service(directory_project.join("Cargo.toml"), directory_project);
}

pub fn lift_release_service(project_cargo_toml_path: PathBuf, project_path: PathBuf) {
    if let Ok(cargo_toml_content) = fs::read_to_string(&project_cargo_toml_path) {
        let project_name = cargo_toml_content
            .lines()
            .find(|line| line.trim_start().starts_with("name"))
            .and_then(|line| line.split('=').nth(1))
            .map(|name| name.trim().trim_matches('"').to_string());

        if project_name.is_none() {
            logger_error("The project name could not be determined from Cargo.toml".to_string());
        };

        thread::sleep(std::time::Duration::from_secs(3));
        clear_terminal();
        logger_debug(format!(
            "󰍦  Starting project service: {}",
            project_name.clone().unwrap()
        ));

        let binary_path = project_path
            .join("target")
            .join("release")
            .join(project_name.clone().unwrap());

        check_file_existing(&binary_path);

        thread::sleep(std::time::Duration::from_secs(1));

        let child = std::process::Command::new(&binary_path)
            .current_dir(&project_path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn();

        if let Ok(mut _child) = child {
            logger_debug(format!(
                "  Service '{}' running (Ctrl+C to stop)",
                &project_name.unwrap()
            ));

            let status = _child.wait();

            if status.is_err() {
                logger_error("Failed to wait on child process".to_string());
            }
        } else {
            logger_error(format!(
                "  Error starting service {}",
                project_name.unwrap()
            ));
        }
    } else {
        logger_error("Error reading content from Cargo.toml".to_string());
    }
}
