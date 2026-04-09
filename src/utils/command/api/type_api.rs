use crate::utils::common::{
    check_path::*, create_dir::create_dir, file::load_template_arg, logger::*,
    remove_directory::delete_folder,
};
use std::{fs, path::PathBuf, thread};

pub struct Module {
    pub module_path: PathBuf,
    pub project_path: PathBuf,
    pub name_module: String,
}

impl Module {
    pub fn new(module_path: PathBuf, project_path: PathBuf, name_module: String) -> Self {
        Self {
            module_path,
            project_path,
            name_module,
        }
    }

    pub fn create_folder_module(&self) {
        check_project_path(&self.project_path);
        check_existing_module_path(&self.module_path, &self.name_module);
        create_dir(&self.module_path);
    }

    pub fn create_module_files(&self) {
        check_module_path(&self.module_path);
        thread::sleep(std::time::Duration::from_secs(1));

        load_template_arg(
            "models.rs",
            &self.module_path.join("models.rs"),
            &self.name_module,
        );
        thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "repositories.rs",
            &self.module_path.join("repositories.rs"),
            &self.name_module,
        );
        thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "services.rs",
            &self.module_path.join("services.rs"),
            &self.name_module,
        );
        thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "handlers.rs",
            &self.module_path.join("handlers.rs"),
            &self.name_module,
        );
        thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "config_mod.rs",
            &self.module_path.join("mod.rs"),
            &self.name_module,
        );
        thread::sleep(std::time::Duration::from_secs(1));
        //load_template("main.rs", path_module);

        logger_debug(" module created successfully".to_string());

        self.inject_module_main();
    }

    fn inject_module_main(&self) {
        check_module_path(&self.module_path);

        logger_info(format!(
            "  Configuring the implementation of the {} module...",
            &self.name_module
        ));

        thread::sleep(std::time::Duration::from_secs(2));

        let path_module_main = PathBuf::new().join(format!(
            "{}/src/app/module/mod.rs",
            &self.project_path.display()
        ));

        let content_new_module: (String, String) = (
            format!(
                "Router::new()\n\t\t.nest(\"/{}\", {{
            tracing::info!(\"  ├─ 󰕳 Starting module configuration {}....\");
            {}::configure(std::sync::Arc::clone(&_state))
        }})",
                self.name_module, self.name_module, self.name_module
            ),
            format!("mod {};\nuse axum::Router;", self.name_module),
        );

        let content_module_main = fs::read_to_string(&path_module_main);

        if let Ok(mut content) = content_module_main {
            if content.contains(&self.name_module) {
                delete_folder(&self.project_path.join("src/app/module"), &self.name_module);
                logger_error(format!(
                    "The module files were deleted, but it is still configured in the {}.",
                    self.project_path.join("src/app/module/mod.rs").display()
                ));
            }

            content = content.replace("Router::new()", &content_new_module.0);

            content = content.replace("use axum::Router;", &content_new_module.1);

            if fs::write(&path_module_main, content).is_err() {
                logger_error(format!(
                    "Error loading new content in main module {}",
                    path_module_main.display()
                ));
            }
        } else {
            logger_error("Error injecting module into main configuration".to_string());
        }
    }
}
