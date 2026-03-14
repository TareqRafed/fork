use anyhow::Result;
use clap::Parser;

mod cmd;
mod ui;

fn main() -> Result<()> {
    let cli = cmd::Cli::parse();
    ui::title();

    match cli.command {
        cmd::Commands::Build {
            recipe,
            tool,
            registry,
            path,
            extra,
        } => cmd::build_command(recipe, tool, registry, path, extra)?,
        cmd::Commands::Run {
            recipe,
            path,
            command,
        } => cmd::run_command(recipe, path, command)?,
        cmd::Commands::Bake {
            recipe,
            registry,
            path,
        } => cmd::bake_command(recipe, registry, path)?,
    }

    Ok(())
}
