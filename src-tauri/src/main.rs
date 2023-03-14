#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello_rust])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn hello_rust() -> &'static str {
  return "Hello from Rust!"
}
