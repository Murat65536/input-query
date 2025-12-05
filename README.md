# input_query

A cross-platform Rust library for querying keyboard key states without requiring a window context.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🖥️ **Cross-platform**: Supports Linux, Windows, and macOS
- 🚀 **No window required**: Query keyboard state directly from the OS
- 🎯 **Simple API**: Easy-to-use interface with minimal boilerplate
- 🔧 **Platform-optimized**: Uses native APIs for best performance on each platform

## Platform Support

| Platform | API Used | Background Thread | Notes |
|----------|----------|-------------------|-------|
| **Linux** | `evdev` | Yes (5ms poll rate) | Requires read access to `/dev/input/event*` devices |
| **Windows** | `GetAsyncKeyState` (Win32) | No (on-demand) | No special permissions required |
| **macOS** | `CGEventSourceKeyState` (Core Graphics) | No (on-demand) | Requires "Input Monitoring" permission |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
input_query = "0.1.0"
```

## Usage

```rust
use input_query::{InputHandler, KeyCode};
use std::thread;
use std::time::Duration;

fn main() {
    let handler = InputHandler::new();

    loop {
        if handler.is_pressed(KeyCode::KeyEsc) {
            println!("Escape key is pressed!");
            break;
        }
        
        if handler.is_pressed(KeyCode::KeySpace) {
            println!("Space bar is pressed!");
        }
        
        // Your application logic here
        thread::sleep(Duration::from_millis(10));
    }
}
```

**Note**: On Linux, input events are monitored in a background thread automatically. On Windows and macOS, the state is queried on-demand. You no longer need to call `update_inputs()` - the library handles updates automatically!

### Example

You can run the included example with:

```bash
cargo run --example simple
```

This will demonstrate real-time key detection. Press ESC to exit the example.

## Platform-Specific Setup

### Linux

On Linux, you need read access to input devices. You can either:

1. Add your user to the `input` group:
   ```bash
   sudo usermod -a -G input $USER
   ```
   Then log out and log back in.

2. Or run your application with appropriate permissions (not recommended for production).

### macOS

On macOS, the application needs "Input Monitoring" permission:

1. Run your application
2. When prompted, or manually go to: **System Preferences → Security & Privacy → Privacy → Input Monitoring**
3. Add and enable your application

### Windows

No special setup required on Windows.

## Supported Keys

The library currently supports:
- All letter keys (A-Z)
- Number keys (0-9)
- Function keys (F1-F12)
- Modifier keys (Shift, Ctrl, Alt, Caps Lock)
- Arrow keys
- Common symbol keys (brackets, punctuation, etc.)
- Space, Enter, Backspace, Tab, Escape

See the [`KeyCode`](https://docs.rs/input_query/latest/input_query/input_handler/enum.KeyCode.html) enum for the complete list.

## API Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

Or visit [docs.rs](https://docs.rs/input_query) (once published).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Linux implementation uses the excellent [evdev](https://crates.io/crates/evdev) crate
- Windows implementation uses the [windows](https://crates.io/crates/windows) crate
- macOS implementation uses the [core-graphics](https://crates.io/crates/core-graphics) crate

