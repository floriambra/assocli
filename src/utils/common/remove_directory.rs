use crate::utils::common::logger::{logger_error, logger_info};

pub fn delete_folder(path: &std::path::PathBuf, name_directory: &str) {
    let command = std::process::Command::new("rm")
        .arg("-rf")
        .arg(path)
        .status();

    if command.is_err() {
        logger_error(format!(
            "Command execution failed when trying to delete directory {}.",
            name_directory
        ));
    }

    logger_info(format!("  Directory {} deleted", name_directory));
}
