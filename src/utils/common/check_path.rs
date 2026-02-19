use std::path::Path;

use console::style;

pub fn check_project_path(path: &Path) -> bool {
    if !path.exists() {
        eprintln!("{}", style(" project does not exist.").red().bold());
        std::process::exit(1)
    }
    true
}

pub fn check_existing_module_path(path: &Path, name_module: &str) -> bool {
    if path.exists() {
        eprintln!(
            "{}",
            style(format!("  The module already exists {}", name_module))
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    true
}

pub fn check_module_path(path: &Path) -> bool {
    if !path.exists() {
        eprintln!(
            "{}",
            style(("  The module does not exists").to_string())
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    true
}

pub fn check_directory(path: &Path, name_directory: &str) -> bool {
    if path.exists() {
        eprintln!(
            "{}",
            style(format!(" The directory {} already exists", name_directory))
                .yellow()
                .bold()
        );
        return false;
    }

    true
}

pub fn check_directory_existing(path: &Path) -> bool {
    if !path.exists() {
        eprintln!(
            "{}",
            style(("  The directory does not exist {}").to_string())
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    true
}

pub fn check_file_existing(path: &Path) -> bool {
    if !path.exists() {
        eprintln!(
            "{}",
            style(("  The file does not exist {}").to_string())
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    true
}
