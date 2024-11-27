use tauri::Manager;

#[tauri::command]
pub async fn open_settings_window(app: tauri::AppHandle) {
    let settings_window = app.get_webview_window("kumori-settings").unwrap();
    let _ = settings_window.show();
    let _ = settings_window.set_focus();
}
