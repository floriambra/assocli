use crate::shared::global::PROJECT_PATH;
use crate::utils::common::{
    check_path::check_directory,
    create_dir::create_dir,
    logger::{logger_error, logger_info, logger_warning},
    remove_directory::delete_folder,
};
use crate::utils::{
    command::new_project::add_project::Project, common::clear_terminal::clear_terminal,
};
use console::style;
use dialoguer::Confirm;
use std::{thread, time::Duration};

pub fn handle_new(project_name: &str) {
    let path_home = PROJECT_PATH.as_deref();

    if let Some(_path_home) = path_home {
        if check_directory(_path_home, "Asso") {
            create_dir(&_path_home.to_path_buf());
        }

        let mut new_project = Project::new(_path_home.to_path_buf(), project_name);

        let confirmed = Confirm::new()
            .with_prompt(
                style(format!(
                    "󰺴 ¿Do you want to create the project with the name: '{project_name}' ?."
                ))
                .blue()
                .bold()
                .to_string(),
            )
            .default(true)
            .interact()
            .unwrap_or_else(|err| {
                logger_error(format!("Error creating project: {err}"));
                false
            });

        if confirmed {
            logger_info(format!(" Creating the project '{project_name}'."));

            thread::sleep(Duration::from_secs(1));

            if new_project.create_project() {
                new_project.create_actix();
                new_project.create_app_structure();
                new_project.create_mod_main();
                new_project.create_env_file();
                new_project.create_files_common();
                new_project.create_files_state();
                new_project.create_env_rs();
                new_project.create_main_rs();
                new_project.add_root_template();
                clear_terminal();
                logger_info("  Project created.".to_string());
            } else {
                logger_warning(format!("  Project not created."));
                delete_folder(&new_project.home_path, project_name);
            }
        } else {
            logger_warning("❌ Cancelled by the user.".to_string());
        }
    } else {
        logger_error("Error obtaining the system's HOME path.".to_string());
    }
}
