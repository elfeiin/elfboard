// fn main() {
//     tauri::Builder::default()
//         .manage(AppState::default())
//         .invoke_handler(tauri::generate_handler![entered_key, entered_end, lifted])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
pub fn main() {
    elfboard::AppBuilder::new().run();
}
