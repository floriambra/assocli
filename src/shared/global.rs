use crate::utils::common::{
    check_path::check_directory,
    create_dir::create_dir,
    logger::{logger_error, logger_info},
};
use once_cell::sync::Lazy;
use std::{env::var, path::PathBuf};

pub static PROJECT_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let var_home_path = var("HOME");

    if let Ok(name_path) = var_home_path {
        let main_project_path = PathBuf::new().join(&name_path).join("Asso");
        Some(main_project_path)
    } else {
        logger_error("Error in home enviroment variable".to_string());
        None
    }
});
