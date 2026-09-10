use std::process::Command;
use serde::{Deserialize, Serialize};
use tauri::ipc::CommandArg;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MusicaData {
    pub title: String,
    pub artist: String,
    pub cover_url: String,
    pub status: String,
    pub position: Option<u64>,
    pub length: Option<u64>,
}

pub struct EstadoReproductor;

impl EstadoReproductor {
    pub fn new() -> Self {
        Self
    }

    /// Alterna la reproducción/pausa mediante playerctl
    pub fn altern_repro_pausar(&self) -> Result<bool, String> {
        let _ = Command::new("playerctl")
            .arg("play-pause")
            .output()
            .map_err(|e| format!("Error al ejecutar playerctl: {}", e))?;

        let status = Self::obtener_estado_reproduccion();
        Ok(status == "Playing")
    }

    /// Adelanta (+) o retrocede (-) los segundos indicados
    pub fn Buscar_relativo(&self, seconds: f64) {
        let arg_sec = if seconds >= 0.0 {
            format!("{}+", seconds)
        } else {
            format!("{}-", seconds.abs())
        };

        let _ = Command::new("playerctl")
            .args(["position", &arg_sec])
            .output();
    }

    /// Siguiente canción
    pub fn siguiente(&self) {
        let _ = Command::new("playerctl").arg("next").output();
    }

    /// Canción anterior
    pub fn anterior(&self) {
        let _ = Command::new("playerctl").arg("previous").output();
    }

    /// Extrae metadatos actuales usando playerctl (Título, Artista, Portada)
    pub fn sacar_metadata() -> MusicaData {
        if let Ok(output) = Command::new("bash").arg("./get_metadata.sh").output() {
            if output.status.success() {
                if let Ok(data) = serde_json::from_slice::<MusicaData>(&output.stdout){
                    return data;
                }
            }
        }

        MusicaData { 
            title: "Nada sonando...".to_string(),
            artist: "Offline".to_string(), 
            cover_url: "/public/cover.png".to_string(), 
            status: "Stopped".to_string(), 
            position: Some(0), 
            length: Some(100), 
        }
    }

    fn obtener_estado_reproduccion() -> String {
        //Self::ejecutar_playerctl_format("{{status}}").unwrap_or_else(|| "Stopped".to_string())
        let output = Command::new("playerctl")
        .arg(["status"])
        .output();

        if let Ok(out) = output{
            String::from_utf16_lossy(&out.stdout).trim().to_string()
        } else {
            "Stopped".to_string()
        }
    }

    fn ejecutar_playerctl_format(format_str: &str) -> Option<String> {
        let output = Command::new("playerctl")
            .args(["metadata", "--format", format_str])
            .output()
            .ok()?;

        if output.status.success() {
            let res = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if res.is_empty() { None } else { Some(res) }
        } else {
            None
        }
    }
}