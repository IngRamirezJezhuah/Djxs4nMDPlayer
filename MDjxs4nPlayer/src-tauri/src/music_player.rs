/* archivo de configuracion donde se alberrga el pryecto de usica
use tauri::{AppHandle, Emitter, async_runtime::TokioHandle};
use serde::Serialize;
use std::time::Duration;

#[derive(Clone, Serialize)]
struct AudioProgress {
    current : f64,
    total: f64,

}

#[tauri::command]
async fn play_Audio(app: AppHandle) {
    let total_duracion = 180.0;
    let mut current = 0.0;
    
    while current <= total_duracion {
        //total_duracion::time::sleep(std::time::Duration::from_millis(500)).await;
        TokioHandle::time::sleep(Duration::from_millis(500)).await;
        current += 0.5;

        let _ = app.emit("audio-progress", AudioProgress {
            current,
            total : total_duracion,
        });
    }
}
*/
/* version sin romper */
// archivo de configuracion donde se alberrga el pryecto de usica
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
struct AudioProgress {
    current : f64,
    total: f64,

}

#[tauri::command]
async fn play_Audio(app: AppHandle) {
    let total_duracion = 180.0;
    let mut current = 0.0;
    
    while current <= total_duracion {
        //total_duracion::time::sleep(std::time::Duration::from_millis(500)).await;
        current += 0.5;

        app.emit("audio-progress", AudioProgress {
            current,
            total : total_duracion,
        }).unwrap();
    }
}