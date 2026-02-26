use std::{fs, path::PathBuf};

use crate::utils::common::logger::*;

pub fn create_file(path: &PathBuf, content: Option<&str>) {
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut name_file = "";

    if let Some(path) = path.to_str() {
        let list: Vec<&str> = path.split("/").collect();
        name_file = list.last().unwrap();
    }

    if path.exists() {
        logger_warning(format!(
            "  File {} It already exists, omitting....",
            name_file
        ));
        return;
    }

    if let Some(affair) = content {
        if fs::write(path, affair).is_err() {
            logger_error(format!("Error when writing {}", &path.display()));
        }
    } else if fs::File::create(path).is_err() {
        logger_error(format!("Error creating file {}", &path.display()));
    }

    logger_info(format!("  Created: {path:?}"));
}
