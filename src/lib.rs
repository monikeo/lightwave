
use std::path::Path;
use std::fs;
use clap::Parser;

const BRIGHTNESS_PATH: &str = "/sys/class/backlight";

#[derive(Parser, Debug)]
#[clap(name = "Lightwave", version = "1.0", about = "Control screen brightness")]
pub enum Command {
    Set {
        #[clap(value_name = "BRIGHTNESS", help = "Brightness value to set")]
        value: u32,
    },
    Get,
    GetMax,

}


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

pub fn get_max_brightness(device_path: &str) -> Option<u32> {
    let max_brightness_path = Path::new(device_path).join("max_brightness");
    let content = fs::read_to_string(max_brightness_path).ok()?;
    content.trim().parse().ok()
}

pub fn set_brightness(device_path: &str, value: u32) -> std::io::Result<()> {
    let brightness_path = Path::new(device_path).join("brightness");
    fs::write(brightness_path, value.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_backlight_devices() {
        let my_device_path = Path::new(BRIGHTNESS_PATH)
            .join("intel_backlight")
            .to_str().unwrap()
            .to_string();
        let check_device_path = get_backlight_devices().unwrap();
        assert_eq!(my_device_path, check_device_path);
    }

    #[test]
    fn test_read_brightness() {
        let my_brightness = 150;
        let device_path = get_backlight_devices().unwrap();
        let check_brightness = read_brightness(&device_path).unwrap_or(0);
        assert_eq!(my_brightness, check_brightness);
    }

    #[test]
    fn test_get_max_brightness() {
        let max_brightness = 400;
        let device_path = get_backlight_devices().unwrap();
        let device_max_brightness = get_max_brightness(&device_path).unwrap_or(0);
        assert_eq!(max_brightness, device_max_brightness);
    }

    #[test]
    fn test_set_brightness() {
        let max_brightness: u32 = 400;
        let brightness_150: u32 = 150;
        let device_path = get_backlight_devices().unwrap();
        let result = set_brightness(&device_path, brightness_150);
        assert_eq!(result.unwrap(), ());
    }
}
