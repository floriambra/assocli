use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
};

use console::style;

pub fn load_template(from: &str, to: &std::path::PathBuf) {
    const CARGO_CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    let from_path_name = format!("src/templates/{from}");
    let root_path = std::path::PathBuf::new().join(CARGO_CARGO_MANIFEST_DIR);

    let from_path = root_path.join(from_path_name);

    if std::fs::copy(from_path, to).is_err() {
        eprintln!(
            "{}",
            style(format!("  Error loading template {from}"))
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    println!("{}", style(format!("  file {from} loaded")).green().bold());
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
            eprintln!(
                "{}",
                style(format!(
                    "  Error writing template to destination {}",
                    to.display()
                ))
                .red()
                .bold()
            );
            std::process::exit(1);
        }

        println!(
            "{}",
            style(format!("  file {from} loaded as {struct_name}"))
                .green()
                .bold()
        );
    } else {
        eprintln!(
            "{}",
            style(format!("  Error read template content{from}"))
                .red()
                .bold()
        );
        std::process::exit(1);
    }
}

pub fn overwrite_file(path: &std::path::PathBuf, content: &str) {
    if !path.exists() {
        eprintln!(
            "{}",
            style(format!(
                "  Error writing file, path {} does not exist",
                path.display()
            ))
            .red()
            .bold()
        );
        std::process::exit(1)
    }
    let content_mod = std::fs::read_to_string(path);

    if content_mod.is_err() {
        eprintln!(
            "{}",
            style(format!(
                "  Failure to read the content of {}",
                path.display()
            ))
            .red()
            .bold()
        );
        std::process::exit(1)
    }

    if let Ok(_content) = content_mod {
        let new_content = format!("{}\n{}", _content, content);
        if std::fs::write(path, new_content).is_err() {
            eprintln!(
                "{}",
                style(format!("  The file writing failed {}", path.display()))
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        println!(
            "{}",
            style(format!(
                "  File {} overwritten successfully",
                path.display()
            ))
            .green()
            .bold()
        );
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
}

pub fn modify_file(path: &std::path::PathBuf, origin_content: &str, modified_content: &str) {
    if let Ok(mut content) = std::fs::read_to_string(path) {
        content = content.replace(origin_content, modified_content);

        if std::fs::write(path, content).is_err() {
            eprintln!(
                "{}",
                style(format!(
                    "  Error loading new content in file error {}",
                    path.display()
                ))
                .red()
                .bold()
            );
            std::process::exit(1);
        }
    } else {
        eprintln!(
            "{}",
            style("  Error rewriting the error handling file")
                .red()
                .bold()
        );

        std::process::exit(1)
    }

    println!(
        "{}",
        style(format!("  File {} modified successfully", path.display()))
            .green()
            .bold()
    );
}

pub fn verify_content_on_file(path: &std::path::PathBuf, content: &str) -> bool {
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);

            for line in reader.lines() {
                let content_line = line.unwrap_or("".to_string());
                if content_line.contains(content) {
                    println!(
                        "{}",
                        style(format!(
                            "  {} already exists in {}",
                            content,
                            path.display()
                        ))
                        .yellow()
                        .bold()
                    );

                    return true;
                }
            }

            false
        }
        Err(_) => {
            eprintln!(
                "{}",
                style(format!("  Error reading file in path {}", path.display()))
                    .red()
                    .bold()
            );

            std::process::exit(1)
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
                println!(
                    "{}",
                    style(format!(
                        "  A content has been concatenated in .{}",
                        path.display()
                    ))
                    .green()
                    .bold()
                );
            } else {
                eprintln!(
                    "{}",
                    style(format!(
                        "  Error concatenating content in {}",
                        path.display()
                    ))
                    .red()
                    .bold()
                );

                std::process::exit(1)
            }
        }
        Err(_) => {
            eprintln!(
                "{}",
                style(format!("  Error reading file in path {}", path.display()))
                    .red()
                    .bold()
            );

            std::process::exit(1)
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
                println!(
                    "{}",
                    style(format!("  File {} rewritten correctly", path.display()))
                        .green()
                        .bold()
                );
            } else {
                eprintln!(
                    "{}",
                    style(format!("  Error rewriting file {}", path.display()))
                        .red()
                        .bold()
                );

                std::process::exit(1)
            }
        }
        Err(_) => {
            eprintln!(
                "{}",
                style(format!("  Error reading file in path {}", path.display()))
                    .red()
                    .bold()
            );

            std::process::exit(1)
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
            println!(
                "{}",
                style(format!(
                    "  File contents have been deleted {}",
                    path.display()
                ))
                .yellow()
                .bold()
            );
        }
        Err(_) => {
            eprintln!(
                "{}",
                style(format!(
                    "  Error deleting file contents {}",
                    path.display()
                ))
                .red()
                .bold()
            );

            std::process::exit(1)
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
