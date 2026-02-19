use dialoguer::{Select, console::style};
use std::process;

const MODULE_TYPES: &[&str] = &["API", "Template", "GraphQL", "MCP"];
const CONFIG_TYPES: &[&str] = &["sqlx", "mongodb", "reddis", "autentication"];
const TYPES_RELATIONAL_BASES: &[&str] = &["postgres", "mysql", "mariadb"];

pub fn choose_module_type(module_name: &str) -> &'static str {
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

    MODULE_TYPES[selection]
}

pub fn choose_your_configuration_type(module_name: &str) -> &'static str {
    let selection = Select::new()
        .with_prompt(
            style(format!(
                "󰅷 How do you want to build the configuration in the module '{}'?",
                module_name
            ))
            .blue()
            .bold()
            .to_string(),
        )
        .items(CONFIG_TYPES)
        .default(0)
        .interact()
        .unwrap_or_else(|err| {
            eprintln!(
                "{}",
                style(format!(" Error selecting config type: {err}"))
                    .red()
                    .bold()
            );
            process::exit(1);
        });

    CONFIG_TYPES[selection]
}

pub fn choose_types_relational_bases() -> &'static str {
    let selection = Select::new()
        .with_prompt(
            style("󰅷 Which database engine do you want to use?".to_string())
                .blue()
                .bold()
                .to_string(),
        )
        .items(TYPES_RELATIONAL_BASES)
        .default(0)
        .interact()
        .unwrap_or_else(|err| {
            eprintln!(
                "{}",
                style(format!(" Error selecting config type: {err}"))
                    .red()
                    .bold()
            );
            process::exit(1);
        });

    TYPES_RELATIONAL_BASES[selection]
}

/*
pub fn choose_types_relational_bases() -> String {
    let input: String = Input::new()
        .with_prompt("󰅷 Enter the database engine name")
        .interact_text()
        .unwrap_or_else(|err| {
            eprintln!(" Error reading input: {err}");
            process::exit(1);
        });

    TYPES_RELATIONAL_BASES
        .iter()
        .find(|&&item| item.eq_ignore_ascii_case(&input))
        .unwrap_or_else(|| {
            eprintln!(" Database engine not found");
            process::exit(1);
        })
        .to_string()
}
*/
