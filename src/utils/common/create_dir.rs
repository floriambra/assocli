use crate::utils::common::logger::*;

pub fn create_dir(path: &std::path::PathBuf) {
    if std::fs::create_dir_all(path).is_err() {
        logger_error(format!("Error creating directory {}", &path.display()));
    }

    logger_info(format!("  Created {} directory", &path.display()));
}
