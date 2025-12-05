//! Windows implementation using Win32 GetAsyncKeyState API.

use crate::input_handler::KeyCode;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY};

/// Windows-specific input handler using GetAsyncKeyState.
///
/// This implementation queries the keyboard state on-demand using the Win32 API,
/// so it doesn't need a background thread. The state is always current when queried.
pub struct InputHandler;

impl InputHandler {
    /// Creates a new input handler.
    pub fn new() -> Self {
        InputHandler
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
        let vk = Self::to_virtual_key(key);
        unsafe {
            GetAsyncKeyState(vk.0 as i32) as u16 & 0x8000 != 0
        }
    }

    fn to_virtual_key(key: KeyCode) -> VIRTUAL_KEY {
        VIRTUAL_KEY(match key {
            KeyCode::KeyEsc => 0x1B,
            KeyCode::Key1 => 0x31,
            KeyCode::Key2 => 0x32,
            KeyCode::Key3 => 0x33,
            KeyCode::Key4 => 0x34,
            KeyCode::Key5 => 0x35,
            KeyCode::Key6 => 0x36,
            KeyCode::Key7 => 0x37,
            KeyCode::Key8 => 0x38,
            KeyCode::Key9 => 0x39,
            KeyCode::Key0 => 0x30,
            KeyCode::KeyMinus => 0xBD,
            KeyCode::KeyEqual => 0xBB,
            KeyCode::KeyBackspace => 0x08,
            KeyCode::KeyTab => 0x09,
            KeyCode::KeyQ => 0x51,
            KeyCode::KeyW => 0x57,
            KeyCode::KeyE => 0x45,
            KeyCode::KeyR => 0x52,
            KeyCode::KeyT => 0x54,
            KeyCode::KeyY => 0x59,
            KeyCode::KeyU => 0x55,
            KeyCode::KeyI => 0x49,
            KeyCode::KeyO => 0x4F,
            KeyCode::KeyP => 0x50,
            KeyCode::KeyLeftBrace => 0xDB,
            KeyCode::KeyRightBrace => 0xDD,
            KeyCode::KeyEnter => 0x0D,
            KeyCode::KeyLeftCtrl => 0xA2,
            KeyCode::KeyA => 0x41,
            KeyCode::KeyS => 0x53,
            KeyCode::KeyD => 0x44,
            KeyCode::KeyF => 0x46,
            KeyCode::KeyG => 0x47,
            KeyCode::KeyH => 0x48,
            KeyCode::KeyJ => 0x4A,
            KeyCode::KeyK => 0x4B,
            KeyCode::KeyL => 0x4C,
            KeyCode::KeySemicolon => 0xBA,
            KeyCode::KeyApostrophe => 0xDE,
            KeyCode::KeyGrave => 0xC0,
            KeyCode::KeyLeftShift => 0xA0,
            KeyCode::KeyBackslash => 0xDC,
            KeyCode::KeyZ => 0x5A,
            KeyCode::KeyX => 0x58,
            KeyCode::KeyC => 0x43,
            KeyCode::KeyV => 0x56,
            KeyCode::KeyB => 0x42,
            KeyCode::KeyN => 0x4E,
            KeyCode::KeyM => 0x4D,
            KeyCode::KeyComma => 0xBC,
            KeyCode::KeyDot => 0xBE,
            KeyCode::KeySlash => 0xBF,
            KeyCode::KeyRightShift => 0xA1,
            KeyCode::KeyKpAsterisk => 0x6A,
            KeyCode::KeyLeftAlt => 0xA4,
            KeyCode::KeySpace => 0x20,
            KeyCode::KeyCapslock => 0x14,
            KeyCode::KeyF1 => 0x70,
            KeyCode::KeyF2 => 0x71,
            KeyCode::KeyF3 => 0x72,
            KeyCode::KeyF4 => 0x73,
            KeyCode::KeyF5 => 0x74,
            KeyCode::KeyF6 => 0x75,
            KeyCode::KeyF7 => 0x76,
            KeyCode::KeyF8 => 0x77,
            KeyCode::KeyF9 => 0x78,
            KeyCode::KeyF10 => 0x79,
            KeyCode::KeyF11 => 0x7A,
            KeyCode::KeyF12 => 0x7B,
            KeyCode::KeyUp => 0x26,
            KeyCode::KeyDown => 0x28,
            KeyCode::KeyLeft => 0x25,
            KeyCode::KeyRight => 0x27,
        })
    }
}
