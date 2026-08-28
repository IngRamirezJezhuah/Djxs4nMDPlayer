
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use tauri::State;

mod audio;
use crate::audio::{EstadoReproductor, MusicaData};
pub struct AppAudioState(pub Mutex<EstadoReproductor>);


#[tauri::command]
fn play_pause(state: State<'_, AppAudioState>) -> Result<bool, String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.altern_repro_pausar()
}

#[tauri::command]
fn seek_audio(seconds: f64, state: State<'_, AppAudioState>) -> Result<(), String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.Buscar_relativo(seconds);
    Ok(())
}

#[tauri::command]
fn next_track(state: State<'_, AppAudioState>) -> Result<(), String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.siguiente();
    Ok(())
}

#[tauri::command]
fn previous_track(state: State<'_, AppAudioState>) -> Result<(), String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.anterior();
    Ok(())
}

#[tauri::command]
fn get_metadata() -> MusicaData {
    EstadoReproductor::sacar_metadata()
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        //en la variable de aqui abajo se importan mis funciones
        .invoke_handler(tauri::generate_handler![
                greet,
                play_pause,
                seek_audio,
                next_track,
                previous_track,
                get_metadata,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// mis cosas del backend se llamaran desde aqui