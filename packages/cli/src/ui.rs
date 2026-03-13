use anyhow::Result;
use boards::{Board, Recipe, get_supported_boards};
use colored::Colorize;
use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::path::Path;
use std::str::FromStr;

pub fn title() {
    println!(
        "\n  {} {}\n",
        "fork".bold().cyan(),
        "— multi-mcu firmware toolchain".dimmed()
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

pub fn select_board(mcu_override: Option<&str>) -> Result<Board> {
    if let Some(mcu) = mcu_override {
        return Board::from_str(mcu);
    }

    let detected = detect_boards()?;
    let all = get_supported_boards();

    match detected.len() {
        1 => {
            let d = &detected[0];
            let prompt = format!(
                "Found {} {}. Use it?",
                d.board.name.bold(),
                format!("(VID 0x{:04x} · PID 0x{:04x})", d.vendor_id, d.product_id).dimmed()
            );
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(&prompt)
                .default(true)
                .interact()?
            {
                return Ok(d.board.clone());
            }
            let idx = select("Select a board", all, |b| b.name.clone())?;
            Ok(all[idx].clone())
        }
        n if n > 1 => {
            let idx = select("Multiple boards detected — select one", &detected, |d| {
                format!(
                    "{}  {}",
                    d.board.name,
                    format!("(VID 0x{:04x} · PID 0x{:04x})", d.vendor_id, d.product_id).dimmed()
                )
            })?;
            Ok(detected[idx].board.clone())
        }
        _ => {
            let idx = select("Select a board", all, |b| b.name.clone())?;
            Ok(all[idx].clone())
        }
    }
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
