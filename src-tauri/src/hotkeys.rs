use crate::address::AddressState;
use crate::input;
use crate::slots::AppState;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// Register all shortcuts based on current slot and address config.
pub fn register_hotkeys(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // --- Slot shortcuts ---
    let state = app.state::<AppState>();
    let slots = state.slots.lock().unwrap().clone();

    for slot in &slots {
        if !slot.paste_shortcut.is_empty() {
            if let Ok(shortcut) = slot.paste_shortcut.parse::<Shortcut>() {
                let handle = app.clone();
                let slot_id = slot.id;
                app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                    if event.state() != ShortcutState::Pressed { return; }
                    let state = handle.state::<AppState>();
                    if !*state.enabled.lock().unwrap() { return; }
                    let content = {
                        let slots = state.slots.lock().unwrap();
                        slots.iter().find(|s| s.id == slot_id).map(|s| s.content.clone()).unwrap_or_default()
                    };
                    if !content.is_empty() {
                        std::thread::spawn(move || {
                            std::thread::sleep(std::time::Duration::from_millis(50));
                            input::release_modifiers();
                            std::thread::sleep(std::time::Duration::from_millis(30));
                            input::send_string(&content);
                        });
                    }
                })?;
            }
        }

        if !slot.copy_shortcut.is_empty() {
            if let Ok(shortcut) = slot.copy_shortcut.parse::<Shortcut>() {
                let handle = app.clone();
                let slot_id = slot.id;
                app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                    if event.state() != ShortcutState::Pressed { return; }
                    let state = handle.state::<AppState>();
                    if !*state.enabled.lock().unwrap() { return; }
                    let h = handle.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        input::release_modifiers();
                        std::thread::sleep(std::time::Duration::from_millis(30));
                        input::send_ctrl_c();
                        std::thread::sleep(std::time::Duration::from_millis(150));
                        if let Ok(text) = h.clipboard().read_text() {
                            let state = h.state::<AppState>();
                            {
                                let mut slots = state.slots.lock().unwrap();
                                if let Some(slot) = slots.iter_mut().find(|s| s.id == slot_id) {
                                    slot.content = text;
                                }
                            }
                            let _ = state.save();
                            let slots = state.slots.lock().unwrap().clone();
                            let _ = h.emit("slots-updated", &slots);
                        }
                    });
                })?;
            }
        }
    }

    // --- Address shortcuts ---
    let addr_state = app.state::<AddressState>();
    let config = addr_state.config.lock().unwrap().clone();

    let address_binds: Vec<(&str, String)> = vec![
        ("street", config.street_shortcut),
        ("city", config.city_shortcut),
        ("state", config.state_shortcut),
        ("zip", config.zip_shortcut),
    ];

    for (field, shortcut_str) in address_binds {
        if shortcut_str.is_empty() { continue; }
        if let Ok(shortcut) = shortcut_str.parse::<Shortcut>() {
            let handle = app.clone();
            let field = field.to_string();
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                if event.state() != ShortcutState::Pressed { return; }
                let app_state = handle.state::<AppState>();
                if !*app_state.enabled.lock().unwrap() { return; }

                let addr_state = handle.state::<AddressState>();
                let addr = addr_state.current.lock().unwrap().clone();
                let text = match field.as_str() {
                    "street" => addr.street,
                    "city" => addr.city,
                    "state" => addr.state,
                    "zip" => addr.zip,
                    _ => return,
                };
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    input::release_modifiers();
                    std::thread::sleep(std::time::Duration::from_millis(30));
                    input::send_string(&text);
                });
            })?;
        }
    }

    Ok(())
}

/// Unregister all global shortcuts, then re-register from current state.
pub fn reregister_hotkeys(app: &AppHandle) -> Result<(), String> {
    app.global_shortcut().unregister_all().map_err(|e| e.to_string())?;
    register_hotkeys(app).map_err(|e| e.to_string())?;
    Ok(())
}
