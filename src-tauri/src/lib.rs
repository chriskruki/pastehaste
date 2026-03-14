mod address;
mod hotkeys;
mod input;
pub mod slots;

use address::AddressState;
use slots::AppState;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager, WindowEvent,
};

#[tauri::command]
fn set_shortcut(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: u8,
    kind: String,
    shortcut: String,
) -> Result<(), String> {
    {
        let mut slots = state.slots.lock().map_err(|e| e.to_string())?;
        if let Some(slot) = slots.iter_mut().find(|s| s.id == id) {
            match kind.as_str() {
                "paste" => slot.paste_shortcut = shortcut,
                "copy" => slot.copy_shortcut = shortcut,
                _ => return Err("kind must be 'paste' or 'copy'".into()),
            }
        }
    }
    state.save()?;
    hotkeys::reregister_hotkeys(&app)?;
    let slots = state.slots.lock().map_err(|e| e.to_string())?.clone();
    let _ = app.emit("slots-updated", &slots);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Initialize slot state
            let data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let state = AppState::new(data_dir.clone());
            app.manage(state);

            // Initialize address state
            let addr_state = AddressState::new(&data_dir);
            app.manage(addr_state);

            // Build system tray menu
            let show_hide = MenuItem::with_id(app, "show_hide", "Show/Hide", true, None::<&str>)?;
            let enable_disable =
                MenuItem::with_id(app, "enable_disable", "Disable", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_hide, &enable_disable, &quit])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("PasteHaste")
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show_hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    "enable_disable" => {
                        let state = app.state::<AppState>();
                        let mut enabled = state.enabled.lock().unwrap();
                        *enabled = !*enabled;
                        let new_label = if *enabled { "Disable" } else { "Enable" };
                        let _ = enable_disable.set_text(new_label);
                        let _ = app.emit("enabled-changed", *enabled);
                    }
                    "quit" => {
                        let state = app.state::<AppState>();
                        let _ = state.save();
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // Register global hotkeys
            let handle = app.handle().clone();
            if let Err(e) = hotkeys::register_hotkeys(&handle) {
                eprintln!("Failed to register hotkeys: {}", e);
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            slots::get_slots,
            slots::set_slot,
            slots::save_slots,
            slots::get_enabled,
            slots::set_enabled,
            set_shortcut,
            address::get_address,
            address::get_address_config,
            address::new_random_address,
            address::set_address_shortcut,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PasteHaste");
}
