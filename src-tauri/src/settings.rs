use tauri::Manager;

#[tauri::command]
pub async fn open_settings_window(app: tauri::AppHandle) {
    let settings_window = app.get_webview_window("kumori-settings").unwrap();
    settings_window.show();
}
