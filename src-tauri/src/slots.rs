use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub id: u8,
    pub label: String,
    pub content: String,
    /// Shortcut string for pasting, e.g. "Alt+1". Empty string means unbound.
    #[serde(default)]
    pub paste_shortcut: String,
    /// Shortcut string for copying into this slot, e.g. "Ctrl+Alt+1". Empty string means unbound.
    #[serde(default)]
    pub copy_shortcut: String,
}

pub struct AppState {
    pub slots: Mutex<Vec<Slot>>,
    pub enabled: Mutex<bool>,
    pub data_path: PathBuf,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let data_path = data_dir.join("slots.json");
        let slots = load_slots_from_file(&data_path);
        Self {
            slots: Mutex::new(slots),
            enabled: Mutex::new(true),
            data_path,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let slots = self.slots.lock().map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&*slots).map_err(|e| e.to_string())?;
        if let Some(parent) = self.data_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.data_path, json).map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn load_slots_from_file(path: &PathBuf) -> Vec<Slot> {
    if let Ok(data) = fs::read_to_string(path) {
        if let Ok(slots) = serde_json::from_str::<Vec<Slot>>(&data) {
            if slots.len() >= 5 {
                return slots;
            }
        }
    }
    default_slots()
}

fn default_slots() -> Vec<Slot> {
    (1..=5)
        .map(|i| Slot {
            id: i,
            label: format!("Slot {}", i),
            content: String::new(),
            paste_shortcut: format!("Alt+{}", i),
            copy_shortcut: format!("Ctrl+Alt+{}", i),
        })
        .collect()
}

#[tauri::command]
pub fn get_slots(state: tauri::State<'_, AppState>) -> Vec<Slot> {
    state.slots.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_slot(state: tauri::State<'_, AppState>, id: u8, content: String) -> Result<(), String> {
    let mut slots = state.slots.lock().map_err(|e| e.to_string())?;
    if let Some(slot) = slots.iter_mut().find(|s| s.id == id) {
        slot.content = content;
    }
    Ok(())
}

#[tauri::command]
pub fn save_slots(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.save()
}

#[tauri::command]
pub fn get_enabled(state: tauri::State<'_, AppState>) -> bool {
    *state.enabled.lock().unwrap()
}

#[tauri::command]
pub fn set_enabled(state: tauri::State<'_, AppState>, enabled: bool) {
    *state.enabled.lock().unwrap() = enabled;
}
