//! Platform-specific input handler implementations and key code definitions.

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub use linux::InputHandler;

#[cfg(target_os = "windows")]
pub use windows::InputHandler;

#[cfg(target_os = "macos")]
pub use macos::InputHandler;

/// Represents a keyboard key that can be queried.
///
/// This enum contains the most commonly used keyboard keys. The key codes are
/// platform-independent, and the library handles the conversion to platform-specific
/// key codes internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    /// Escape key
    KeyEsc,
    /// Number row keys
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    /// Minus key (-)
    KeyMinus,
    /// Equal key (=)
    KeyEqual,
    /// Backspace key
    KeyBackspace,
    /// Tab key
    KeyTab,
    /// Letter keys - First row
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    /// Left bracket key ([)
    KeyLeftBrace,
    /// Right bracket key (])
    KeyRightBrace,
    /// Enter/Return key
    KeyEnter,
    /// Left Control key
    KeyLeftCtrl,
    /// Letter keys - Second row
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    /// Semicolon key (;)
    KeySemicolon,
    /// Apostrophe/Quote key (')
    KeyApostrophe,
    /// Grave/Backtick key (`)
    KeyGrave,
    /// Left Shift key
    KeyLeftShift,
    /// Backslash key (\)
    KeyBackslash,
    /// Letter keys - Third row
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    /// Comma key (,)
    KeyComma,
    /// Period/Dot key (.)
    KeyDot,
    /// Slash key (/)
    KeySlash,
    /// Right Shift key
    KeyRightShift,
    /// Keypad asterisk key (*)
    KeyKpAsterisk,
    /// Left Alt/Option key
    KeyLeftAlt,
    /// Space bar
    KeySpace,
    /// Caps Lock key
    KeyCapslock,
    /// Function keys
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    /// Arrow keys
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
}
