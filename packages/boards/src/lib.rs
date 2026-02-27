use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub vid: u16,
    pub pid: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
    pub docker_image: String,
    pub dockerfile: String,
    pub usb: UsbDevice,
    pub flash_tool: String,
    pub build_command: Vec<String>,
}

pub fn get_supported_boards() -> Vec<Board> {
    vec![
        Board {
            name: "rp2040".to_string(),
            docker_image: "raspberrypi/pico-sdk:latest".to_string(),
            dockerfile: "docker/rp2040.Dockerfile".to_string(),
            usb: UsbDevice {
                vid: 0x2e8a,
                pid: vec![0x0003, 0x0005, 0x0009],
            },
            flash_tool: "elf2uf2-rs".to_string(),
            build_command: vec!["cmake".to_string(), "-B".to_string(), "build".to_string(), ".".to_string(), "&&".to_string(), "make".to_string(), "-C".to_string(), "build".to_string()],
        },
        Board {
            name: "esp32c3".to_string(),
            docker_image: "espressif/idf:latest".to_string(),
            dockerfile: "docker/esp32c3.Dockerfile".to_string(),
            usb: UsbDevice {
                vid: 0x303a,
                pid: vec![0x1001],
            },
            flash_tool: "espflash".to_string(),
            build_command: vec!["idf.py".to_string(), "build".to_string()],
        },
        Board {
            name: "stm32f405".to_string(),
            docker_image: "rustembedded/os-check:latest".to_string(),
            dockerfile: "docker/stm32f405.Dockerfile".to_string(),
            usb: UsbDevice {
                vid: 0x0483,
                pid: vec![0xdf11],
            },
            flash_tool: "dfu-util".to_string(),
            build_command: vec!["cargo".to_string(), "build".to_string(), "--release".to_string()],
        },
    ]
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get_supported_boards()
            .into_iter()
            .find(|b| b.name == s.to_lowercase())
            .ok_or_else(|| anyhow::anyhow!("Unsupported board: {}", s))
    }
}
