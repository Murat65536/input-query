//! # input_query
//!
//! A cross-platform Rust library for querying keyboard key states without requiring a window context.
//!
//! ## Usage
//!
//! ```no_run
//! use input_query::{InputHandler, KeyCode};
//! use std::thread;
//! use std::time::Duration;
//!
//! let handler = InputHandler::new();
//!
//! loop {
//!     if handler.is_pressed(KeyCode::KeyEsc) {
//!         println!("Escape key is pressed!");
//!         break;
//!     }
//!     
//!     if handler.is_pressed(KeyCode::KeySpace) {
//!         println!("Space bar is pressed!");
//!     }
//!     
//!     thread::sleep(Duration::from_millis(10));
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
