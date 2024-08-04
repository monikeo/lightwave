use lightwave::*;
use clap::Parser;

fn main() {
    let command = Command::parse();
    let device_path = match get_backlight_devices() {
        Some(path) => path,
        None => {
            eprintln!(" [-] ERROR: No backlight device found!");
            return;
        }
    };

    let mut brightness_state = match read_brightness(&device_path) {
        Some(brightness) => brightness,
        None => {
            eprintln!(" [-] ERROR: Failed to read brightness.");
            return;
        }
    };

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
            if total_value != brightness_state && total_value < max_brightness {

            }
        },
        Command::Decrease{value} => {
            let total_value = brightness_state - value;
            if total_value < 0 {return}
            if total_value != brightness_state && total_value < max_brightness {

            }
        },
    }
}
