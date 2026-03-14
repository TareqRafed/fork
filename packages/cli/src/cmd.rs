use anyhow::{Result, bail};
use recipes::{self, Board};
use clap::{Parser, Subcommand};
use colored::Colorize;
use fork_core::{bake_image, build_local_image, build_project, detect_runtime, ensure_image};
use std::{path::PathBuf, str::FromStr};

use crate::ui::{self, Status};

#[derive(Parser)]
#[command(name = "fork")]
#[command(about = "Multi-recipe firmware development, simplified.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        #[arg(short = 'c')]
        recipe: String,
        #[arg(short, long)]
        tool: Option<String>,
        #[arg(short, long)]
        registry: Option<String>,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra: Vec<String>,
    },

    Run {
        #[arg(short, long)]
        recipe: String,
        #[arg(default_value = ".")]
        path: PathBuf,
        /// command to execute inside the container
        command: String,
    },

    Bake {
        #[arg(short, long)]
        recipe: String,
        #[arg(short, long)]
        registry: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

pub fn build_command(
    recipe: String,
    tool: Option<String>,
    registry: Option<String>,
    path: PathBuf,
    extra: Vec<String>,
) -> Result<()> {
    let runtime = detect_runtime()?;
    ui::log(Status::Info, &format!("Using {}", runtime.bold()));

    let board = Board::from_str(&recipe)?;
    let recipe = ui::select_recipe(&board, &path, tool.as_deref())?;

    if recipe.dockerfile.is_empty() {
        ui::log(
            Status::Error,
            "Failed to find an image, this is probably a bug, please open a PR",
        );
        bail!("No image found");
    }

    let image = {
        let tag = recipes::image_tag(registry.as_deref(), &board.name, &recipe);
        ui::log(Status::Info, &format!("Ensuring image {}", tag.bold()));

        match registry.as_deref() {
            Some(_) => ensure_image(&runtime, &tag, &recipe.dockerfile)?,
            None => build_local_image(&runtime, &tag, &recipe.dockerfile)?,
        }

        tag
    };

    ui::log(
        Status::Info,
        &format!(
            "Building {} with {}",
            board.name.bold(),
            recipe.label.bold()
        ),
    );

    ui::divider();

    build_project(&path, &image, &recipe.default_cmd, &extra, &runtime)?;

    ui::divider();
    ui::log(Status::Ok, "Build complete.");

    Ok(())
}

pub fn run_command(recipe: String, path: PathBuf, command: String) -> Result<()> {
    let runtime = detect_runtime()?;
    ui::log(Status::Info, &format!("Using {}", runtime.bold()));

    let board = Board::from_str(&recipe)?;
    let recipe = ui::select_recipe(&board, &path, None)?;

    ui::log(Status::Info, &format!("Running: {}", command.bold()));
    ui::divider();

    build_project(&path, &recipe.dockerfile, &command, &[], &runtime)?;

    ui::divider();
    Ok(())
}

pub fn bake_command(recipe: String, registry: String, path: PathBuf) -> Result<()> {
    let runtime = detect_runtime()?;
    ui::log(Status::Info, &format!("Using {}", runtime.bold()));

    let board = Board::from_str(&recipe)?;

    for recipe in board.all_recipes(&path) {
        if recipe.dockerfile.is_empty() {
            continue;
        }
        let tag = recipes::image_tag(Some(&registry), &board.name, &recipe);
        ui::log(Status::Info, &format!("Baking {}", tag.bold()));
        bake_image(&runtime, &tag, &recipe.dockerfile)?;
        ui::log(Status::Ok, &format!("Pushed {}", tag));
    }

    Ok(())
}
