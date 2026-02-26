use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
};

use crate::utils::common::logger::*;

pub fn load_template(from: &str, to: &std::path::PathBuf) {
    const CARGO_CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    let from_path_name = format!("src/templates/{from}");
    let root_path = std::path::PathBuf::new().join(CARGO_CARGO_MANIFEST_DIR);

    let from_path = root_path.join(from_path_name);

    if std::fs::copy(from_path, to).is_err() {
        logger_error(format!("Error loading template {from}"));
    }

    logger_info(format!("  file {from} loaded"));
}

pub fn load_template_arg(from: &str, to: &std::path::PathBuf, name: &str) {
    const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    let from_path_name = format!("src/templates/{from}");
    let root_path = std::path::PathBuf::new().join(CARGO_MANIFEST_DIR);
    let from_path = root_path.join(from_path_name);

    // 2. LEER: En lugar de copy, leemos el contenido a una variable String
    let content_result = std::fs::read_to_string(&from_path);

    if let Ok(mut content) = content_result {
        let struct_name = capitalize_first(name);
        content = content.replace("generic", name);
        content = content.replace("GENERIC", &struct_name);

        if std::fs::write(to, content).is_err() {
            logger_error(format!(
                "Error writing template to destination {}",
                to.display()
            ));
        }

        logger_info(format!("  file {from} loaded as {struct_name}"));
    } else {
        logger_error(format!("Error read template content{from}"));
    }
}

pub fn overwrite_file(path: &std::path::PathBuf, content: &str) {
    if !path.exists() {
        logger_error(format!(
            "Error writing file, path {} does not exist",
            path.display()
        ));
    }
    let content_mod = std::fs::read_to_string(path);

    if content_mod.is_err() {
        logger_error(format!("Failure to read the content of {}", path.display()));
    }

    if let Ok(_content) = content_mod {
        let new_content = format!("{}\n{}", _content, content);
        if std::fs::write(path, new_content).is_err() {
            logger_error(format!("The file writing failed {}", path.display()));
        }

        logger_info(format!(
            "  File {} overwritten successfully",
            path.display()
        ));
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
}

pub fn modify_file(path: &std::path::PathBuf, origin_content: &str, modified_content: &str) {
    if let Ok(mut content) = std::fs::read_to_string(path) {
        content = content.replace(origin_content, modified_content);

        if std::fs::write(path, content).is_err() {
            logger_error(format!(
                "Error loading new content in file error {}",
                path.display()
            ));
        }
    } else {
        logger_error("Error rewriting the error handling file".to_string());
    }

    logger_info(format!("  File {} modified successfully", path.display()));
}

pub fn verify_content_on_file(path: &std::path::PathBuf, content: &str) -> bool {
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);

            for line in reader.lines() {
                let content_line = line.unwrap_or("".to_string());
                if content_line.contains(content) {
                    logger_warning(format!(
                        "  {} already exists in {}",
                        content,
                        path.display()
                    ));
                    return true;
                }
            }
            false
        }
        Err(_) => {
            logger_error(format!("  Error reading file in path {}", path.display()));
            false
        }
    }
}

pub fn concatenate_content(path: &PathBuf, mut content: String) {
    match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut old_content = String::new();
            if reader.read_to_string(&mut old_content).is_ok() {
                content.push_str(&old_content);
                logger_info(format!(
                    "  A content has been concatenated in .{}",
                    path.display()
                ));
            } else {
                logger_error(format!("Error concatenating content in {}", path.display()));
            }
        }
        Err(_) => {
            logger_error(format!("Error reading file in path {}", path.display()));
        }
    }

    match OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)
    {
        Ok(mut file) => {
            if file.write_all(content.as_bytes()).is_ok() {
                logger_info(format!("  File {} rewritten correctly", path.display()));
            } else {
                logger_error(format!("Error rewriting file {}", path.display()));
            }
        }
        Err(_) => {
            logger_error(format!("Error reading file in path {}", path.display()));
        }
    }
}

pub fn delete_file_content(path: &PathBuf) {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)
    {
        Ok(_) => {
            logger_warning(format!(
                "  File contents have been deleted {}",
                path.display()
            ));
        }
        Err(_) => {
            logger_error(format!("Error deleting file contents {}", path.display()));
        }
    }
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
