use std::path::Path;

use crate::utils::common::logger::*;

pub fn check_project_path(path: &Path) -> bool {
    if !path.exists() {
        logger_error("Project does not exist.".to_string());
    }
    true
}

pub fn check_project_path_existing(path: &Path) -> bool {
    if path.exists() {
        logger_error(format!("The project already exists {}", path.display()));
    }
    true
}

pub fn check_existing_module_path(path: &Path, name_module: &str) -> bool {
    if path.exists() {
        logger_error(format!("The module already exists {}", name_module));
    }
    true
}

pub fn check_module_path(path: &Path) -> bool {
    if !path.exists() {
        logger_error("The module does not exists".to_string());
    }
    true
}

pub fn check_directory(path: &Path, name_directory: &str) -> bool {
    if path.exists() {
        logger_warning(format!(" The directory {} already exists", name_directory));
        return false;
    }
    true
}

pub fn check_directory_existing(path: &Path) -> bool {
    if !path.exists() {
        logger_error(format!("The directory {} does not exist", path.display()));
    }
    true
}

pub fn check_file_existing(path: &Path) -> bool {
    if !path.exists() {
        logger_error(format!("The file {} does not exist", path.display()));
    }
    true
}

pub fn check_file(path: &Path) -> bool {
    if path.exists() {
        logger_warning(format!("The file {} already exists", path.display()));
        return false;
    }
    true
}
