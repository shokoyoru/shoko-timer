# Screen Timer
A simple screen timer with json output for status bars (like waybar).

## Features
- Automatically handles system suspend or lid close.
- Doesn't count sleep time.
- Uses '/dev/shm' for writing and auto-cleanup on reboot for linux
- Uses temp directories for windows/macOS and cleaned every new session
- Outputs JSON format for easy GUI integration.

## Usage
Run the binary and point your status bar to JSON output.
## License
Do whatever you want.