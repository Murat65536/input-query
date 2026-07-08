use input_query::{InputHandler, KeyCode};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Press esc to quit");

    let handler = InputHandler::new();

    loop {
        if handler.is_pressed(KeyCode::KeyEsc) {
            break;
        }

        if handler.is_pressed(KeyCode::KeySpace) {
            println!("Space bar");
        }

        if handler.is_pressed(KeyCode::KeyW) {
            println!("W key");
        }

        if handler.is_pressed(KeyCode::KeyA) {
            println!("A key");
        }

        if handler.is_pressed(KeyCode::KeyS) {
            println!("S key");
        }

        if handler.is_pressed(KeyCode::KeyD) {
            println!("D key");
        }

        thread::sleep(Duration::from_millis(50));
    }
}
