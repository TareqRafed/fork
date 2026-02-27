use anyhow::Result;
use boards::{Board, get_supported_boards};
use nusb::list_devices;

pub struct DeviceInfo {
    pub board: Board,
    pub vendor_id: u16,
    pub product_id: u16,
}

pub fn detect_mcus() -> Result<Vec<DeviceInfo>> {
    let mut detected = Vec::new();
    let devices = list_devices()?;
    let supported = get_supported_boards();

    for device in devices {
        let vid = device.vendor_id();
        let pid = device.product_id();

        if let Some(board) = supported.iter().find(|b| {
            b.usb.vid == vid && b.usb.pid.contains(&pid)
        }) {
            detected.push(DeviceInfo {
                board: board.clone(),
                vendor_id: vid,
                product_id: pid,
            });
        }
    }

    Ok(detected)
}
