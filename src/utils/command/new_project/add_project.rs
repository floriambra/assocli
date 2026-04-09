use crate::utils::common::{
    add_dependency::add_dependency,
    check_path::{
        check_directory, check_directory_existing, check_file_existing, check_project_path_existing,
    },
    create_dir::create_dir,
    create_file::create_file,
    file::{load_template, overwrite_file},
    status_cargo::execute_cargo,
    logger::*,
};
use std::{path::PathBuf, thread};

#[derive(Debug, Clone)]
pub struct Project {
    pub home_path: PathBuf,
    pub name_project: String,
}

impl Project {
    pub fn new(project_path: PathBuf, name_project: &str) -> Self {
        Self {
            home_path: project_path,
            name_project: name_project.to_string(),
        }
    }

    pub fn create_project(&mut self) -> bool {
        logger_info("📁 Creating project...".to_string());

        if !check_project_path_existing(&self.home_path.join(&self.name_project)) {
            return false;
        }

        let current_dir_project: String = format!("{}", self.home_path.display());
        execute_cargo("new", Some(&self.name_project), current_dir_project)
        
    }

    pub fn create_actix(&self) {
        let project_path = format!("{}", self.home_path.join(&self.name_project).display());
        let cargo_toml_path = self.home_path.join(&self.name_project).join("Cargo.toml");

        logger_debug("🔍 Checking project Cargo...".to_string());
        check_file_existing(&cargo_toml_path);

        std::thread::sleep(std::time::Duration::from_secs(1));

        logger_debug("  Adding 'axum' to the project...".to_string());

        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("axum", Some("json"), &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("dotenvy", None, &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("serde", Some("derive"), &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("serde_json", None, &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("validator", Some("derive"), &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("tokio", Some("full"), &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("dotenvy", None, &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("tracing", None, &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency("tower-http", Some("fs"), &project_path);
        std::thread::sleep(std::time::Duration::from_secs(1));

        add_dependency(
            "tracing-subscriber",
            Some("env-filter,fmt,ansi"),
            &project_path,
        );
    }

    pub fn create_app_structure(&self) {
        let src_path = self.home_path.join(&self.name_project).join("src");
        let app_path = src_path.join("app");

        check_file_existing(&src_path);

        let subdirs = ["config", "module", "shared"];

        for directory in subdirs {
            thread::sleep(std::time::Duration::from_secs(1));
            let path = app_path.join(directory);
            let mod_rs_path = path.join("mod.rs");
            let mod_rs_path_str = mod_rs_path.to_str().unwrap();

            if check_directory(&path, directory) {
                create_dir(&path.to_path_buf());
            }

            create_file(&std::path::PathBuf::new().join(mod_rs_path_str), None);
        }

        let mod_file_path = app_path.join("mod.rs");
        const CONTENT: &str = "pub mod module;\npub mod shared;\npub mod config;\n";

        thread::sleep(std::time::Duration::from_secs(5));

        create_file(&mod_file_path, Some(CONTENT));

        let app_path_string = app_path.display().to_string();
        logger_info(format!(
            "  App structure created correctly in {}",
            app_path_string
        ));
    }

    pub fn create_mod_main(&self) {
        let path_directory_module = self
            .home_path
            .join(&self.name_project)
            .join("src/app/module");

        let path_module_main = path_directory_module.join("mod.rs");

        load_template("config_modules.rs", &path_module_main);

        logger_info("  Main module configuration created".to_string());
    }

    pub fn create_env_rs(&self) {
        let config_dir = self
            .home_path
            .join(&self.name_project)
            .join("src/app/config");
        let env_rs_path = config_dir.join("env.rs");
        let mod_rs_path = config_dir.join("mod.rs");

        const CONTENT: &str = "\npub mod env;";

        check_directory_existing(&config_dir);

        thread::sleep(std::time::Duration::from_secs(1));

        if check_file_existing(&mod_rs_path) {
            overwrite_file(&mod_rs_path, CONTENT);
        }

        load_template("env.rs", &env_rs_path);
    }

    pub fn create_files_common(&self) {
        const CONTENT: &str = "pub mod error;\npub mod validation;";

        let path_shared = self
            .home_path
            .join(&self.name_project)
            .join("src/app/shared");
        let path_common = path_shared.join("common");

        if check_directory(&path_common, "common") {
            create_dir(&path_common);

            load_template("error.rs", &path_common.join("error.rs"));

            load_template("validation.rs", &path_common.join("validation.rs"));

            create_file(&path_common.join("mod.rs"), Some(CONTENT));

            overwrite_file(&path_shared.join("mod.rs"), "pub mod common;\n");
        }
    }

    pub fn create_files_state(&self) {
        let path_shared = self
            .home_path
            .join(&self.name_project)
            .join("src/app/shared");
        let path_state = path_shared.join("state");

        if check_directory(&path_state, "state") {
            create_dir(&path_state);

            load_template("state.rs", &path_state.join("state.rs"));

            let path_mod_state = path_state.join("mod.rs");

            create_file(&path_mod_state, Some("pub mod state;\n"));

            overwrite_file(&path_shared.join("mod.rs"), "pub mod state;\n");
        }
    }

    pub fn create_env_file(&self) {
        const CONTENT: &str = r#"ADDRESS="0.0.0.0"
        PORT=3000
        "#;

        let env_path = self.home_path.join(&self.name_project).join(".env");

        thread::sleep(std::time::Duration::from_secs(1));

        if !env_path.exists() {
            create_file(&env_path, Some(CONTENT));
        }
    }

    pub fn create_main_rs(&self) {
        let main_path = self.home_path.join(&self.name_project).join("src/main.rs");

        if main_path.exists() {
            if std::fs::remove_file(&main_path).is_err() {
                logger_error("Error deleting/restoring the main file".to_string());
            }
        }

        thread::sleep(std::time::Duration::from_secs(1));

        load_template("main.rs", &main_path);

        logger_info("  main.rs created successfully".to_string());
    }

    pub fn add_root_template(&self) {
        let static_path = self
            .home_path
            .join(&self.name_project)
            .join("templates/static");

        thread::sleep(std::time::Duration::from_secs(1));

        create_dir(&static_path);

        load_template("static/index.html", &static_path.join("index.html"));
        load_template("static/styles.css", &static_path.join("style.css"));

        logger_info("  index.html in the root has been added".to_string());
    }
}
