use std::{
    ops::{Mul, Not, Shl},
    sync::Mutex,
};

// const COMMANDS: &[u8] = &[];

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Default)]
struct Utf8Codepoint {
    char_buf: Vec<u8>,
}

impl Utf8Codepoint {
    pub fn is_valid(&self) -> bool {
        match self.char_buf.as_slice() {
            [0..=127] => true,                                    // 1 byte
            [192..=223, 128..=191] => true,                       // 2 byte
            [224..=239, 128..=191, 128..=191] => true,            // 3 byte
            [240..=247, 128..=191, 128..=191, 128..=191] => true, // 4 byte
            _ => false,
        }
    }

    pub fn to_str(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.char_buf.clone()) }
    }

    // pub fn to_u32(&self) -> u32 {
    //     let mut a = [0; 4];
    //     for (i, b) in self.char_buf.iter().enumerate() {
    //         a[i] = *b;
    //     }
    //     u32::from_be_bytes(a)
    // }

    pub fn current_byte(&mut self) -> Option<&mut u8> {
        self.char_buf.last_mut()
    }

    pub fn push(&mut self, b: u8) {
        self.char_buf.push(b);
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn update_current_byte(&mut self, column: u8, row: u8) {
        let b = if let Some(b) = self.current_byte() {
            b
        } else {
            self.char_buf.push(row.shl(column.mul(2)));
            return;
        };
        *b &= 3u8.shl(column.mul(2)).not();
        *b |= row.shl(column.mul(2));
    }
}

#[derive(Default)]
struct KeyboardState {
    codepoint: Utf8Codepoint,
    pressed: Option<String>,
}

#[derive(Default)]
struct AppState {
    kb_state: Mutex<KeyboardState>,
}

#[tauri::command]
fn entered_key(column: u8, row: u8, state: tauri::State<AppState>) {
    assert![column < 4];
    assert![row < 4];
    let mut kb_state = state.kb_state.lock().unwrap();
    kb_state.pressed = None;
    kb_state.codepoint.update_current_byte(column, row);
}

#[tauri::command]
fn entered_end(state: tauri::State<AppState>) {
    let mut kb_state = state.kb_state.lock().unwrap();
    if kb_state.codepoint.is_valid() {
        kb_state.pressed = Some(kb_state.codepoint.to_str());
        kb_state.codepoint.reset();
    } else if kb_state.codepoint.char_buf.len() < 4 {
        kb_state.codepoint.push(0);
    } else {
        kb_state.codepoint.reset();
    }
}

#[tauri::command]
fn lifted(state: tauri::State<AppState>) {
    let mut kb_state = state.kb_state.lock().unwrap();
    kb_state.pressed = None;
    kb_state.codepoint.reset();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![entered_key, entered_end, lifted])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
