## Lightwave
Lightwave is a lightweight command-line tool designed to manage screen brightness on Linux systems. Written in Rust, Lightwave allows users to easily adjust the brightness of their screens through a clean and efficient interface.

Features:
- View current brightness level.
- Set a new brightness level within the allowable range.
- Increase or decrease brightness by a specified amount.
- Simple and intuitive command usage.

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

3. Install the tool
   ```bash
   sudo cp target/release/lightwave /usr/local/bin/
   ```

3. Run the tool (you may need sudo to adjust brightness):
   ```bash
   sudo lightwave [option]
   ```

## Usage
Lightwave provides several commands to control screen brightness. You may need sudo to adjust the brightness.

1. View the current brightness level
   ```bash
   lightwave get
   ```

2. View the maximum allowable brightness level
   ```bash
   lightwave get-max
   ```

3. Set a new brightness level
   ```bash
   sudo lightwave set <value>
   ```

4. Increase the brightness by a specified amount
   ```bash
   sudo lightwave increase <value>
   ```

5. Decrease the brightness by a specified amount
   ```bash
   sudo lightwave decrease <value>
   ```
Where <value> is the amount to decrease the brightness by.

For more information and additional commands
```bash
lightwave --help
```

## Example Hyprland Setup
To integrate Lightwave with Hyprland for brightness control via keyboard shortcuts, add the following lines to your Hyprland configuration file:
```bash
binde = , XF86MonBrightnessUp, exec, lightwave increase 4
binde = , XF86MonBrightnessDown, exec, lightwave decrease 4
```
These bindings will allow you to increase or decrease the brightness using the respective function keys.
```bash
bind supports flags in this format
e -> repeat, will repeat when held.
```

## Permission Issues
If you encounter permission issues when trying to control the brightness, please check the ownership and permissions of the Lightwave binary

1. Check ownership and permissions
   ```bash
   ls -l /usr/local/bin/lightwave
   ```
   Ensure the owner is root and the permissions are set correctly

2. Set the correct permissions
   ```bash
   sudo chown root:root /usr/local/bin/lightwave
   sudo chmod u+s /usr/local/bin/lightwave
   ```
   This sets the setuid bit, allowing the binary to run with root privileges.

3. If the issue persists
   ```bash
   sudo chmod u+x /usr/local/bin/lightwave
   ```
   This ensures the binary is executable.

   
## License
Lightwave is licensed under the MIT License. See the LICENSE file for more information.

