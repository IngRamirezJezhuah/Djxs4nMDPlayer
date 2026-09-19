/*    AUDIO.RS
=======================
modulo encargado de "reconocer la musica": aqui se reciben los metadatos del
reproductor MPRIS activo usando playerctl como puente. tambien se encarga de
bajar y guardar la portada en la carpeta public del proyecto para que el front
la pinte (guardando SOLO cuando cambia, para no meter al trunk en un bucle de
recarga infinita).
 */
use std::io::Read;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

// estructura que se manda al front
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MusicaData {
    pub title: String,
    pub artist: String,
    pub cover_url: String,
    pub status: String,
    pub position: f64,
    pub length: f64,
}


pub struct EstadoReproductor;
impl EstadoReproductor {
    pub fn new() -> Self {
        Self
    }

    /// Alterna play/pausa y devuelve true si quedo reproduciendo
    pub fn altern_repro_pausar(&self) -> Result<bool, String> {
        let _ = Command::new("playerctl")
            .arg("play-pause")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("Error pegado con playerctl: {}", e))?;

        Ok(Self::leer_status() == "Playing")
    }

    /// Adelanta (+) o retrocede (-) los segundos indicados
    pub fn buscar_relativo(&self, seconds: f64) {
        let arg = if seconds >= 0.0 {
            format!("{}+", seconds)
        } else {
            format!("{}", seconds)
        };
        let _ = Command::new("playerctl")
            .args(["position", &arg])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    /// Siguiente cancion
    pub fn siguiente(&self) {
        let _ = Command::new("playerctl")
            .arg("next")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    /// Cancion anterior
    pub fn anterior(&self) {
        let _ = Command::new("playerctl")
            .arg("previous")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    /// Junta todo lo que el front necesita para pintar la cancion actual
    pub fn sacar_metadata() -> MusicaData {
        let (title, artist, art_url, status) = Self::leer_metadatos();
        MusicaData {
            title,
            artist,
            cover_url: Self::preparar_portada(art_url).unwrap_or_default(),
            status,
            position: Self::leer_position(),
            length: Self::leer_length(),
        }
    }

    // ---------- ayudantes internos ----------

    // ejecuta playerctl y devuelve la salida en texto (None si falla)
    fn playerctl(sub: &str, args: &[&str]) -> Option<String> {
        let out = Command::new("playerctl")
            .arg(sub)
            .args(args)
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    // lote de metadatos: titulo, artista, url cruda de la portada y estado
    fn leer_metadatos() -> (String, String, Option<String>, String) {
        let title = Self::playerctl("metadata", &["--format", "{{title}}"])
            .unwrap_or_else(|| "Nada sonando...".to_string());
        let artist = Self::playerctl("metadata", &["--format", "{{artist}}"])
            .unwrap_or_else(|| "Artista desconocido".to_string());
        let art_url = Self::playerctl("metadata", &["--format", "{{mpris:artUrl}}"]);
        let status = Self::playerctl("status", &[]).unwrap_or_else(|| "Detenido".to_string());
        (title, artist, art_url, status)
    }

    // posicion actual en segundos (playerctl la imprime en segundos con decimales)
    fn leer_position() -> f64 {
        Self::playerctl("position", &[])
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    // duracion total en segundos (mpris:length viene en microsegundos)
    fn leer_length() -> f64 {
        Self::playerctl("metadata", &["--format", "{{mpris:length}}"])
            .and_then(|s| s.parse::<f64>().ok())
            .map(|micros| micros / 1_000_000.0)
            .unwrap_or(0.0)
    }

    // estado textual del reproductor (Playing / Paused / Stopped)
    fn leer_status() -> String {
        Self::playerctl("status", &[]).unwrap_or_else(|| "Stopped".to_string())
    }

    // descarga la portada cruda (file:// o http/https) y devuelve sus bytes
    fn descargar_portada(art: &str) -> Option<Vec<u8>> {
        if let Some(path) = art.strip_prefix("file://") {
            return std::fs::read(path).ok();
        }

        if art.starts_with("http://") || art.starts_with("https://") {
            if let Ok(resp) = ureq::get(art)
                .timeout(std::time::Duration::from_secs(10))
                .call()
            {
                // limite de 5 para no comerme la ram con la caratula
                let mut reader = resp.into_reader().take(5 * 1024 * 1024);
                let mut bytes = Vec::new();
                if reader.read_to_end(&mut bytes).is_ok() {
                    return Some(bytes);
                }
            }
        }
        None
    }

    // guarda la portada en la carpeta public y devuelve la ruta relativa fija /public/cover.png?v=<mtime> para que el front la muestre. SOLO se
    fn preparar_portada(art_url: Option<String>) -> Option<String> {
        let art = art_url.unwrap_or_default();
        if art.is_empty() {
            return None;
        }

        let bytes = Self::descargar_portada(&art)?;

        // raiz del proyecto (MDjxs4nPlayer) a partir del directorio del crate
        let project_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent()?;

        // destino fuente (public/) y copia servida por trunk (dist/public/)
        let dest_src = project_dir.join("public").join("cover.png");
        let dest_dist = project_dir.join("dist").join("public").join("cover.png");

        // slash anti-bucle: si ya esta guardada la misma portada no escribir
        if std::fs::read(&dest_src).map(|b| b == bytes).unwrap_or(false) {
            return Self::ruta_versionada(&dest_src);
        }

        let mut guardado = false;
        if std::fs::write(&dest_src, &bytes).is_ok() {
            guardado = true;
        }
        if let Some(parent) = dest_dist.parent() {
            if std::fs::create_dir_all(parent).is_ok()
                && std::fs::write(&dest_dist, &bytes).is_ok()
            {
                guardado = true;
            }
        }
        if !guardado {
            return None;
        }

        Self::ruta_versionada(&dest_src)
    }

    // ruta fija con ?v= basado en el mtime para que el front recargue la
    // imagen cuando la portada cambie (sin quedarse con la cacheada)
    fn ruta_versionada(ruta: &std::path::Path) -> Option<String> {
        let ver = std::fs::metadata(ruta)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Some(format!("/public/cover.png?v={}", ver))
    }
}
