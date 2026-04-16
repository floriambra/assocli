use crate::shared::global::PROJECT_PATH;
use crate::utils::common::logger::*;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process, thread,
};

pub fn check_toml_project(name_project: &PathBuf) -> bool {
    let path = PROJECT_PATH.as_deref();

    if let Some(_path) = path {
        let dir_project = _path.join(name_project);
        let cargo_toml = dir_project.join("Cargo.toml");

        if !cargo_toml.exists() {
            logger_error("Cargo.toml file is not present in the project".to_string());
            return false;
        }

        true
    } else {
        logger_error("Error searching for project in team".to_string());
        false
    }
}

pub fn execute_cargo(arg: &str, optional_arg: Option<&str>, name_project: String) -> bool {
    let path_project = PROJECT_PATH.as_deref();
    let mut directory_project = PathBuf::new();

    if let Some(path) = path_project {
        directory_project = directory_project.join(path).join(&name_project);
    } else {
        logger_error("Project path issues".to_string());
        return false;
    }

    thread::sleep(std::time::Duration::from_secs(1));

    let mut cmd = process::Command::new("cargo");
    cmd.arg(arg);

    if let Some(opt) = optional_arg {
        cmd.arg(opt);
    }

    // Stream native Cargo logs directly (stdout + stderr), without custom progress UI.
    cmd.current_dir(&directory_project)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            let stdout = child.stdout.take().expect("stdout must be in Piped mode");
            let stderr = child.stderr.take().expect("stderr must be in Piped mode");

            // Keep detailed stderr lines to print a useful error summary if cargo fails.
            let stderr_handle = thread::spawn(move || {
                let reader = BufReader::new(stderr);
                let mut collected_errors: Vec<String> = Vec::new();

                for line in reader.lines() {
                    match line {
                        Ok(text) => {
                            eprintln!("{text}");

                            let lower = text.to_lowercase();
                            if lower.contains("error")
                                || lower.contains("failed")
                                || lower.contains("could not compile")
                            {
                                collected_errors.push(text);
                            }
                        }
                        Err(_) => break,
                    }
                }

                collected_errors
            });

            // Stream stdout as-is so user sees native Cargo logs.
            let stdout_handle = thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(text) = line {
                        println!("{text}");
                    } else {
                        break;
                    }
                }
            });

            let exit_status = child.wait().expect("Failed waiting cargo process");

            // Ensure stream threads finish before evaluating final result.
            let _ = stdout_handle.join();

            let collected_errors = match stderr_handle.join() {
                Ok(lines) => lines,
                Err(_) => vec!["Cargo stderr reader thread failed".to_string()],
            };

            if exit_status.success() {
                logger_info("  Compilation completed successfully".to_string());
                true
            } else {
                if collected_errors.is_empty() {
                    logger_error("Compilation failed. Cargo returned an error status.".to_string());
                } else {
                    logger_error(format!(
                        "Compilation failed. Cargo error details:\n{}",
                        collected_errors.join("\n")
                    ));
                }
                false
            }
        }
        Err(e) => {
            logger_error(format!("Error al iniciar cargo: {}", e));
            false
        }
    }
}
