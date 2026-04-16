use std::path::PathBuf;

use crate::utils::command::api::type_api::Module;
use crate::utils::common::{
    add_dependency::add_dependency,
    check_path::*,
    create_dir::create_dir,
    create_file::create_file,
    file::*,
    logger::{logger_error, logger_info},
};
use std::{thread, time};

pub struct Sqlx {
    pub module: Module,
    pub database_engine: String,
}

impl Sqlx {
    pub fn new(module: Module, database_engine: String) -> Self {
        Self {
            module,
            database_engine: database_engine.to_lowercase(),
        }
    }

    pub fn create_folder_database(&self) {
        thread::sleep(time::Duration::from_secs(1));

        check_project_path(&self.module.project_path);
        check_module_path(&self.module.module_path);

        if check_directory(
            &self.module.project_path.join("src/app/shared/database"),
            "database",
        ) {
            create_dir(&self.module.project_path.join("src/app/shared/database"));
        }
    }

    pub fn create_configuration_files(&self) {
        thread::sleep(time::Duration::from_secs(1));

        let path_folder_database: PathBuf =
            self.module.project_path.join("src/app/shared/database");
        check_directory_existing(&path_folder_database);

        let path_config_sqlx = path_folder_database.join("connection_sqlx.rs");
        create_file(&path_config_sqlx, None);

        self.ensure_connection_sqlx_base(&path_config_sqlx);

        if self.is_postgres() {
            self.ensure_psql_connection_fn(&path_config_sqlx);
        } else if self.is_mariadb() {
            self.ensure_mariadb_connection_fn(&path_config_sqlx);
        } else {
            logger_error(format!(
                "Database engine '{}' is not supported for sqlx configuration",
                self.database_engine
            ));
        }

        let path_mod_database = path_folder_database.join("mod.rs");
        create_file(&path_mod_database, Some("pub mod connection_sqlx;"));

        let path_docker_file = self.module.project_path.join("docker-compose.yml");
        create_file(&path_docker_file, None);
        self.ensure_docker_compose_base(&path_docker_file);

        if self.is_postgres() {
            self.ensure_postgres_service(&path_docker_file);
        } else if self.is_mariadb() {
            self.ensure_mariadb_service(&path_docker_file);
        } else {
            logger_error(format!(
                "No docker template mapped for engine '{}'",
                self.database_engine
            ));
        }
    }

    pub fn create_configuration_database(&self) {
        self.inject_dependencies();
        self.adding_environment_variables();
        self.adding_database_module();
        self.modify_global_status();
        self.modify_main();
    }

    fn inject_dependencies(&self) {
        thread::sleep(time::Duration::from_secs(1));

        if self.is_postgres() {
            add_dependency(
                "sqlx",
                Some("runtime-tokio-rustls,postgres,uuid,macros"),
                self.module.project_path.to_str().unwrap(),
            );
        } else if self.is_mariadb() {
            add_dependency(
                "sqlx",
                Some("runtime-tokio-rustls,mysql,uuid,macros"),
                self.module.project_path.to_str().unwrap(),
            );
        } else {
            logger_error(format!(
                "No sqlx feature mapping available for '{}'",
                self.database_engine
            ));
        }
    }

    fn adding_environment_variables(&self) {
        thread::sleep(time::Duration::from_secs(1));
        let path_enviroment_variables = self.module.project_path.join(".env");

        if self.is_postgres() {
            let content = "DB_PASSWORD=6745YHDSDJ8923\nDB_NAME=asso\nDATABASE_URL_PSQL=postgres://postgres:6745YHDSDJ8923@127.0.0.1/asso".to_string();

            if verify_content_on_file(&path_enviroment_variables, "DATABASE_URL_PSQL=postgres://") {
                return;
            }
            overwrite_file(&path_enviroment_variables, &content);
        } else if self.is_mariadb() {
            let content = "DB_PASSWORD=6745YHDSDJ8923\nDB_NAME=asso\nDATABASE_URL_MARIADB=mysql://root:6745YHDSDJ8923@127.0.0.1:3306/asso".to_string();

            if verify_content_on_file(&path_enviroment_variables, "DATABASE_URL_MARIADB=mysql://") {
                return;
            }
            overwrite_file(&path_enviroment_variables, &content);
        } else {
            logger_error(format!(
                "No env DATABASE_URL mapping available for '{}'",
                self.database_engine
            ));
        }
    }

