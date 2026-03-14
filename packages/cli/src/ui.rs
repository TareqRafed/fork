use anyhow::Result;
use recipes::{Board, Recipe};
use colored::Colorize;
use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::path::Path;

pub fn title() {
    println!(
        "\n  {} {}\n",
        "fork".bold().cyan(),
        "— container orchestration for firmware".dimmed()
    );
}

pub enum Status {
    Ok,
    Info,
    Warn,
    Error,
}

pub fn log(status: Status, msg: &str) {
    let symbol = match status {
        Status::Ok => "✓".green().to_string(),
        Status::Info => "→".cyan().to_string(),
        Status::Warn => "!".yellow().to_string(),
        Status::Error => "✗".red().to_string(),
    };
    println!("  {} {}", symbol, msg);
}

pub fn select<T>(prompt: &str, items: &[T], label: impl Fn(&T) -> String) -> Result<usize> {
    let labels: Vec<String> = items.iter().map(&label).collect();
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&labels)
        .default(0)
        .interact()
        .map_err(Into::into)
}

pub fn select_recipe(
    board: &Board,
    project_path: &Path,
    tool_override: Option<&str>,
) -> Result<Recipe> {
    if let Some(name) = tool_override {
        return board.get_recipe_by_path(name, project_path);
    }

    let detected = board.resolve_recipes(project_path);

    match detected.len() {
        0 => {
            log(
                Status::Warn,
                "No toolchain auto-detected — select manually.",
            );
            let all = board.all_recipes(project_path);
            if all.is_empty() {
                anyhow::bail!("No toolchains configured for {}.", board.name);
            }
            let idx = select("Select a toolchain", &all, |r| r.label.clone())?;
            Ok(all[idx].clone())
        }
        1 => {
            log(
                Status::Ok,
                &format!("Toolchain: {}", detected[0].label.bold()),
            );
            Ok(detected[0].clone())
        }
        _ => {
            let idx = select(
                "Multiple toolchains detected — select one",
                &detected,
                |r| r.label.clone(),
            )?;
            Ok(detected[idx].clone())
        }
    }
}

pub fn divider() {
    println!("  {}", "─".repeat(50).dimmed());
}
