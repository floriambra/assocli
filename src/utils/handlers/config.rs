use crate::{
    shared::global::PROJECT_PATH,
    utils::{
        command::{api::type_api::Module, config::config_sqlx::Sqlx},
        common::selection_type::{choose_types_relational_bases, choose_your_configuration_type},
    },
};

use crate::utils::common::{check_path::*, logger::*};
use std::{path::PathBuf, thread, time};

pub fn handler_config(name_module: &str, name_project: &str) {
    let mut path_project = PathBuf::new();
    let mut path_module = PathBuf::new();

    if let Some(_path) = PROJECT_PATH.as_deref() {
        path_project = PathBuf::from(_path).join(name_project);
        path_module = path_project.join("src/app/module").join(name_module);

        check_project_path(&path_project);
        check_module_path(&path_module);
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
                    configuring_sqlx_postgres.modify_module();
                    configuring_sqlx_postgres.create_configuration_database();
                    

                    thread::sleep(time::Duration::from_secs(1));
                    logger_info("\n 󰪩 Module configured to work with sqlx and postgres.\n\n󰪩 Use Asso docker compose to lift development container for postgres.".to_string());
                }
                "mariadb" => {
                    database_engine.push_str("mariadb");
                    logger_debug("\n Configuring module for sqlx and mariadb....\n".to_string());
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let configuring_sqlx_mariadb: Sqlx = Sqlx::new(module, database_engine);
                    configuring_sqlx_mariadb.create_folder_database();
                    configuring_sqlx_mariadb.create_configuration_files();
                    configuring_sqlx_mariadb.modify_module();
                    configuring_sqlx_mariadb.create_configuration_database();
                    

                    thread::sleep(time::Duration::from_secs(1));
                    logger_info("\n 󰪩 Module configured to work with sqlx and mariadb.\n\n󰪩 Use Asso docker compose to lift development container for mariadb.".to_string());
                }
                _ => logger_error("Database engine has not been currently mapped...".to_string()),
            }
        }
        _ => logger_error("The settings have not been currently assigned.....".to_string()),
    }
}