    fn adding_database_module(&self) {
        thread::sleep(time::Duration::from_secs(1));
        let path_module_shared = self.module.project_path.join("src/app/shared/mod.rs");
        let content = "pub mod database;";

        if verify_content_on_file(&path_module_shared, content) {
            return;
        }

        overwrite_file(&path_module_shared, content);
    }

    fn modify_global_status(&self) {
        thread::sleep(time::Duration::from_secs(1));
        let path_state_global = self
            .module
            .project_path
            .join("src/app/shared/state/state.rs");

        if self.is_postgres() {
            if verify_content_on_file(&path_state_global, "pub psql_pool: PgPool,") {
                return;
            }

            if !verify_content_on_file(&path_state_global, "use sqlx::PgPool;") {
                concatenate_content(&path_state_global, "use sqlx::PgPool;\n".to_string());
            }

            if !verify_content_on_file(
                &path_state_global,
                "use crate::app::shared::database::connection_sqlx::sqlx::connection_psql;",
            ) {
                concatenate_content(
                    &path_state_global,
                    "use crate::app::shared::database::connection_sqlx::sqlx::connection_psql;\n"
                        .to_string(),
                );
            }

            if verify_content_on_file(&path_state_global, "pub struct AppState {}") {
                modify_file(
                    &path_state_global,
                    "pub struct AppState {}",
                    "pub struct AppState {\n}",
                );
            }

            modify_file(
                &path_state_global,
                "pub struct AppState {",
                "pub struct AppState {\n\tpub psql_pool: PgPool,",
            );

            if !verify_content_on_file(&path_state_global, "url_postgres: &str") {
                if verify_content_on_file(&path_state_global, "pub async fn new(url_mariadb: &str,")
                {
                    modify_file(
                        &path_state_global,
                        "pub async fn new(url_mariadb: &str,",
                        "pub async fn new(url_postgres: &str, url_mariadb: &str,",
                    );
                } else if verify_content_on_file(&path_state_global, "pub async fn new(") {
                    modify_file(
                        &path_state_global,
                        "pub async fn new(",
                        "pub async fn new(url_postgres: &str, ",
                    );
                } else {
                    modify_file(
                        &path_state_global,
                        "pub fn new(",
                        "pub async fn new(url_postgres: &str, ",
                    );
                }
            }

            if !verify_content_on_file(&path_state_global, "tracing::info!") {
                modify_file(
                    &path_state_global,
                    "-> Self {",
                    "-> Self {\n\ttracing::info!(\"🔧 Initializing AppState...\");",
                );
            }

            if verify_content_on_file(&path_state_global, " Self {}") {
                modify_file(&path_state_global, " Self {}", " Self {\n}");
            }

            if !verify_content_on_file(
                &path_state_global,
                "psql_pool: connection_psql(url_postgres).await,",
            ) {
                modify_file(
                    &path_state_global,
                    "  Self {",
                    "  Self { \n\tpsql_pool: connection_psql(url_postgres).await,",
                );
            }
        } else if self.is_mariadb() {
            if verify_content_on_file(&path_state_global, "pub mariadb_pool: MySqlPool,") {
                return;
            }

            if !verify_content_on_file(&path_state_global, "use sqlx::MySqlPool;") {
                concatenate_content(&path_state_global, "use sqlx::MySqlPool;\n".to_string());
            }

            if !verify_content_on_file(
                &path_state_global,
                "use crate::app::shared::database::connection_sqlx::sqlx::connection_mariadb;",
            ) {
                concatenate_content(
                    &path_state_global,
                    "use crate::app::shared::database::connection_sqlx::sqlx::connection_mariadb;\n"
                        .to_string(),
                );
            }

            if verify_content_on_file(&path_state_global, "pub struct AppState {}") {
                modify_file(
                    &path_state_global,
                    "pub struct AppState {}",
                    "pub struct AppState {\n}",
                );
            }

            modify_file(
                &path_state_global,
                "pub struct AppState {",
                "pub struct AppState {\n\tpub mariadb_pool: MySqlPool,",
            );

            if !verify_content_on_file(&path_state_global, "url_mariadb: &str") {
                if verify_content_on_file(
                    &path_state_global,
                    "pub async fn new(url_postgres: &str,",
                ) {
                    modify_file(
                        &path_state_global,
                        "pub async fn new(url_postgres: &str,",
                        "pub async fn new(url_postgres: &str, url_mariadb: &str,",
                    );
                } else if verify_content_on_file(&path_state_global, "pub async fn new(") {
                    modify_file(
                        &path_state_global,
                        "pub async fn new(",
                        "pub async fn new(url_mariadb: &str, ",
                    );
                } else {
                    modify_file(
                        &path_state_global,
                        "pub fn new(",
                        "pub async fn new(url_mariadb: &str, ",
                    );
                }
            }

            if !verify_content_on_file(&path_state_global, "tracing::info!") {
                modify_file(
                    &path_state_global,
                    "-> Self {",
                    "-> Self {\n\ttracing::info!(\"🔧 Initializing AppState...\");",
                );
            }

            if verify_content_on_file(&path_state_global, " Self {}") {
                modify_file(&path_state_global, " Self {}", " Self {\n}");
            }

            if !verify_content_on_file(
                &path_state_global,
                "mariadb_pool: connection_mariadb(url_mariadb).await,",
            ) {
                modify_file(
                    &path_state_global,
                    "  Self {",
                    "  Self { \n\tmariadb_pool: connection_mariadb(url_mariadb).await,",
                );
            }
        } else {
            logger_error(format!(
                "No AppState mapping available for '{}'",
                self.database_engine
            ));
        }

        self.normalize_state_new_signature(&path_state_global);
    }

