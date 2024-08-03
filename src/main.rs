
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
    match get_max_brightness(&device_path) {
        Some(max_brightness) => {
            match command {
                Command::Set{value} => {
                    if let Err(err) = set_brightness(&device_path, value) {
                        eprintln!(" [-] ERROR: Failed to set brightness: {}", value);
                    } else {
                        println!(" [+] Brightness set to {}", value);    
                    }
                },
                Command::Get => {
                    match read_brightness(&device_path) {
                        Some(brightness) => {
                            println!(" [+] Current Brightness {}", brightness);
                        },
                        None => {
                            eprintln!(" [-] ERROR: Failed to read brightness.");
                        }
                    }
                },
                Command::GetMax => {
                    println!(" [+] Max Brightness {}", max_brightness);
                },
                Command::Increase{value} => {},
                Command::Descrease{value} => {},
            }
        },
        None => {
            eprintln!(" [-] ERROR: Failed to read maximum brightness.");
        }
    }
}
