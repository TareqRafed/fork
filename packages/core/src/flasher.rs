use anyhow::Result;
use boards::Board;
use std::path::Path;

pub fn flash_device(board: &Board, _firmware_path: &Path) -> Result<()> {
    println!("Flashing {} using {}...", board.name, board.flash_tool);
    Ok(())
}
