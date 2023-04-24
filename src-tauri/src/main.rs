use std::ops::{Mul, Not, Shl};

pub struct State {
    last_duet: Option<u8>,
    current_char: u32,
    pressing: bool,
}

static mut STATE: State = State {
    last_duet: None,
    current_char: 0,
    pressing: false,
};

const COMMANDS: &[u8] = &[];

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set(index: u8, value: u8) {
    println!["setting {index}{value}"];
    unsafe {
        STATE.last_duet = Some(index);
    }
    unsafe {
        STATE.current_char &= 3u32.shl(index).not();
        STATE.current_char |= value.shl(index.mul(2)) as u32;
    }
}

#[tauri::command]
fn exited(index: u8) {
    if let Some(last_index) = unsafe { STATE.last_duet } {
        if index == last_index {
            if unsafe { STATE.current_char as usize } < COMMANDS.len() {
                unsafe {
                    STATE.pressing = true;
                }
                // emit command
                println!["emitting command"];
            } else {
                if let Ok(c) = char::try_from(unsafe { STATE.current_char }) {
                    unsafe {
                        STATE.pressing = true;
                    }
                    // emit char
                    println!["{c}"];
                } else {
                    unsafe {
                        STATE = State {
                            last_duet: None,
                            current_char: 0,
                            pressing: false,
                        };
                    }
                }
            }
        }
    }
}

#[tauri::command]
fn reset() {
    unsafe {
        STATE = State {
            last_duet: None,
            current_char: 0,
            pressing: false,
        };
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set, exited, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
