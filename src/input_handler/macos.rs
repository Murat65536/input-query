//! macOS implementation using Core Graphics Event APIs.

use crate::input_handler::KeyCode;
use core_graphics::event::{CGEventSourceStateID, CGEventType, EventField};
use core_graphics::event_source::CGEventSource;

/// macOS-specific input handler using CGEventSourceKeyState.
///
/// This implementation queries the keyboard state on-demand using the Core Graphics
/// framework, so it doesn't maintain any internal state that needs updating.
///
/// # Permissions
///
/// This requires "Input Monitoring" permission on macOS. You may need to grant
/// this permission in System Preferences → Security & Privacy → Privacy → Input Monitoring.
pub struct InputHandler;

impl InputHandler {
    /// Creates a new input handler.
    pub fn new() -> Self {
        InputHandler
    }

    /// Updates the internal key state.
    ///
    /// On macOS, this is a no-op since CGEventSourceKeyState queries the current
    /// state directly each time `is_pressed` is called.
    pub fn update_inputs(&mut self) {
        // macOS uses CGEventSourceKeyState which queries state on demand
        // No explicit update needed
    }

    /// Checks if a specific key is currently pressed.
    ///
    /// # Arguments
    ///
    /// * `key` - The key code to check
    ///
    /// # Returns
    ///
    /// `true` if the key is currently pressed, `false` otherwise.
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        let keycode = Self::to_keycode(key);
        unsafe {
            core_graphics::event::CGEventSourceKeyState(
                CGEventSourceStateID::CombinedSessionState,
                keycode
            )
        }
    }

    fn to_keycode(key: KeyCode) -> u16 {
        // macOS keycodes based on Carbon/HIToolbox
        match key {
            KeyCode::KeyEsc => 0x35,
            KeyCode::Key1 => 0x12,
            KeyCode::Key2 => 0x13,
            KeyCode::Key3 => 0x14,
            KeyCode::Key4 => 0x15,
            KeyCode::Key5 => 0x17,
            KeyCode::Key6 => 0x16,
            KeyCode::Key7 => 0x1A,
            KeyCode::Key8 => 0x1C,
            KeyCode::Key9 => 0x19,
            KeyCode::Key0 => 0x1D,
            KeyCode::KeyMinus => 0x1B,
            KeyCode::KeyEqual => 0x18,
            KeyCode::KeyBackspace => 0x33,
            KeyCode::KeyTab => 0x30,
            KeyCode::KeyQ => 0x0C,
            KeyCode::KeyW => 0x0D,
            KeyCode::KeyE => 0x0E,
            KeyCode::KeyR => 0x0F,
            KeyCode::KeyT => 0x11,
            KeyCode::KeyY => 0x10,
            KeyCode::KeyU => 0x20,
            KeyCode::KeyI => 0x22,
            KeyCode::KeyO => 0x1F,
            KeyCode::KeyP => 0x23,
            KeyCode::KeyLeftBrace => 0x21,
            KeyCode::KeyRightBrace => 0x1E,
            KeyCode::KeyEnter => 0x24,
            KeyCode::KeyLeftCtrl => 0x3B,
            KeyCode::KeyA => 0x00,
            KeyCode::KeyS => 0x01,
            KeyCode::KeyD => 0x02,
            KeyCode::KeyF => 0x03,
            KeyCode::KeyG => 0x05,
            KeyCode::KeyH => 0x04,
            KeyCode::KeyJ => 0x26,
            KeyCode::KeyK => 0x28,
            KeyCode::KeyL => 0x25,
            KeyCode::KeySemicolon => 0x29,
            KeyCode::KeyApostrophe => 0x27,
            KeyCode::KeyGrave => 0x32,
            KeyCode::KeyLeftShift => 0x38,
            KeyCode::KeyBackslash => 0x2A,
            KeyCode::KeyZ => 0x06,
            KeyCode::KeyX => 0x07,
            KeyCode::KeyC => 0x08,
            KeyCode::KeyV => 0x09,
            KeyCode::KeyB => 0x0B,
            KeyCode::KeyN => 0x2D,
            KeyCode::KeyM => 0x2E,
            KeyCode::KeyComma => 0x2B,
            KeyCode::KeyDot => 0x2F,
            KeyCode::KeySlash => 0x2C,
            KeyCode::KeyRightShift => 0x3C,
            KeyCode::KeyKpAsterisk => 0x43,
            KeyCode::KeyLeftAlt => 0x3A,
            KeyCode::KeySpace => 0x31,
            KeyCode::KeyCapslock => 0x39,
            KeyCode::KeyF1 => 0x7A,
            KeyCode::KeyF2 => 0x78,
            KeyCode::KeyF3 => 0x63,
            KeyCode::KeyF4 => 0x76,
            KeyCode::KeyF5 => 0x60,
            KeyCode::KeyF6 => 0x61,
            KeyCode::KeyF7 => 0x62,
            KeyCode::KeyF8 => 0x64,
            KeyCode::KeyF9 => 0x65,
            KeyCode::KeyF10 => 0x6D,
            KeyCode::KeyF11 => 0x67,
            KeyCode::KeyF12 => 0x6F,
            KeyCode::KeyUp => 0x7E,
            KeyCode::KeyDown => 0x7D,
            KeyCode::KeyLeft => 0x7B,
            KeyCode::KeyRight => 0x7C,
        }
    }
}
