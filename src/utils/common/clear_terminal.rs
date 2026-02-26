use crate::utils::common::logger::*;

pub fn clear_terminal() {
    if cfg!(windows) {
        let clear = std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status();

        if clear.is_err() {
            logger_error("Error process clear terminal".to_string());
        }
    } else {
        let clear = std::process::Command::new("clear").status();

        if clear.is_err() {
            logger_error("Error process clear terminal".to_string());
        }
    }
}
