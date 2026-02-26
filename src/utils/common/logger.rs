use console::style;
use std::process::exit;

pub fn logger_error(msg: String) {
    eprintln!("{}", style(format!(" {msg}")).red().bold());
    exit(1)
}

pub fn logger_debug(msg: String) {
    println!("{}", style(msg).cyan().bold());
}

pub fn logger_info(msg: String) {
    println!("{}", style(msg).green().bold());
}

pub fn logger_warning(msg: String) {
    println!("{}", style(msg).yellow().bold());
}
