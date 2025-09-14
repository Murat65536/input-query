use evdev::{self, KeyCode, EventSummary};
use std::{io::ErrorKind, thread};

const KEY_COUNT: usize = 0x300;

pub struct InputHandler {
    devices: Vec<evdev::Device>,
    pressed_keys: [bool;KEY_COUNT],
}

impl InputHandler {
    pub fn new() -> Self {
        let devices: Vec<_> = evdev::enumerate().map(|(_, device)| {
            device.set_nonblocking(true).expect("Failed to set non blocking on device");
            device
        }).collect();
        InputHandler {
            devices: devices,
            pressed_keys: [false;KEY_COUNT],
        }
    }

    pub fn update_inputs(&mut self) {
        for device in &mut self.devices {
            match device.fetch_events() {
                Ok(events) => {
                    for event in events {
                        match event.destructure() {
                            EventSummary::Key(_, key_type, 1) => {
                                self.pressed_keys[key_type.code() as usize] = true;
                            }
                            EventSummary::Key(_, key_type, 0) => {
                                self.pressed_keys[key_type.code() as usize] = false;
                            },
                            _ => {}
                        }
                    }
                    
                },
                Err(err) => {
                    if err.kind() != ErrorKind::WouldBlock {
                        panic!("{}", err);
                    }
                }
            }
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys[key.code() as usize]
    }
}

