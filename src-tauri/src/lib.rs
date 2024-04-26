#[tauri::command]
fn minimize_window(window: tauri::Window) {
    println!("minimize_window");
    window.minimize().unwrap_or_default();
}

#[tauri::command]
fn maximize_window(window: tauri::Window) {
    println!("maximize_window");
    if window.is_maximized().unwrap_or_default() {
        window.unmaximize().unwrap_or_default();
    } else {
        window.maximize().unwrap_or_default();
    }
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    println!("close_window");
    window.close().unwrap_or_default();
}

#[tauri::command]
fn is_maximized(window: tauri::Window) -> String {
    println!("close_window");
    window.is_maximized().unwrap_or_default().to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            maximize_window,
            close_window,
            is_maximized,
            minimize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
