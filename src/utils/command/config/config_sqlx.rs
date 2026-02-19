use std::path::PathBuf;

use crate::utils::command::api::type_api::Module;
use crate::utils::common::{
    add_dependency::add_dependency,
    check_path::*,
    create_dir::create_dir,
    create_file::create_file,
    file::*,
};

pub struct Sqlx {
    pub module: Module,
    pub database_engine: String,
}

impl Sqlx {
    pub fn new(module: Module, database_engine: String) -> Self {
        Self {
            module,
            database_engine,
        }
    }

    pub fn create_folder_database(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));

        check_project_path(&self.module.project_path);
        check_module_path(&self.module.module_path);

        let path_folder_database: PathBuf =
            self.module.project_path.join("src/app/shared/database");
        if check_directory(&path_folder_database, "database") {
            create_dir(&path_folder_database);
        }
    }

    pub fn create_configuration_files(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let path_folder_database: PathBuf =
            self.module.project_path.join("src/app/shared/database");
        check_directory_existing(&path_folder_database);

        let path_config_sqlx = path_folder_database.join("connection_sqlx.rs");
        create_file(&path_config_sqlx, None);
        load_template("config_sqlx_postgres.rs", &path_config_sqlx);

        let path_mod_database = path_folder_database.join("mod.rs");
        create_file(&path_mod_database, Some("pub mod connection_sqlx;"));

        let path_docker_file = self.module.project_path.join("docker-compose.yml");
        create_file(&path_docker_file, None);
        load_template("docker-file-postgres-database.yml", &path_docker_file);
    }

    pub fn inject_dependencies(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        add_dependency(
            "sqlx",
            Some("runtime-tokio-rustls,postgres,uuid,macros"),
            self.module.project_path.to_str().unwrap(),
        );
    }
    pub fn adding_environment_variables(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let path_enviroment_variables = self.module.project_path.join(".env");
        let content = 
            "DB_PASSWORD=6745YHDSDJ8923\nDB_NAME=asso\nDATABASE_URL=postgres://postgres:6745YHDSDJ8923@127.0.0.1/asso".to_string();

        if verify_content_on_file(&path_enviroment_variables, "DATABASE_URL=postgres:") {
            return;
        };
        overwrite_file(&path_enviroment_variables, &content);
    }

    pub fn adding_database_module(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let path_module_shared = self.module.project_path.join("src/app/shared/mod.rs");
        let content = "pub mod database;";

        if verify_content_on_file(&path_module_shared, content) {
            return;
        };
        overwrite_file(&path_module_shared, content);
    }

    pub fn modify_global_status(&self) { 
        std::thread::sleep(std::time::Duration::from_secs(1));
        let path_state_global = self.module.project_path.join("src/app/shared/state/state.rs");

        if verify_content_on_file(&path_state_global, "pub sqlx_pool: PgPool,") {
            return; 
        }
        
        concatenate_content(&path_state_global, "use sqlx::PgPool;\nuse crate::app::shared::database::connection_sqlx::sqlx::connection;\n\n".to_string());

        if verify_content_on_file(&path_state_global, "pub struct AppState {}") {
           modify_file(&path_state_global, "pub struct AppState {}", "pub struct AppState {\n}"); 
        }


        modify_file(&path_state_global,"pub struct AppState {","pub struct AppState {\n\tpub sqlx_pool: PgPool,");

        if !verify_content_on_file(&path_state_global, "url_postgres: &str") {
            modify_file(&path_state_global, "pub fn new(", "pub async fn new(url_postgres: &str,");
        }

        if !verify_content_on_file(&path_state_global, "tracing::info!") { 
            modify_file(&path_state_global, "-> Self {", "-> Self {\n\ttracing::info!(\"🔧 Initializing AppState...\");");
        }

        if verify_content_on_file(&path_state_global, " Self {}") {
           modify_file(&path_state_global, " Self {}", " Self {\n}"); 
        }

        modify_file(&path_state_global,"  Self {","  Self { \n\tsqlx_pool: connection(url_postgres).await,");
        
    }

    pub fn modify_main(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let path_main = self.module.project_path.join("src/main.rs");

        if !verify_content_on_file(&path_main, "env_url_database_postgres") {
            modify_file(&path_main, "let env_address: &str = &var_env.get_or(\"ADDRESS\", \"127.0.0.1\");","let env_address: &str = &var_env.get_or(\"ADDRESS\", \"127.0.0.1\");\nlet env_url_database_postgres = &var_env.get_or(\"DATABASE_URL\", \" \");\nif env_url_database_postgres.is_empty() {\ntracing::error!(\" Error reading url from database, not found in environment variables\");\n}");
            if verify_content_on_file(&path_main, "AppState::new()") {
                modify_file(&path_main, "AppState::new()", "AppState::new().await");
            }
            modify_file(&path_main, "AppState::new(", "AppState::new( env_url_database_postgres");
        } 


    }

     pub fn modify_module(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let path_module = self.module.module_path.join("mod.rs");
        let path_repositories = self.module.module_path.join("repositories.rs");

        if !verify_content_on_file(&path_module, "sqlx_pool") {
            modify_file(&path_module, "pub fn configure(state: std::sync::Arc<AppState>) -> Router {", "pub fn configure(state: std::sync::Arc<AppState>) -> Router {\nlet pool = state.sqlx_pool.clone();");
            modify_file(&path_module, "Repository::new(", "Repository::new(pool");
        }

        delete_file_content(&path_repositories);
        load_template_arg("repository_sqlx_postgres.rs", &path_repositories, &self.module.name_module);
          
    }


}
