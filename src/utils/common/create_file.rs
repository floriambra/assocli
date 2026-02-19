use console::style;
use std::{fs, path::PathBuf};

pub fn create_file(path: &PathBuf, content: Option<&str>) {
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut name_file = "";

    if let Some(path) = path.to_str() {
        let list: Vec<&str> = path.split("/").collect();
        name_file = list.last().unwrap();
    }

    if path.exists() {
        println!(
            "{}",
            style(format!(
                "  File {} It already exists, omitting....",
                name_file
            ))
            .yellow()
            .bold()
        );
        return;
    }

    if let Some(affair) = content {
        if fs::write(&path, affair).is_err() {
            eprintln!(
                "{}",
                style(format!("  Error when writing {}", &path.display()))
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }
    } else if fs::File::create(&path).is_err() {
        eprintln!(
            "{}",
            style(format!("  Error creating file {}", &path.display()))
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    println!("{}", style(format!("  Created: {path:?}")).green().bold());
}
