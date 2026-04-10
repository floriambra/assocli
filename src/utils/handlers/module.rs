use crate::{
    shared::global::PROJECT_PATH,
    utils::{
        command::{api::type_api::Module, template::type_template::Template},
        common::{check_path::check_project_path, logger::*, selection_type::choose_module_type},
    },
};
use std::path::PathBuf;

pub fn handler_module(name_module: &str, name_project: &str) {
    let mut path_project = PathBuf::new();

    if let Some(path) = PROJECT_PATH.as_deref() {
        path_project = path_project.join(path).join(name_project);
    } else {
        logger_error("Error creating project path to query".to_string());
    }

    check_project_path(&path_project);

    match choose_module_type(name_module) {
        "API" => {
            let new_module: Module = Module::new(
                path_project
                    .clone()
                    .join(format!("src/app/module/{}", name_module)),
                path_project.clone(),
                name_module.to_string(),
            );

            new_module.create_folder_module();
            new_module.create_module_files();
        }
        "Template" => {
            let new_module_template: Template = Template::new(
                path_project
                    .clone()
                    .join(format!("src/app/module/{}", name_module)),
                path_project.clone(),
                name_module.to_string(),
            );

            new_module_template.add_dependency_tera();
            new_module_template.create_folder_module();
            new_module_template.create_dir_templates();
            new_module_template.load_templates_files();
            new_module_template.create_module_files();
        }
        _ => println!("La opcion no existe, intente de nuevo..."),
    }

    /*
    *
    fn handle_api_module(name: &str) {
    println!("🔧 Generating API module: {}", name);
    }

    fn handle_template_module(name: &str) {
    println!("🧩 Generating Template module: {}", name);
    }

    */
}
