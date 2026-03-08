use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub vid: u16,
    pub pid: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTool {
    pub name: String,
    pub docker_image: String,
    pub build_command: Vec<String>,
    pub artifact_path: String,
    pub detect_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
    pub usb: UsbDevice,
    pub flash_tool: String,
    pub build_tools: Vec<BuildTool>,
}

impl Board {
    pub fn get_build_tool(&self, name: &str) -> Result<&BuildTool> {
        self.build_tools
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| anyhow!("Build tool '{}' not found for board '{}'", name, self.name))
    }

    pub fn detect_build_tool(&self, project_path: &Path) -> Option<&BuildTool> {
        self.detect_all_build_tools(project_path).into_iter().next()
    }

    pub fn detect_all_build_tools(&self, project_path: &Path) -> Vec<&BuildTool> {
        self.build_tools
            .iter()
            .filter(|tool| {
                if tool.detect_command.is_empty() {
                    return false;
                }
                Command::new("sh")
                    .arg("-c")
                    .arg(&tool.detect_command)
                    .current_dir(project_path)
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
            })
            .collect()
    }
}

pub fn load_board_from_toml(toml_str: &str) -> Result<Board> {
    toml::from_str(toml_str).map_err(|e| anyhow!("Failed to parse board TOML: {}", e))
}

pub fn load_boards_from_dir(dir: &Path) -> Result<Vec<Board>> {
    let mut boards = Vec::new();
    if !dir.exists() {
        return Err(anyhow!("Boards directory not found: {}", dir.display()));
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "toml") {
            let content = std::fs::read_to_string(&path)?;
            let board = load_board_from_toml(&content)?;
            boards.push(board);
        }
    }
    Ok(boards)
}

pub fn get_supported_boards() -> Vec<Board> {
    let boards_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../boards");
    load_boards_from_dir(&boards_dir).unwrap_or_default()
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get_supported_boards()
            .into_iter()
            .find(|b| b.name == s.to_lowercase())
            .ok_or_else(|| anyhow!("Unsupported board: {}", s))
    }
}
