pub mod progress_bar {
    use std::io::Error;

    use indicatif::{ProgressBar, ProgressStyle};

    use crate::utils::common::logger::{logger_error, logger_info};

    pub fn start_progress(progress_bar: ProgressBar, message: String) {
        progress_bar.set_style(
            ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos:>3}% {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        progress_bar.set_message(message);
    }

    /* 
    pub fn progressing(progress_bar: ProgressBar, line: &Result<String,Error>) {
        let mut progress = 0;
        while let Ok(_) = line {
            progress = (progress + 1).min(100);
            progress_bar.set_position(progress);
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
    }
*/
pub fn progressing(progress_bar: ProgressBar, line: &Result<String, Error>) {
    // Solo actuamos si la línea se leyó correctamente
    if line.is_ok() {
        // 1️⃣ Reiniciar barra al inicio por cada nuevo paquete
        progress_bar.set_position(0);

        // 2️⃣ Animación visual de 0 a 100 para esta dependencia
        for step in 0..=100 {
            progress_bar.set_position(step);
            // Pausa ajustable para efecto visual. 
            // 15ms = ~1.5 seg por paquete. Modifica según tu preferencia.
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    }
}

    pub fn progress_message_finish(progress_bar: ProgressBar, output: std::process::Output) {
        if output.status.success() {
            progress_bar.finish_with_message("\nProgress completed....");
            logger_info("  Compilation completed successfully".to_string());
        } else {
            progress_bar.finish_with_message("\nProgress fault....");
            std::thread::sleep(std::time::Duration::from_secs(2));
            logger_error("Error during compilation.".to_string());
        }
    }
}
