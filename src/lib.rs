
use std::path::Path;
use std::fs;

const BRIGHTNESS_PATH: &str = "/sys/class/backlight";

pub fn get_backlight_devices() -> Option<String> {
    let entries = fs::read_dir(BRIGHTNESS_PATH).ok()?;
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_dir() {
            return Some(path.to_str()?.to_string());
        }
    }
    None
}

pub fn read_brightness(device_path: &str) -> Option<u32> {
    let brightness_path = Path::new(device_path).join("brightness");
    let content = fs::read_to_string(brightness_path).ok()?;
    content.trim().parse().ok()
}

pub fn set_brightness(device_path: &str, value: u32) -> std::io::Result<()> {
    let brightness_path = Path::new(device_path).join("brightness");
    fs::write(brightness_path, value.to_string())
}

