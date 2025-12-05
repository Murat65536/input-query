//! Linux implementation using evdev for direct input device access.

use evdev::{self, EventSummary};
use std::io::ErrorKind;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use parking_lot::Mutex;
use crate::input_handler::KeyCode;

const KEY_COUNT: usize = 0x300;

struct SharedState {
    pressed_keys: [bool; KEY_COUNT],
}

/// Linux-specific input handler that reads from evdev devices.
///
/// This implementation enumerates all available input devices and monitors
/// their key events in a background thread. It requires read access to 
/// `/dev/input/event*` devices.
///
/// The background thread polls input devices every 5ms to keep the key state updated.
pub struct InputHandler {
    state: Arc<Mutex<SharedState>>,
    _thread_handle: Option<thread::JoinHandle<()>>,
}

impl InputHandler {
    /// Creates a new input handler and starts a background thread to monitor input devices.
    ///
    /// The background thread will automatically poll input devices every 5ms until
    /// the `InputHandler` is dropped.
    pub fn new() -> Self {
        let devices: Vec<_> = evdev::enumerate().map(|(_, device)| {
            device.set_nonblocking(true).expect("Failed to set non blocking on device");
            device
        }).collect();

        let state = Arc::new(Mutex::new(SharedState {
            pressed_keys: [false; KEY_COUNT],
        }));

        let state_clone = Arc::clone(&state);
        let thread_handle = thread::spawn(move || {
            Self::input_thread(devices, state_clone);
        });

        InputHandler {
            state,
            _thread_handle: Some(thread_handle),
        }
    }

    fn input_thread(mut devices: Vec<evdev::Device>, state: Arc<Mutex<SharedState>>) {
        loop {
            let mut state_guard = state.lock();
            for device in &mut devices {
                match device.fetch_events() {
                    Ok(events) => {
                        for event in events {
                            match event.destructure() {
                                EventSummary::Key(_, key_type, 1) => {
                                    state_guard.pressed_keys[key_type.code() as usize] = true;
                                }
                                EventSummary::Key(_, key_type, 0) => {
                                    state_guard.pressed_keys[key_type.code() as usize] = false;
                                },
                                _ => {}
                            }
                        }
                    },
                    Err(err) => {
                        if err.kind() != ErrorKind::WouldBlock {
                            eprintln!("Input device error: {}", err);
                        }
                    }
                }
            }
            drop(state_guard);
            thread::sleep(Duration::from_millis(5));
        }
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
        let evdev_code = Self::to_evdev_code(key);
        let state = self.state.lock();
        state.pressed_keys[evdev_code as usize]
    }