    fn modify_main(&self) {
        thread::sleep(time::Duration::from_secs(1));
        let path_main = self.module.project_path.join("src/main.rs");

        if self.is_postgres() {
            if !verify_content_on_file(&path_main, "env_url_database_postgres") {
                modify_file(
                    &path_main,
                    "let env_address: &str = &var_env.get_or(\"ADDRESS\", \"127.0.0.1\");",
                    "let env_address: &str = &var_env.get_or(\"ADDRESS\", \"127.0.0.1\");\nlet env_url_database_postgres = &var_env.get_or(\"DATABASE_URL_PSQL\", \" \");\nif env_url_database_postgres.is_empty() {\ntracing::error!(\" Error reading postgres url from database, not found in environment variables\");\n}",
                );
            }

            if verify_content_on_file(&path_main, "AppState::new()") {
                modify_file(&path_main, "AppState::new()", "AppState::new().await");
            }

            if verify_content_on_file(&path_main, "AppState::new(") {
                if verify_content_on_file(&path_main, "AppState::new(env_url_database_postgres") {
                    // already mapped
                } else if verify_content_on_file(
                    &path_main,
                    "AppState::new( env_url_database_postgres",
                ) {
                    // already mapped
                } else if verify_content_on_file(&path_main, ", env_url_database_postgres") {
                    // already mapped
                } else if verify_content_on_file(&path_main, "AppState::new().await") {
                    modify_file(
                        &path_main,
                        "AppState::new().await",
                        "AppState::new(env_url_database_postgres).await",
                    );
                } else if verify_content_on_file(
                    &path_main,
                    "AppState::new(env_url_database_mariadb",
                ) {
                    modify_file(
                        &path_main,
                        "AppState::new(env_url_database_mariadb",
                        "AppState::new(env_url_database_postgres, env_url_database_mariadb",
                    );
                } else if verify_content_on_file(
                    &path_main,
                    "AppState::new( env_url_database_mariadb",
                ) {
                    modify_file(
                        &path_main,
                        "AppState::new( env_url_database_mariadb",
                        "AppState::new( env_url_database_postgres, env_url_database_mariadb",
                    );
                } else if verify_content_on_file(&path_main, "AppState::new(") {
                    modify_file(
                        &path_main,
                        "AppState::new(",
                        "AppState::new(env_url_database_postgres, ",
                    );
                }
            }
        } else if self.is_mariadb() {
            if !verify_content_on_file(&path_main, "env_url_database_mariadb") {
                modify_file(
                    &path_main,
                    "let env_address: &str = &var_env.get_or(\"ADDRESS\", \"127.0.0.1\");",
                    "let env_address: &str = &var_env.get_or(\"ADDRESS\", \"127.0.0.1\");\nlet env_url_database_mariadb = &var_env.get_or(\"DATABASE_URL_MARIADB\", \" \");\nif env_url_database_mariadb.is_empty() {\ntracing::error!(\" Error reading mariadb url from database, not found in environment variables\");\n}",
                );
            }

            if verify_content_on_file(&path_main, "AppState::new()") {
                modify_file(&path_main, "AppState::new()", "AppState::new().await");
            }

            if verify_content_on_file(&path_main, "AppState::new(") {
                if verify_content_on_file(&path_main, "AppState::new(env_url_database_mariadb") {
                    // already mapped
                } else if verify_content_on_file(
                    &path_main,
                    "AppState::new( env_url_database_mariadb",
                ) {
                    // already mapped
                } else if verify_content_on_file(&path_main, ", env_url_database_mariadb") {
                    // already mapped
                } else if verify_content_on_file(&path_main, "AppState::new().await") {
                    modify_file(
                        &path_main,
                        "AppState::new().await",
                        "AppState::new(env_url_database_mariadb).await",
                    );
                } else if verify_content_on_file(
                    &path_main,
                    "AppState::new(env_url_database_postgres",
                ) {
                    modify_file(
                        &path_main,
                        "AppState::new(env_url_database_postgres",
                        "AppState::new(env_url_database_postgres, env_url_database_mariadb",
                    );
                } else if verify_content_on_file(
                    &path_main,
                    "AppState::new( env_url_database_postgres",
                ) {
                    modify_file(
                        &path_main,
                        "AppState::new( env_url_database_postgres",
                        "AppState::new( env_url_database_postgres, env_url_database_mariadb",
                    );
                } else if verify_content_on_file(&path_main, "AppState::new(") {
                    modify_file(
                        &path_main,
                        "AppState::new(",
                        "AppState::new(env_url_database_mariadb, ",
                    );
                }
            }
        } else {
            logger_error(format!(
                "No main.rs mapping available for '{}'",
                self.database_engine
            ));
        }
    }

