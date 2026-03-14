use anyhow::{Context, Result};
use std::sync::LazyLock;

use crate::engine::Board;

pub fn load_board_from_toml(toml_str: &str) -> Result<Board> {
    toml::from_str(toml_str).context("Failed to parse board TOML")
}

static SUPPORTED_BOARDS: LazyLock<Vec<Board>> = LazyLock::new(|| {
    include!(concat!(env!("OUT_DIR"), "/board_tomls.rs"))
        .iter()
        .map(|&s| load_board_from_toml(s).expect("Corrupted built-in board TOML"))
        .collect()
});

pub fn get_supported_boards() -> &'static [Board] {
    &SUPPORTED_BOARDS
}
