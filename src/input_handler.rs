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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    KeyEsc,
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
    KeyMinus,
    KeyEqual,
    KeyBackspace,
    KeyTab,
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
    KeyLeftBrace,
    KeyRightBrace,
    KeyEnter,
    KeyLeftCtrl,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    KeySemicolon,
    KeyApostrophe,
    KeyGrave,
    KeyLeftShift,
    KeyBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    KeyComma,
    KeyDot,
    KeySlash,
    KeyRightShift,
    KeyKpAsterisk,
    KeyLeftAlt,
    KeySpace,
    KeyCapslock,
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
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
}
