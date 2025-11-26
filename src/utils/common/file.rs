use console::style;

pub fn load_template(from: &str, to: std::path::PathBuf) {
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

    println!(
        "{}",
        style(format!("  Template: {from} loaded")).green().bold()
    );
}

pub fn load_template_arg(from: &str, to: std::path::PathBuf, name: &str) {
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

        if std::fs::write(&to, content).is_err() {
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
            style(format!("  Template: {from} loaded as {struct_name}"))
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

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
