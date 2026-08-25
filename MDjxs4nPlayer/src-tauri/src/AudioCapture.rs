//Este módulo utiliza cpal para capturar las muestras PCM en tiempo 
//real desde el dispositivo loopback (monitor de audio del sistema) 
//y almacenarlas de forma segura en un búfer compartido:  

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct AudioCapture {
    pub buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCapture {
    pub fn init() -> Self {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .expect("No se encontró dispositivo de audio de entrada/loopback");

        let config = device.default_input_config().unwrap();
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(4096)));
        let buffer_clone = Arc::clone(&buffer);

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut lock = buffer_clone.lock().unwrap();
                lock.clear();
                lock.extend_from_slice(data);
            },
            |err| eprintln!("Error en captura de audio: {}", err),
            None,
        ).unwrap();

        stream.play().unwrap();
        AudioCapture { buffer }
    }
}