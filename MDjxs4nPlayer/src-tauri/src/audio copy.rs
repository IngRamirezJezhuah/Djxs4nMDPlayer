use std::{fs::File, io::{BufReader, Sink}, result};
//use rodio::{Decoder, Sink,OutputStream, OutputStreamHandle, source, stream};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use lofty::prelude::*;
use lofty::probe::Probe;
use rodio::{Decoder, OutputStream, Sink};


#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct MusicaData {
    pub title: String,
    pub artist: String,
    pub cover_base64: Option<String>,
}

pub struct EstadoReproductor {
    pub playlist: Vec<String>,
    pub musica_actual: usize,
    pub esta_jalando: bool,
    //la chingadera que controla la reprocduccion de audio
    sink: Option<Sink>,
    // la salida de audio guardada
    _stream: Option<OutputStream>,
    _stream_handle: Option<OutputStreamHandle>,
}

impl EstadoReproductor {
    pub fn new() -> Self {
        Self {
            playlist: Vec::new(),
            musica_actual: 0,
            esta_jalando: false,
            sink: None,
            _stream: None,
            _stream_handle: None,
        }
    }

    pub fn garantizar_flujo(&mut self) -> Result<(), String> {
        if self.sink.is_none() {
            let (stream, stream_handle) = OutputStream::try_default()
                .map_err(|e| format! ("Errore al abrir salida de audio: {}", e))?;
            
            let sink = Sink::connect_new(&stream_handle);
            self._stream = Some(stream);
            self._stream_handle = Some(stream);
            self.sink = Some(sink);
        }
        Ok(())
    }

    pub fn Cargar_y_reproducir(&mut self, file_path: &str) -> Result<MusicaData, String> {
        self.garantizar_flujo()?;

        let file = File::open(file_path)
            .map_err(|e| format!("No s pudo abrir el archivo {}: {}", file_path, e))?;
        let lector = BufReader::new(file);

        let source = Decoder::new(lector)
            .map_err(|e| format!("Error al edcodificar el audio: {}", e))?;

        if let Some(ref sink) = self.sink {
            sink.stop();
            sink.append(source);
            sink.play();
            self.esta_jalando = true;
        }

        let metadata = Self::Sacar_Metadata(file_path);
        Ok(metadata)
    }

    pub fn altern_repro_pausar(&mut self) -> bool {
        if let Some(ref sink) = self.sink {
            if sink.is_paused() {
                sink.play();
                self.esta_jalando= true;
            } else {
                sink.pause();
                self.esta_jalando= false;
            }
        }
        self.esta_jalando
    }

    pub fn Buscar_relativo(&mut self, seconds: f64) {
        if let Some(ref sink) = self.sink {
            let current_pos = sink.get_pos();
            // CORRECCIÓN: Se sustituyó 'seconfs' y 'secs' por el argumento formal 'seconds'
            let new_pos = if seconds >= 0.0 {
                current_pos + std::time::Duration::from_secs_f64(seconds)
            } else {
                let sub = std::time::Duration::from_secs_f64(seconds.abs());
                if current_pos > sub {
                    current_pos - sub
                } else {
                    std::time::Duration::ZERO
                }
            };
            let _ = sink.try_seek(new_pos);
        }
    }

    pub fn Sacar_Metadata(file_path: &str) -> MusicaData {
        let mut title = "Desconocido".to_string();
        let mut artist = "Atrista Desconocido".to_string();
        let mut cover_base64 = None;

        if let Ok(tagged_file) = Probe::open(file_path).and_then(|p| p.read())  {
            if let Some(tag) = tagged_file.primary_tag() {
                if let Some(t) = tag.title() {title = t.to_string();}
                if let Some(a) = tag.artist() { artist = a.to_string();}

                if let Some(picture) = tag.pictures().first(){
                    let mime = picture.mime_type().as_str();
                    let b64 = impl_base64_encode(picture.data());
                    cover_base64 = Some(format!("data:{};base64,{}", mime, b64));
                } 
            }
        }
        MusicaData { title, artist, cover_base64 }
    }

    fn impl_base64_encode(data: &[u8]) -> String {
        use std::fmt::Write;
        let mut result = String::with_capacity(data.len() * 4 / 3 + 4 );
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for chunk in data.chunks(3) {
            let b = match chunk.len() {
                3 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32),
                2 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8,
                1 => (chunk[0] as u32) << 16,
                _ => 0,
            };
            result.push(CHARS[(b >> 18 & 0x3F) as usize] as char);
            result.push(CHARS[(b >> 12 & 0x3F) as usize] as char);
            if chunk.len() > 1 {
                result.push(CHARS[(b >> 6 & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
            if chunk.len() > 2 {
                result.push(CHARS[(b & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
        }
        result
    }
    
}

/*
Antes de que se me olvide yo del futuro esta Estructura de codigo existe para definir la estructura de
el estado de reproduccion que matendra la instamcio axctiva de sink (el canal del sonido)

ahi pues la de seek abre el archivo de musica con lofty y si conitnen una imagen
la codifica en u string para que leptos pueda renderizar la caratula directa,ente en la etiqueta  de imagen
*/
