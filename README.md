# Input Query

A cross-platform Rust library for querying keyboard key states without requiring a window context.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
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

## Platform-Specific Setup

### Linux

On Linux, you need read access to input devices.

Add your user to the `input` group:
   ```bash
   sudo usermod -aG input $USER
   ```
   Then log out and log back in.

### macOS

On macOS, the application needs "Input Monitoring" permission:

1. Run your application
2. When prompted, or manually go to: **System Preferences → Security & Privacy → Privacy → Input Monitoring**
3. Add and enable your application

### Windows

No special setup required on Windows.