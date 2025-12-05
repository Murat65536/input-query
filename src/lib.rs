//! # input_query
//!
//! A cross-platform Rust library for querying keyboard key states without requiring a window context.
//!
//! This library provides a simple, unified API for checking if keyboard keys are currently pressed
//! across Linux, Windows, and macOS platforms.
//!
//! ## Platform Support
//!
//! - **Linux**: Uses `evdev` to directly read input events from device files
//! - **Windows**: Uses `GetAsyncKeyState` from the Win32 API
//! - **macOS**: Uses `CGEventSourceKeyState` from the Core Graphics framework
//!
//! ## Usage
//!
//! ```no_run
//! use input_query::{InputHandler, KeyCode};
//!
//! let mut handler = InputHandler::new();
//!
//! loop {
//!     handler.update_inputs();
//!     
//!     if handler.is_pressed(KeyCode::KeyEsc) {
//!         println!("Escape key is pressed!");
//!         break;
//!     }
//!     
//!     if handler.is_pressed(KeyCode::KeySpace) {
//!         println!("Space bar is pressed!");
//!     }
//! }
//! ```
//!
//! ## Note on Permissions
//!
//! - **Linux**: Requires read access to `/dev/input/event*` devices. You may need to add your user
//!   to the `input` group or run with appropriate permissions.
//! - **macOS**: Requires accessibility permissions. The application may need to be granted
//!   "Input Monitoring" permission in System Preferences → Security & Privacy → Privacy.
//! - **Windows**: No special permissions required.

pub mod input_handler;
pub use input_handler::{InputHandler, KeyCode};
