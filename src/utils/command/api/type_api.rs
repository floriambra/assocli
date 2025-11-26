use crate::utils::common::{create_dir::create_dir, file::load_template_arg};
use console::style;

pub struct NewModule {
    pub project_path: std::path::PathBuf,
    pub name_module: String,
}

impl NewModule {
    pub fn new(project_path: std::path::PathBuf, name_module: String) -> Self {
        Self {
            project_path,
            name_module,
        }
    }

    pub fn create_folder_module(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating module files,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        let path = self
            .project_path
            .join(format!("src/app/module/{}", self.name_module));

        if !path.exists() {
            create_dir(path);
        } else {
            eprintln!(
                "{}",
                style(format!(
                    "  Error creating directory, a module with the name already exists {}",
                    self.name_module
                ))
                .red()
                .bold()
            );
            std::process::exit(1)
        }
    }

    pub fn create_module_files(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating module files,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        let path_module = self
            .project_path
            .join(format!("src/app/module/{}", self.name_module));

        if !path_module.exists() {
            eprintln!(
                "{}",
                style("  Error, the module path does not exist")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        std::thread::sleep(std::time::Duration::from_secs(1));

        load_template_arg(
            "models.rs",
            path_module.join("models.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "repositories.rs",
            path_module.join("repositories.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "services.rs",
            path_module.join("services.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "handlers.rs",
            path_module.join("handlers.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "config_mod.rs",
            path_module.join("mod.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        //load_template("main.rs", path_module);

        println!("{}", style(" module created successfully").green().bold());
    }
}
