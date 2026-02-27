use clap::{Parser, Subcommand};
use core::{detect_mcus, builder, flasher};
use boards::Board;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Result;

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
                    println!("{}: {} (VID: 0x{:04x}, PID: 0x{:04x})", i + 1, dev.board.name, dev.vendor_id, dev.product_id);
                }
            }
        }
        Commands::Build { mcu, path } => {
            let board = match mcu {
                Some(m) => Board::from_str(&m)?,
                None => {
                    let devices = detect_mcus()?;
                    if devices.is_empty() {
                        return Err(anyhow::anyhow!("No MCU detected. Please specify --mcu."));
                    }
                    devices[0].board.clone()
                }
            };

            println!("Building for {}...", board.name);
            let artifact = builder::build_project(&board, &path).await?;
            println!("Built successfully: {:?}", artifact);
        }
        Commands::Flash { file, mcu } => {
            let board = match mcu {
                Some(m) => Board::from_str(&m)?,
                None => {
                    let devices = detect_mcus()?;
                    if devices.is_empty() {
                        return Err(anyhow::anyhow!("No MCU detected. Please specify --mcu."));
                    }
                    devices[0].board.clone()
                }
            };

            let firmware_path = match file {
                Some(f) => f,
                None => {
                    PathBuf::from(".").join("build").join(&board.name).join("firmware.bin")
                }
            };

            if !firmware_path.exists() {
                return Err(anyhow::anyhow!("Firmware file not found: {:?}. Try building first with `fork build`.", firmware_path));
            }

            println!("Flashing {} to {}...", firmware_path.display(), board.name);
            flasher::flash_device(&board, &firmware_path)?;
            println!("Flashing complete.");
        }
    }

    Ok(())
}