    fn to_evdev_code(key: KeyCode) -> u16 {
        use evdev::KeyCode as EvKeyCode;
        match key {
            KeyCode::KeyEsc => EvKeyCode::KEY_ESC.code(),
            KeyCode::Key1 => EvKeyCode::KEY_1.code(),
            KeyCode::Key2 => EvKeyCode::KEY_2.code(),
            KeyCode::Key3 => EvKeyCode::KEY_3.code(),
            KeyCode::Key4 => EvKeyCode::KEY_4.code(),
            KeyCode::Key5 => EvKeyCode::KEY_5.code(),
            KeyCode::Key6 => EvKeyCode::KEY_6.code(),
            KeyCode::Key7 => EvKeyCode::KEY_7.code(),
            KeyCode::Key8 => EvKeyCode::KEY_8.code(),
            KeyCode::Key9 => EvKeyCode::KEY_9.code(),
            KeyCode::Key0 => EvKeyCode::KEY_0.code(),
            KeyCode::KeyMinus => EvKeyCode::KEY_MINUS.code(),
            KeyCode::KeyEqual => EvKeyCode::KEY_EQUAL.code(),
            KeyCode::KeyBackspace => EvKeyCode::KEY_BACKSPACE.code(),
            KeyCode::KeyTab => EvKeyCode::KEY_TAB.code(),
            KeyCode::KeyQ => EvKeyCode::KEY_Q.code(),
            KeyCode::KeyW => EvKeyCode::KEY_W.code(),
            KeyCode::KeyE => EvKeyCode::KEY_E.code(),
            KeyCode::KeyR => EvKeyCode::KEY_R.code(),
            KeyCode::KeyT => EvKeyCode::KEY_T.code(),
            KeyCode::KeyY => EvKeyCode::KEY_Y.code(),
            KeyCode::KeyU => EvKeyCode::KEY_U.code(),
            KeyCode::KeyI => EvKeyCode::KEY_I.code(),
            KeyCode::KeyO => EvKeyCode::KEY_O.code(),
            KeyCode::KeyP => EvKeyCode::KEY_P.code(),
            KeyCode::KeyLeftBrace => EvKeyCode::KEY_LEFTBRACE.code(),
            KeyCode::KeyRightBrace => EvKeyCode::KEY_RIGHTBRACE.code(),
            KeyCode::KeyEnter => EvKeyCode::KEY_ENTER.code(),
            KeyCode::KeyLeftCtrl => EvKeyCode::KEY_LEFTCTRL.code(),
            KeyCode::KeyA => EvKeyCode::KEY_A.code(),
            KeyCode::KeyS => EvKeyCode::KEY_S.code(),
            KeyCode::KeyD => EvKeyCode::KEY_D.code(),
            KeyCode::KeyF => EvKeyCode::KEY_F.code(),
            KeyCode::KeyG => EvKeyCode::KEY_G.code(),
            KeyCode::KeyH => EvKeyCode::KEY_H.code(),
            KeyCode::KeyJ => EvKeyCode::KEY_J.code(),
            KeyCode::KeyK => EvKeyCode::KEY_K.code(),
            KeyCode::KeyL => EvKeyCode::KEY_L.code(),
            KeyCode::KeySemicolon => EvKeyCode::KEY_SEMICOLON.code(),
            KeyCode::KeyApostrophe => EvKeyCode::KEY_APOSTROPHE.code(),
            KeyCode::KeyGrave => EvKeyCode::KEY_GRAVE.code(),
            KeyCode::KeyLeftShift => EvKeyCode::KEY_LEFTSHIFT.code(),
            KeyCode::KeyBackslash => EvKeyCode::KEY_BACKSLASH.code(),
            KeyCode::KeyZ => EvKeyCode::KEY_Z.code(),
            KeyCode::KeyX => EvKeyCode::KEY_X.code(),
            KeyCode::KeyC => EvKeyCode::KEY_C.code(),
            KeyCode::KeyV => EvKeyCode::KEY_V.code(),
            KeyCode::KeyB => EvKeyCode::KEY_B.code(),
            KeyCode::KeyN => EvKeyCode::KEY_N.code(),
            KeyCode::KeyM => EvKeyCode::KEY_M.code(),
            KeyCode::KeyComma => EvKeyCode::KEY_COMMA.code(),
            KeyCode::KeyDot => EvKeyCode::KEY_DOT.code(),
            KeyCode::KeySlash => EvKeyCode::KEY_SLASH.code(),
            KeyCode::KeyRightShift => EvKeyCode::KEY_RIGHTSHIFT.code(),
            KeyCode::KeyKpAsterisk => EvKeyCode::KEY_KPASTERISK.code(),
            KeyCode::KeyLeftAlt => EvKeyCode::KEY_LEFTALT.code(),
            KeyCode::KeySpace => EvKeyCode::KEY_SPACE.code(),
            KeyCode::KeyCapslock => EvKeyCode::KEY_CAPSLOCK.code(),
            KeyCode::KeyF1 => EvKeyCode::KEY_F1.code(),
            KeyCode::KeyF2 => EvKeyCode::KEY_F2.code(),
            KeyCode::KeyF3 => EvKeyCode::KEY_F3.code(),
            KeyCode::KeyF4 => EvKeyCode::KEY_F4.code(),
            KeyCode::KeyF5 => EvKeyCode::KEY_F5.code(),
            KeyCode::KeyF6 => EvKeyCode::KEY_F6.code(),
            KeyCode::KeyF7 => EvKeyCode::KEY_F7.code(),
            KeyCode::KeyF8 => EvKeyCode::KEY_F8.code(),
            KeyCode::KeyF9 => EvKeyCode::KEY_F9.code(),
            KeyCode::KeyF10 => EvKeyCode::KEY_F10.code(),
            KeyCode::KeyF11 => EvKeyCode::KEY_F11.code(),
            KeyCode::KeyF12 => EvKeyCode::KEY_F12.code(),
            KeyCode::KeyUp => EvKeyCode::KEY_UP.code(),
            KeyCode::KeyDown => EvKeyCode::KEY_DOWN.code(),
            KeyCode::KeyLeft => EvKeyCode::KEY_LEFT.code(),
            KeyCode::KeyRight => EvKeyCode::KEY_RIGHT.code(),
        }
    }
}

