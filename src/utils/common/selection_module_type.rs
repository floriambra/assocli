use dialoguer::{Select, console::style};
use std::process;

const MODULE_TYPES: &[&str] = &["API", "Template", "GraphQL", "MCP"];

pub fn choose_module_type(module_name: &str) -> String {
    let selection = Select::new()
        .with_prompt(
            style(format!(
                "󰅷 How do you want to build the module '{}'?",
                module_name
            ))
            .blue()
            .bold()
            .to_string(),
        )
        .items(MODULE_TYPES)
        .default(0)
        .interact()
        .unwrap_or_else(|err| {
            eprintln!(
                "{}",
                style(format!(" Error selecting module type: {err}"))
                    .red()
                    .bold()
            );
            process::exit(1);
        });

    MODULE_TYPES[selection].to_string()
}