    pub fn modify_module(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let path_module = self.module.module_path.join("mod.rs");
        let path_repositories = self.module.module_path.join("repositories.rs");

        let pool_field = if self.is_postgres() {
            "psql_pool"
        } else if self.is_mariadb() {
            "mariadb_pool"
        } else {
            logger_error(format!(
                "No module pool mapping available for '{}'",
                self.database_engine
            ));
            return;
        };

        if !verify_content_on_file(&path_module, "Repository::new(pool") {
            modify_file(&path_module, "Repository::new(", "Repository::new(pool");
        }

        if verify_content_on_file(&path_module, "let pool") {
            logger_error("The selected module already has a sqlx configuration (postgres or mariadb). A module can only use one sqlx pool.".to_string());
        }

        if verify_content_on_file(
            &path_module,
            "pub fn configure(_state: std::sync::Arc<AppState>) -> Router {",
        ) {
            let insertion = format!(
                "pub fn configure(_state: std::sync::Arc<AppState>) -> Router {{\nlet pool = _state.{}.clone();",
                pool_field
            );

            modify_file(
                &path_module,
                "pub fn configure(_state: std::sync::Arc<AppState>) -> Router {",
                &insertion,
            );
        }

        if !verify_content_on_file(&path_module, "let pool = _state.psql_pool.clone();")
            && !verify_content_on_file(&path_module, "let pool = _state.mariadb_pool.clone();")
        {
            logger_error("Unable to determine sqlx pool configuration for module. Module must have exactly one sqlx pool.".to_string());
        }

        delete_file_content(&path_repositories);

        if self.is_postgres() {
            load_template_arg(
                "repository_sqlx_postgres.rs",
                &path_repositories,
                &self.module.name_module,
            );
        } else if self.is_mariadb() {
            load_template_arg(
                "repository_sqlx_mariadb.rs",
                &path_repositories,
                &self.module.name_module,
            );
        } else {
            logger_error(format!(
                "No repository template mapped for '{}'",
                self.database_engine
            ));
        }

        logger_info(format!("{} configured successfully", self.database_engine));
    }

