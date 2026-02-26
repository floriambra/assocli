use crate::{
    shared::global::PROJECT_PATH,
    utils::{
        command::{api::type_api::Module, config::config_sqlx::Sqlx},
        common::selection_type::{choose_types_relational_bases, choose_your_configuration_type},
    },
};

use crate::utils::common::logger::*;

pub fn handler_config(name_module: &str, name_project: &str) {
    let mut path_project = std::path::PathBuf::new();
    let mut path_module = std::path::PathBuf::new();

    if let Some(_path) = PROJECT_PATH.as_deref() {
        path_project = std::path::PathBuf::from(_path).join(name_project);
        path_module = path_project.join("src/app/module").join(name_module);

        println!("{}", path_module.display());
        if !path_project.exists() {
            logger_error(format!("The project {} does not exist", name_project))
        }

        if !path_module.exists() {
            logger_error(format!(
                "The module {} does not exist in project {}",
                name_project, name_module
            ));
        }
    } else {
        logger_error("Error creating project path to query".to_string());
    }

    match choose_your_configuration_type(name_module) {
        "sqlx" => {
            let module: Module = Module {
                module_path: path_module,
                project_path: path_project,
                name_module: name_module.to_string(),
            };

            let mut database_engine = String::new();
            match choose_types_relational_bases() {
                "postgres" => {
                    database_engine.push_str("postgres");
                    logger_debug("\n Configuring module for sqlx and postgres....\n".to_string());
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let configuring_sqlx_postgres: Sqlx = Sqlx::new(module, database_engine);
                    configuring_sqlx_postgres.create_folder_database();
                    configuring_sqlx_postgres.create_configuration_files();
                    configuring_sqlx_postgres.inject_dependencies();
                    configuring_sqlx_postgres.adding_environment_variables();
                    configuring_sqlx_postgres.adding_database_module();
                    configuring_sqlx_postgres.modify_global_status();
                    configuring_sqlx_postgres.modify_main();
                    configuring_sqlx_postgres.modify_module();

                    std::thread::sleep(std::time::Duration::from_secs(1));
                    logger_info("\n 󰪩 Module configured to work with sqlx and postgres.\n\n󰪩 Use Asso docker compose to lift development container for postgres.".to_string());
                }
                "mariadb" => {
                    println!("Estamos en mariadb")
                }
                _ => logger_error("Database engine has not been currently mapped...".to_string()),
            }
        }
        _ => logger_error("The settings have not been currently assigned.....".to_string()),
    }
}
