mod priority;
mod process;
mod rules;
mod watcher;

use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};

struct AppState {
    rules: Arc<Mutex<Vec<rules::Rule>>>,
    path: std::path::PathBuf,
}

#[derive(serde::Serialize)]
struct ProcessDto {
    pid: u32,
    parent_pid: u32,
    name: String,
}

#[tauri::command]
fn list_running_processes() -> Result<Vec<ProcessDto>, String> {
    process::list_processes()
        .map(|v| {
            v.into_iter()
                .map(|p| ProcessDto { pid: p.pid, parent_pid: p.parent_pid, name: p.name })
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_rules(state: tauri::State<AppState>) -> Vec<rules::Rule> {
    state.rules.lock().unwrap().clone()
}

#[tauri::command]
fn save_rules(list: Vec<rules::Rule>, state: tauri::State<AppState>) -> Result<(), String> {
    rules::save(&state.path, &list).map_err(|e| e.to_string())?;
    *state.rules.lock().unwrap() = list;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let path = app.path().app_config_dir()?.join("rules.json");
            let rules = Arc::new(Mutex::new(rules::load(&path)));
            watcher::start(rules.clone());
            app.manage(AppState { rules, path });

            let show = MenuItem::with_id(app, "show", "Open", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, e| match e.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|w, e| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                api.prevent_close();
                let _ = w.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            list_running_processes,
            list_rules,
            save_rules
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}