    fn is_postgres(&self) -> bool {
        self.database_engine == "postgres"
    }

    fn is_mariadb(&self) -> bool {
        self.database_engine == "mariadb"
    }

    fn ensure_connection_sqlx_base(&self, path_config_sqlx: &PathBuf) {
        if verify_content_on_file(path_config_sqlx, "pub mod sqlx {") {
            return;
        }

        if std::fs::write(path_config_sqlx, "pub mod sqlx {\n}\n").is_err() {
            logger_error(format!(
                "Error creating base sqlx connection module in {}",
                path_config_sqlx.display()
            ));
        }
    }

    fn ensure_psql_connection_fn(&self, path_config_sqlx: &PathBuf) {
        if verify_content_on_file(
            path_config_sqlx,
            "pub async fn connection_psql(url: &str) -> PgPool {",
        ) {
            return;
        }

        if !verify_content_on_file(
            path_config_sqlx,
            "use sqlx::{PgPool, postgres::PgPoolOptions};",
        ) {
            self.insert_after(
                path_config_sqlx,
                "pub mod sqlx {\n",
                "    use sqlx::{PgPool, postgres::PgPoolOptions};\n",
            );
        }

        if !verify_content_on_file(path_config_sqlx, "use std::time::Duration;") {
            self.insert_after(
                path_config_sqlx,
                "pub mod sqlx {\n",
                "    use std::time::Duration;\n",
            );
        }

        let fn_block = "    pub async fn connection_psql(url: &str) -> PgPool {\n        let connection_manager = PgPoolOptions::new()\n            .max_connections(20)\n            .min_connections(2)\n            .acquire_timeout(Duration::from_secs(5))\n            .idle_timeout(Duration::from_secs(300))\n            .max_lifetime(Duration::from_secs(1800))\n            .test_before_acquire(true)\n            .connect(url)\n            .await;\n\n        match connection_manager {\n            Ok(pool) => {\n                tracing::info!(\"Successful database connection pool sqlx postgres\");\n                if let Err(e) = sqlx::query(\"SELECT 1\").execute(&pool).await {\n                    tracing::error!(\"Database connectivity test failed: {:?}\", e);\n                    std::process::exit(1);\n                }\n\n                pool\n            }\n            Err(err) => {\n                tracing::error!(\"Error creating database pool sqlx postgres: {:?}\", err);\n                std::process::exit(1)\n            }\n        }\n    }\n";

        self.insert_before(path_config_sqlx, "}\n", fn_block);
    }

    fn ensure_mariadb_connection_fn(&self, path_config_sqlx: &PathBuf) {
        if verify_content_on_file(
            path_config_sqlx,
            "pub async fn connection_mariadb(url: &str) -> MySqlPool {",
        ) {
            return;
        }

        if !verify_content_on_file(
            path_config_sqlx,
            "use sqlx::{MySqlPool, mysql::MySqlPoolOptions};",
        ) {
            self.insert_after(
                path_config_sqlx,
                "pub mod sqlx {\n",
                "    use sqlx::{MySqlPool, mysql::MySqlPoolOptions};\n",
            );
        }

        if !verify_content_on_file(path_config_sqlx, "use std::time::Duration;") {
            self.insert_after(
                path_config_sqlx,
                "pub mod sqlx {\n",
                "    use std::time::Duration;\n",
            );
        }

        let fn_block = "    pub async fn connection_mariadb(url: &str) -> MySqlPool {\n        let connection_manager = MySqlPoolOptions::new()\n            .max_connections(20)\n            .min_connections(2)\n            .acquire_timeout(Duration::from_secs(5))\n            .idle_timeout(Duration::from_secs(300))\n            .max_lifetime(Duration::from_secs(1800))\n            .test_before_acquire(true)\n            .connect(url)\n            .await;\n\n        match connection_manager {\n            Ok(pool) => {\n                tracing::info!(\"Successful database connection pool sqlx mariadb\");\n                if let Err(e) = sqlx::query(\"SELECT 1\").execute(&pool).await {\n                    tracing::error!(\"Database connectivity test failed: {:?}\", e);\n                    std::process::exit(1);\n                }\n\n                pool\n            }\n            Err(err) => {\n                tracing::error!(\"Error creating database pool sqlx mariadb: {:?}\", err);\n                std::process::exit(1)\n            }\n        }\n    }\n";

        self.insert_before(path_config_sqlx, "}\n", fn_block);
    }

