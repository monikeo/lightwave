use lightwave::*;
use clap::Parser;

fn set_brightness_command(device_path: &str, brightness_state: &mut u32, value: u32, max_brightness: u32) {

}

fn adjust_brightness(device_path: &str, brightness_state: &mut u32, value: u32, max_brightness: u32, increase: bool) {

}

fn handle_command(command: Command, device_path: &str, brightness_state: &mut u32, max_brightness: u32) {
    match command {
        Command::Set{value} => {},
        Command::Get => {
            println!("[+] Current Brightness: {}", brightness_state);
        },
        Command::GetMax => {
            println!("[+] Max Brightness: {}", max_brightness);
        },
        Command::Increase{value} => {},
        Command::Decrease{value} => {},
    }
}

fn main() {
    let command = Command::parse();

    // // Find backlight device path
    let device_path = match get_backlight_devices() {
        Some(path) => path,
        None => {
            eprintln!(" [-] ERROR: No backlight device found!");
            return;
        }
    };

    // Read current brightness state
    let mut brightness_state = match read_brightness(&device_path) {
        Some(brightness) => brightness,
        None => {
            eprintln!(" [-] ERROR: Failed to read brightness.");
            return;
        }
    };

    // Get maximum brightness
    let max_brightness = match get_max_brightness(&device_path) {
        Some(max_brightness) => max_brightness,
        None => {
            eprintln!(" [-] ERROR: Failed to read maximum brightness.");
            return;
        }
    };

    match command {
        Command::Set{value} => {
            if value < 0 {return}
            if value < max_brightness && value != brightness_state {
                if let Err(err) = set_brightness(&device_path, value) {
                    eprintln!(" [-] ERROR: Failed to set brightness: {}", value);
                } else {
                    brightness_state = value;
                    println!(" [+] Brightness set to {}", value);
                }
            }
        },
        Command::Get => {
            println!(" [+] Current Brightness {}", brightness_state);
        },
        Command::GetMax => {
            println!(" [+] Max Brightness {}", max_brightness);
        },
        Command::Increase{value} => {
            let total_value = value + brightness_state;
            if total_value >= max_brightness {
                let status = set_brightness(&device_path, max_brightness);                    
                if status.is_ok() {
                    brightness_state = max_brightness;
                }
            } else if total_value != brightness_state {
                let status = set_brightness(&device_path, total_value);
                if status.is_ok() {
                    brightness_state = total_value;
                }
            }
        },
        Command::Decrease{value} => {
            let total_value = brightness_state - value;
            if total_value < 0 {return}
            if total_value != brightness_state && total_value < max_brightness {
                let status = set_brightness(&device_path, total_value);
                if status.is_ok() {
                    brightness_state = total_value;
                }
            }
        },
    }
}
