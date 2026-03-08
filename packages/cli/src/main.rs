use anyhow::Result;
use boards::{get_supported_boards, Board, BuildTool};
use clap::{Parser, Subcommand};
use core::{builder, detect_mcus, flasher};
use dialoguer::{Confirm, Select};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "fork")]
#[command(about = "A CLI for multi-MCU development", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Detect,
    Build {
        #[arg(short, long)]
        mcu: Option<String>,
        #[arg(short, long)]
        tool: Option<String>,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Flash {
        #[arg(short, long)]
        file: Option<PathBuf>,
        #[arg(short, long)]
        mcu: Option<String>,
    },
}

fn select_board(mcu_override: Option<&str>) -> Result<Board> {
    if let Some(mcu) = mcu_override {
        return Board::from_str(mcu);
    }

    let detected = detect_mcus()?;
    let all_boards = get_supported_boards();

    if let Some(dev) = detected.first() {
        let prompt = format!(
            "Found {} (VID: 0x{:04x}, PID: 0x{:04x}). Use this board?",
            dev.board.name, dev.vendor_id, dev.product_id
        );
        if Confirm::new().with_prompt(&prompt).default(true).interact()? {
            return Ok(dev.board.clone());
        }
    }

    if all_boards.is_empty() {
        return Err(anyhow::anyhow!("No boards configured."));
    }

    let board_names: Vec<&str> = all_boards.iter().map(|b| b.name.as_str()).collect();
    let selection = Select::new()
        .with_prompt("Select a board")
        .items(&board_names)
        .default(0)
        .interact()?;

    Ok(all_boards.into_iter().nth(selection).unwrap())
}

fn select_build_tool<'a>(
    board: &'a Board,
    project_path: &Path,
    tool_override: Option<&str>,
) -> Result<&'a BuildTool> {
    if let Some(tool_name) = tool_override {
        return board.get_build_tool(tool_name);
    }

    let detected = board.detect_all_build_tools(project_path);

    match detected.len() {
        0 => {
            println!("No build tools auto-detected for this project.");
            let tool_names: Vec<&str> = board.build_tools.iter().map(|t| t.name.as_str()).collect();
            let selection = Select::new()
                .with_prompt("Select a build tool")
                .items(&tool_names)
                .default(0)
                .interact()?;
            Ok(&board.build_tools[selection])
        }
        1 => {
            println!("Detected build tool: {}", detected[0].name);
            Ok(detected[0])
        }
        _ => {
            let tool_names: Vec<&str> = detected.iter().map(|t| t.name.as_str()).collect();
            let selection = Select::new()
                .with_prompt("Multiple build tools detected. Select one")
                .items(&tool_names)
                .default(0)
                .interact()?;
            Ok(detected[selection])
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect => {
            let devices = detect_mcus()?;
            if devices.is_empty() {
                println!("No supported MCUs detected.");
            } else {
                println!("Detected MCUs:");
                for (i, dev) in devices.iter().enumerate() {
                    println!(
                        "{}: {} (VID: 0x{:04x}, PID: 0x{:04x})",
                        i + 1,
                        dev.board.name,
                        dev.vendor_id,
                        dev.product_id
                    );
                }
            }
        }
        Commands::Build { mcu, tool, path } => {
            let board = select_board(mcu.as_deref())?;
            let build_tool = select_build_tool(&board, &path, tool.as_deref())?;

            println!("Building for {} with {}...", board.name, build_tool.name);
            let artifact = builder::build_project(&board, &path, Some(&build_tool.name)).await?;
            println!("Built successfully: {:?}", artifact);
        }
        Commands::Flash { file, mcu } => {
            let board = select_board(mcu.as_deref())?;

            let firmware_path = match file {
                Some(f) => f,
                None => PathBuf::from(".")
                    .join("build")
                    .join(&board.name)
                    .join("firmware.bin"),
            };

            if !firmware_path.exists() {
                return Err(anyhow::anyhow!(
                    "Firmware file not found: {:?}. Try building first with `fork build`.",
                    firmware_path
                ));
            }

            println!("Flashing {} to {}...", firmware_path.display(), board.name);
            flasher::flash_device(&board, &firmware_path)?;
            println!("Flashing complete.");
        }
    }

    Ok(())
}