    fn insert_after(&self, path: &PathBuf, marker: &str, insertion: &str) {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        if let Some(index) = content.find(marker) {
            let split_at = index + marker.len();
            let mut new_content = String::new();
            new_content.push_str(&content[..split_at]);
            new_content.push_str(insertion);
            new_content.push_str(&content[split_at..]);

            if std::fs::write(path, new_content).is_err() {
                logger_error(format!("Error updating file {}", path.display()));
            }
        } else {
            logger_error(format!(
                "Could not find marker '{}' in {}",
                marker,
                path.display()
            ));
        }
    }

    fn insert_before(&self, path: &PathBuf, marker: &str, insertion: &str) {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        if let Some(index) = content.rfind(marker) {
            let mut new_content = String::new();
            new_content.push_str(&content[..index]);
            if !content[..index].ends_with('\n') {
                new_content.push('\n');
            }
            new_content.push_str(insertion);
            new_content.push_str(&content[index..]);

            if std::fs::write(path, new_content).is_err() {
                logger_error(format!("Error updating file {}", path.display()));
            }
        } else {
            logger_error(format!(
                "Could not find marker '{}' in {}",
                marker,
                path.display()
            ));
        }
    }

    fn ensure_docker_compose_base(&self, path_docker_file: &PathBuf) {
        const VOLUMES_MARKER: &str = "  # asso-managed-volumes";
        let content = std::fs::read_to_string(path_docker_file).unwrap_or_default();

        if content.trim().is_empty() {
            let base = format!("services:\nvolumes:\n{VOLUMES_MARKER}\n");
            if std::fs::write(path_docker_file, base).is_err() {
                logger_error(format!(
                    "Error creating docker-compose base in {}",
                    path_docker_file.display()
                ));
            }
            return;
        }

        if !content.contains("services:") {
            if content.contains("volumes:\n") {
                self.insert_before(path_docker_file, "volumes:\n", "services:\n");
            } else {
                let mut new_content = content.clone();
                if !new_content.ends_with('\n') {
                    new_content.push('\n');
                }
                new_content.push_str("services:\n");
                if std::fs::write(path_docker_file, new_content).is_err() {
                    logger_error(format!(
                        "Error adding services section in {}",
                        path_docker_file.display()
                    ));
                }
            }
        }

        let refreshed = std::fs::read_to_string(path_docker_file).unwrap_or_default();
        if !refreshed.contains("volumes:\n") {
            let mut new_content = refreshed;
            if !new_content.ends_with('\n') {
                new_content.push('\n');
            }
            new_content.push_str("volumes:\n");
            if std::fs::write(path_docker_file, new_content).is_err() {
                logger_error(format!(
                    "Error adding volumes section in {}",
                    path_docker_file.display()
                ));
            }
        }

        let refreshed = std::fs::read_to_string(path_docker_file).unwrap_or_default();
        if !refreshed.contains(VOLUMES_MARKER) {
            self.insert_after(path_docker_file, "volumes:\n", "  # asso-managed-volumes\n");
        }
    }

