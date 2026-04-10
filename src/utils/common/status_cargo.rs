use crate::shared::global::PROJECT_PATH;
use crate::utils::common::logger::*;
use crate::utils::common::progress::progress_bar;
use indicatif::ProgressBar;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process, thread,
};

pub fn check_toml_project(name_project: &PathBuf) -> bool {
    let path = PROJECT_PATH.as_deref();

    if let Some(_path) = path {
        let dir_project = _path.join(name_project);
        let cargo_toml = dir_project.join("Cargo.toml");

        if !cargo_toml.exists() {
            logger_error("Cargo.toml file is not present in the project".to_string())
        }

        true
    } else {
        logger_error("Error searching for project in team".to_string());
        false
    }
}

pub fn execute_cargo(arg: &str, optional_arg: Option<&str>, name_project: String) -> bool {
    let path_project = PROJECT_PATH.as_deref();
    let mut directory_project = PathBuf::new();

    if let Some(path) = path_project {
        directory_project = directory_project.join(path).join(&name_project);
    } else {
        logger_error("Project path issues".to_string());
    }

    thread::sleep(std::time::Duration::from_secs(1));

    let progress_def = ProgressBar::new(100);

    let mut cmd = process::Command::new("cargo");
    cmd.arg(arg);
    if let Some(opt) = optional_arg {
        cmd.arg(opt);
    }

    cmd.current_dir(&directory_project)
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            
            let progress_clone = progress_def.clone();
            // 1. Extraer stderr para lectura en tiempo real
            let stderr = child
                .stderr
                .take()
                .expect("stderr must be in Piped mode");
            let reader = BufReader::new(stderr);

            // Hilo dedicado para leer sin bloquear el proceso principal
            progress_bar::start_progress(progress_def.clone(), format!("Compiling {}", name_project));
            thread::spawn(move || {
                for line in reader.lines() {
                    // FIX #1: `lines()` devuelve `Result<String, std::io::Error>`
                    progress_bar::progressing(progress_clone.clone(), &line);
                    if let Ok(line_str) = line {
                        if line_str.contains("Compiling") || line_str.contains("Checking") {
                            progress_clone.inc(1);
                            
                            // Si tu barra necesita el texto descomenta:
                            // progress_bar::update_message(progress_clone.clone(), line_str);
                        }
                    } else {
                        break; // Fin del stream o error de I/O
                    }
                }
                progress_clone.finish();
            });

            // FIX #2: `progressing` espera `Child`. Se pasa UNA sola vez para vincularlo.
            // ⚠️ Nota: Si esta función toma `Child` por valor, lo consume y NO podrás llamar a `wait()` después.
            // La firma idiomática en Rust es `&mut Child` o devolver el `Child`.
            let exit_status = child.wait().expect("Fallo al esperar al proceso cargo");

            

            // Esperar a que termine el proceso

            // FIX #3: `progress_message_finish` espera `Output`. Lo construimos manualmente.
            // Docs: https://doc.rust-lang.org/std/process/struct.Output.html
            let output = process::Output {
                status: exit_status,
                stdout: Vec::new(), // Redirigido a Stdio::null()
                stderr: Vec::new(), // Consumido por el hilo de lectura
            };

            progress_bar::progress_message_finish(progress_def, output.clone());

            if !output.status.success() {
                logger_error("Compilación fallida. Revisa el log de cargo.".to_string());
                false
            } else {
                true
            }
        }
        Err(e) => {
            logger_error(format!("Error al iniciar cargo: {}", e));
            false
        }
    }
}
