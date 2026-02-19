use crate::{
    shared::global::PROJECT_PATH,
    utils::{
        command::{api::type_api::Module, config::config_sqlx::Sqlx},
        common::selection_type::{choose_types_relational_bases, choose_your_configuration_type},
    },
};
use console::style;

pub fn handler_config(name_module: &str, name_project: &str) {
    let mut path_project = std::path::PathBuf::new();
    let mut path_module = std::path::PathBuf::new();

    if let Some(_path) = PROJECT_PATH.as_deref() {
        path_project = std::path::PathBuf::from(_path).join(name_project);
        path_module = path_project.join("src/app/module").join(name_module);

        println!("{}", path_module.display());
        if !path_project.exists() {
            eprintln!(
                "{}",
                style(format!("The project {} does not exist", name_project))
                    .on_white()
                    .bold()
            );
            std::process::exit(1)
        }

        if !path_module.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "The module {} does not exist in project {}",
                    name_project, name_module
                ))
                .on_white()
                .bold()
            );
            std::process::exit(1)
        }
    } else {
        eprintln!(
            "{}",
            style("  Error creating project path to query")
                .red()
                .bold()
        )
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
                    println!(
                        "{}",
                        style("\n Configuring module for sqlx and postgres....\n")
                            .cyan()
                            .bold()
                    );
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
                    println!(
                "{}",
                style("\n 󰪩 Module configured to work with sqlx and postgres.\n\n󰪩 Use Asso docker compose to lift development container for postgres.")
                    .green()
                    .bold()
            );
                }
                "mariadb" => {
                    println!("Estamos en mariadb")
                }
                _ => eprintln!(
                    "{}",
                    style("  Database engine has not been currently mapped.....")
                        .yellow()
                        .bold()
                ),
            }
        }
        _ => eprintln!(
            "{}",
            style(
                "  The settings have not been currently assigned.....
"
            )
            .yellow()
            .bold()
        ),
    }
}