    fn ensure_postgres_service(&self, path_docker_file: &PathBuf) {
        const VOLUMES_MARKER: &str = "  # asso-managed-volumes";
        let content = std::fs::read_to_string(path_docker_file).unwrap_or_default();

        if !content.contains("db_postgres:") {
            let block = "  db_postgres:\n    image: postgres:16.2\n    restart: always\n    ports:\n      - \"5432:5432\"\n    environment:\n      POSTGRES_PASSWORD: ${DB_PASSWORD}\n      POSTGRES_DB: ${DB_NAME}\n    container_name: database_sqlx_postgres\n    volumes:\n      - pg_data:/var/lib/postgresql/data\n";
            self.insert_after(path_docker_file, "services:\n", block);
        }

        let refreshed = std::fs::read_to_string(path_docker_file).unwrap_or_default();
        let has_managed_pg = refreshed.contains("\n  pg_data:\n");
        if !has_managed_pg {
            if refreshed.contains(VOLUMES_MARKER) {
                self.insert_after(path_docker_file, VOLUMES_MARKER, "\n  pg_data:\n");
            } else {
                self.insert_after(
                    path_docker_file,
                    "volumes:\n",
                    "  # asso-managed-volumes\n  pg_data:\n",
                );
            }
        }
    }

    fn ensure_mariadb_service(&self, path_docker_file: &PathBuf) {
        const VOLUMES_MARKER: &str = "  # asso-managed-volumes";
        let content = std::fs::read_to_string(path_docker_file).unwrap_or_default();

        if !content.contains("db_mariadb:") {
            let block = "  db_mariadb:\n    image: mariadb:11.4\n    restart: always\n    ports:\n      - \"3306:3306\"\n    environment:\n      MARIADB_ROOT_PASSWORD: ${DB_PASSWORD}\n      MARIADB_DATABASE: ${DB_NAME}\n    container_name: database_sqlx_mariadb\n    volumes:\n      - mariadb_data:/var/lib/mysql\n";
            self.insert_after(path_docker_file, "services:\n", block);
        }

        let refreshed = std::fs::read_to_string(path_docker_file).unwrap_or_default();
        let has_managed_mariadb = refreshed.contains("\n  mariadb_data:\n");
        if !has_managed_mariadb {
            if refreshed.contains(VOLUMES_MARKER) {
                self.insert_after(path_docker_file, VOLUMES_MARKER, "\n  mariadb_data:\n");
            } else {
                self.insert_after(
                    path_docker_file,
                    "volumes:\n",
                    "  # asso-managed-volumes\n  mariadb_data:\n",
                );
            }
        }
    }

    fn normalize_state_new_signature(&self, path_state_global: &PathBuf) {
        let mut content = std::fs::read_to_string(path_state_global).unwrap_or_default();

        if content.is_empty() {
            return;
        }

        let has_postgres = content.contains("pub psql_pool: PgPool,")
            || content.contains("psql_pool: connection_psql(url_postgres).await,");
        let has_mariadb = content.contains("pub mariadb_pool: MySqlPool,")
            || content.contains("mariadb_pool: connection_mariadb(url_mariadb).await,");

        let target_signature = if has_postgres && has_mariadb {
            "pub async fn new(url_postgres: &str, url_mariadb: &str,"
        } else if has_postgres {
            "pub async fn new(url_postgres: &str,"
        } else if has_mariadb {
            "pub async fn new(url_mariadb: &str,"
        } else {
            return;
        };

        let candidates = [
            "pub async fn new(url_postgres: &str, url_mariadb: &str,",
            "pub async fn new(url_mariadb: &str, url_postgres: &str,",
            "pub async fn new(url_postgres: &str,",
            "pub async fn new(url_mariadb: &str,",
            "pub async fn new(",
            "pub fn new(",
        ];

        let mut replaced = false;
        for candidate in candidates {
            if content.contains(candidate) {
                content = content.replacen(candidate, target_signature, 1);
                replaced = true;
                break;
            }
        }

        if replaced && std::fs::write(path_state_global, content).is_err() {
            logger_error(format!(
                "Error normalizing AppState::new signature in {}",
                path_state_global.display()
            ));
        }
    }
}
