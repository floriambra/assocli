use crate::utils::common::{
    add_dependency::add_dependency, create_dir::create_dir, create_file::create_file,
    file::load_template,
};

use console::style;

#[derive(Debug, Clone)]
pub struct NewProject {
    pub path: std::path::PathBuf,
    pub project_path: std::path::PathBuf,
}

impl NewProject {
    pub fn new(path: std::path::PathBuf, project_path: std::path::PathBuf) -> Self {
        Self { path, project_path }
    }

    pub fn create_project(&mut self, name: &str) -> bool {
        println!("📁 Creating project...");

        let path = self.path.to_str().unwrap_or("");
        let path_project = format!("{path}/{name}");

        if std::path::Path::new(&path_project).exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  The project '{name}' already exists in '{path}'"
                ))
                .yellow()
                .bold()
            );
            return false;
        }

        let status = std::process::Command::new("cargo")
            .arg("new")
            .arg(name)
            .current_dir(path)
            .status();

        if let Err(err) = &status {
            eprintln!(
                "{}",
                style(format!("  Error executing cargo new: {err}"))
                    .red()
                    .bold()
            );
            return false;
        }

        self.project_path = std::path::PathBuf::from(&path_project);

        true
    }

    pub fn create_actix(&self) {
        let cargo_toml_path = self.project_path.join("Cargo.toml");

        if let Some(project_path) = self.project_path.to_str() {
            if !cargo_toml_path.exists() {
                eprintln!(
                    "{}",
                    style(format!("  'Cargo.toml' was bot found in '{project_path}'",))
                        .red()
                        .bold()
                );
                std::process::exit(1)
            }

            println!("{}", style("🔍 Checking project Cargo...").cyan().bold());

            std::thread::sleep(std::time::Duration::from_secs(1));

            println!(
                "{}",
                style("  Adding 'axum' to the project...").blue().bold()
            );

            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("axum", Some("json"), project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("dotenvy", None, project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("serde", Some("derive"), project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("serde_json", None, project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("validator", Some("derive"), project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("tokio", Some("full"), project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("dotenvy", None, project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("tracing", None, project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));

            add_dependency(
                "tracing-subscriber",
                Some("env-filter,fmt,ansi"),
                project_path,
            );
        } else {
            eprintln!(
                "{}",
                style("  Error creating axum, problems with the project path.",)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    }

    pub fn create_app_structure(&self) {
        let src_path = self.project_path.join("src");
        let app_path = src_path.join("app");

        if let Some(project_path) = self.project_path.to_str() {
            if !src_path.exists() {
                eprintln!(
                    "{}",
                    style(format!(
                        "  The src directory was not found in '{project_path}'",
                    ))
                    .red()
                    .bold()
                );
                std::process::exit(1);
            }

            let subdirs = ["config", "module", "shared"];

            for dir in subdirs {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let path = app_path.join(dir);
                let mod_rs_path = path.join("mod.rs");
                let mod_rs_path_str = mod_rs_path.to_str().unwrap();

                if !path.exists() {
                    create_dir(path);
                } else {
                    println!(
                        "{}",
                        style(format!("  It already exists: {}", path.display()))
                            .yellow()
                            .bold()
                    );
                }

                create_file(std::path::PathBuf::new().join(mod_rs_path_str), None);
            }

            let mod_file_path = app_path.join("mod.rs");
            const CONTENT: &str = "pub mod module;\npub mod shared;\npub mod config;\n";

            std::thread::sleep(std::time::Duration::from_secs(1));

            let common_directory = app_path.join("shared/common");

            create_dir(common_directory);

            create_file(mod_file_path, Some(CONTENT));

            println!(
                "{}",
                style(format!(
                    "  App structure created correctly in {}'",
                    app_path.display()
                ))
                .cyan()
                .bold()
            );
        } else {
            eprintln!(
                "{}",
                style("  Error creating app structure, problems with the project path.",)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    }

    pub fn create_env_rs(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating env rs,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        let config_dir = self.project_path.join("src/app/config");
        let env_rs_path = config_dir.join("env.rs");
        let mod_rs_path = config_dir.join("mod.rs");

        const CONTENT: &str = "\npub mod env;";

        if !config_dir.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  The config folder was not found in {config_dir:?}"
                ))
                .red()
                .bold()
            );
            std::process::exit(1)
        }
        std::thread::sleep(std::time::Duration::from_secs(1));

        if mod_rs_path.exists() {
            create_file(mod_rs_path, Some(CONTENT));
        } else {
            eprintln!(
                "{}",
                style("  Error: mod.rs path does not exist").red().bold()
            );
            std::process::exit(1)
        }

        load_template("env.rs", env_rs_path);
    }

    pub fn create_files_common(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating files common,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }
        const CONTENT: &str = "pub mod error;\npub mod validation;";

        let path_common = self.project_path.join("src/app/shared/common");

        load_template("error.rs", path_common.join("error.rs"));

        load_template("validation.rs", path_common.join("validation.rs"));

        create_file(path_common.join("mod.rs"), Some(CONTENT));
    }

    pub fn create_env_file(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating env file,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        const CONTENT: &str = r#"ADDRESS="127.0.0.1"
        PORT=3000
        "#;

        let env_path = self.project_path.join(".env");

        std::thread::sleep(std::time::Duration::from_secs(1));

        if env_path.exists() {
            println!(
                "{}",
                style("  .env already exists, omitting...").yellow().bold()
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
            std::process::exit(1)
        }

        create_file(env_path, Some(CONTENT));
    }

    pub fn create_main_rs(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating main.rs file,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        let main_path = self.project_path.join("src/main.rs");

        if main_path.exists() {
            let _ = std::fs::remove_file(&main_path);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template("main.rs", main_path);

        println!(
            "{}",
            style("  main.rs created successfully").green().bold()
        );
    }
}
