use anyhow::Result;
use clap::Parser;

mod cmd;
mod ui;

fn main() -> Result<()> {
    let cli = cmd::Cli::parse();
    ui::title();

    match cli.command {
        cmd::Commands::Build {
            mcu,
            tool,
            registry,
            path,
            extra,
        } => cmd::build_command(mcu, tool, registry, path, extra)?,
        cmd::Commands::Run { mcu, path, command } => cmd::run_command(mcu, path, command)?,
        cmd::Commands::Bake {
            mcu,
            registry,
            path,
        } => cmd::bake_command(mcu, registry, path)?,
    }

    Ok(())
}
