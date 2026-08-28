use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MusicaData {
    pub title: String,
    pub artist: String,
    pub cover_url: String,
    pub status: String,
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
        let title = Self::ejecutar_playerctl_format("{{title}}")
            .unwrap_or_else(|| "Nada sonando...".to_string());
        
        let artist = Self::ejecutar_playerctl_format("{{artist}}")
            .unwrap_or_else(|| "Offline".to_string());
        
        let cover_url = Self::ejecutar_playerctl_format("{{mpris:artUrl}}")
            .unwrap_or_else(|| "/public/cover.png".to_string());

        let status = Self::obtener_estado_reproduccion();

        MusicaData {
            title,
            artist,
            cover_url,
            status,
        }
    }

    fn obtener_estado_reproduccion() -> String {
        Self::ejecutar_playerctl_format("{{status}}").unwrap_or_else(|| "Stopped".to_string())
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