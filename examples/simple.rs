use input_query::{InputHandler, KeyCode};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Input Query Example");
    println!("Press ESC to exit");
    println!("---");

    let handler = InputHandler::new();

    loop {
        if handler.is_pressed(KeyCode::KeyEsc) {
            println!("Escape key pressed! Exiting...");
            break;
        }

        if handler.is_pressed(KeyCode::KeySpace) {
            println!("Space bar is pressed!");
        }

        if handler.is_pressed(KeyCode::KeyW) {
            println!("W key is pressed!");
        }

        if handler.is_pressed(KeyCode::KeyA) {
            println!("A key is pressed!");
        }

        if handler.is_pressed(KeyCode::KeyS) {
            println!("S key is pressed!");
        }

        if handler.is_pressed(KeyCode::KeyD) {
            println!("D key is pressed!");
        }

        thread::sleep(Duration::from_millis(50));
    }
}
