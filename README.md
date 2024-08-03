## Lightwave
Lightwave is a lightweight command-line tool designed to manage screen brightness on Linux systems. Written in Rust, Lightwave allows users to easily adjust the brightness of their screens through a clean and efficient interface.

Features:
- View current brightness level.
- Set a new brightness level within the allowable range.
- Easy to use with simple commands.

Lightwave supports single backlight devices typically found on most Linux distributions, including Arch Linux. It provides a straightforward way to control screen brightness from the terminal.

## Installation
To build and install Lightwave, you'll need to have Rust installed. Follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/monikeo281000/lightwave.git
   cd lightwave
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the tool (you may need sudo to adjust brightness):
   ```bash
   sudo ./target/release/lightwave
   ```

## Usage
Run lightwave to view the current brightness and maximum allowable brightness. You can also set a new brightness level:
```bash
sudo lightwave --set <value>
```
Where <value> is the desired brightness level.

For more info
```bash
lightware --help
```



