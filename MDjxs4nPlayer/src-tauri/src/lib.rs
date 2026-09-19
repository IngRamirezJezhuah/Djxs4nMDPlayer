// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use tauri::State;

mod audio;
use crate::audio::{EstadoReproductor, MusicaData};
pub struct AppAudioState(pub Mutex<EstadoReproductor>);


// alterna play pausa del reproductor
#[tauri::command]
fn play_pause(state: State<'_, AppAudioState>) -> Result<bool, String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.altern_repro_pausar()
}
// Adelanta o atrasa los segundos
#[tauri::command]
fn seek_audio(seconds: f64, state: State<'_, AppAudioState>) -> Result<(), String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.buscar_relativo(seconds);
    Ok(())
}
// siguienter cancion
#[tauri::command]
fn next_track(state: State<'_, AppAudioState>) -> Result<(), String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.siguiente();
    Ok(())
}
//cancion anterior
#[tauri::command]
fn previous_track(state: State<'_, AppAudioState>) -> Result<(), String> {
    let player = state.0.lock().map_err(|e| e.to_string())?;
    player.anterior();
    Ok(())
}

//el de los datos de la cancion
#[tauri::command]
fn get_metadata() -> MusicaData {
    EstadoReproductor::sacar_metadata()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // el estado compartido del reproductor. Sin esto los comandos
        // que usan State fallan y los botones no hacen nada.
        .manage(AppAudioState(Mutex::new(EstadoReproductor::new())))
        //en la variable de aqui abajo se importan mis funciones
        .invoke_handler(tauri::generate_handler![
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