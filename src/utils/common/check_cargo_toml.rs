use crate::shared::global::PROJECT_PATH;
use crate::utils::common::logger::*;

pub fn check_toml_project(name_project: &str) -> bool {
    let path = PROJECT_PATH.as_deref();

    if let Some(_path) = path {
        let dir_project = _path.to_path_buf().join(&name_project);
        let cargo_toml = dir_project.join("Cargo.toml");

        if !cargo_toml.exists() {
            logger_error("Cargo.toml file is not present in the project".to_string())
        }

        true
    } else {
        logger_error("Error searching for project in team".to_string());
        false
    }
}
