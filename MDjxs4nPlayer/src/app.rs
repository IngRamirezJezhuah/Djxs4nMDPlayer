use gloo_timers::future::TimeoutFuture;
use leptos::task::spawn_local;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SeekArgs {
    seconds: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MusicaData {
    pub title: String,
    pub artist: String,
    pub cover_url: String,
    pub status: String,
    pub position: f64,
    pub length: f64,
}

/// Documentation for [`Mdplayer`]
#[component]
pub fn Mdplayer() -> impl IntoView {

    let (is_playing, set_is_playing) = signal(false);
    let (title, set_title) = signal("Cargandp...".to_string());
    let (artist, set_artist) = signal("Nom Artista".to_string());
    let (cover_url, set_cover_url) = signal("/public/cover.png".to_string());
    let (position, set_position) = signal(0.0);
    let (length, set_length) = signal(0.0);

    let progress_pct = move || -> f64 {
        if length.get() > 0.0 {
            ((position.get() as f64 / length.get() as f64) * 100.0).min(100.0)
        } else {
            0.0
        }
    };

    let aplicar_metadata = move |meta: MusicaData| {
        set_title.set(meta.title);
        set_artist.set(meta.artist);
        // cover_url llega como ruta relativa fija (/public/cover.png?v=...) o vacia.
        // si esta vacia dejamos la imagen predeterminada que ya estaba cargada.
        if !meta.cover_url.is_empty() {
            set_cover_url.set(meta.cover_url);
        }
        set_is_playing.set(meta.status == "Playing");
        set_position.set(meta.position);
        set_length.set(meta.length);
    };

    let update_metadata = move || {
        spawn_local(async move {
            let res = invoke("get_metadata", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            if let Ok(meta) = serde_wasm_bindgen::from_value::<MusicaData>(res) {
                aplicar_metadata(meta);
            }
        });
    };

    update_metadata();

    // actualizacion en tiempo real de la cancion y su progreso
    spawn_local(async move {
        loop {
            TimeoutFuture::new(1000).await;
            update_metadata();
        }
    });

    let toggle_play = move |_| {
        spawn_local(async move {
            let res = invoke("play_pause", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            if let Ok(state) = serde_wasm_bindgen::from_value::<bool>(res) {
                set_is_playing.set(state);
            }
            update_metadata();
        });
    };

    // Acción para adelantar (+10s) o retrasar (-10s)
    let seek = move |secs: f64| {
        move |_| {
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&SeekArgs { seconds: secs }).unwrap();
                let _ = invoke("seek_audio", args).await;
                update_metadata();
            });
        }
    };

    let next_track = move |_| {
        spawn_local(async move{
            let _ = invoke("next_track", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            // Actualizamos los metadatos inmediatamente despueus de cambiar
            let res = invoke("get_metadata", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            if let Ok(meta) = serde_wasm_bindgen::from_value::<MusicaData>(res) {
                aplicar_metadata(meta);
            }
        });
    };

    let previous_track = move |_| {
        spawn_local(async move {
            let _ = invoke("previous_track", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            // Actualizamos los metadatos inmediatamente después de cambiar
            let res = invoke("get_metadata", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            if let Ok(meta) = serde_wasm_bindgen::from_value::<MusicaData>(res) {
                aplicar_metadata(meta);
            }
        });
    };

    view! {
        <div>
            <div class="cont">
                <div class="circulo">
                    //<img src="/public/cover.png" alt="alttern" class="port-giratorio after"/>
                    <img src=move || cover_url.get() alt="alttern" class="port-giratorio after"/>
                </div>
                //<img src="/public/cover.png" alt="cover" class="cover"/>
                <img src=move || cover_url.get() alt="cover" class="cover"/>

                <div class="btn-cont">
                /*
                    <p> Titulo </p>
                    <p> Nom Artista </p>
                    <button class="btn-trsp">"↻"</button>
                    <button class="btn-trsp">"◀" </button>
                    <button class="btn">"▐▐"</button>
                    <button class="btn">"▷"</button>
                    <button class="btn-trsp">"▶"</button>
                    <button class="btn-trsp">"↺"</button>
                */  
                    <p>{move || title.get()}</p>
                    <p>{move || artist.get()}</p>

                    {/* Botones de control */}
                    <button class="btn-trsp" on:click=seek(-10.0)>"↺ 10s"</button>
                    <button class="btn-trsp" on:click=previous_track>"◀"</button>
                    <button class="btn" on:click=toggle_play>
                        {move || if is_playing.get() { "▐▐" } else { "▷" }}
                    </button>
                    <button class="btn-trsp" on:click=next_track>"▶"</button>
                    <button class="btn-trsp" on:click=seek(10.0)>"10s ↻"</button>
                    <div>"----------------"</div>
                    //<progress value= "25" max= "100">25%</progress>
                    <progress value=progress_pct max="100">"..."</progress>
                </div>
            </div>
            /*
            <div id="contenedor">
                <div  class="contenedor-loader">
                    <div class="loader"></div>
                </div>
            </div>
            */
        </div>
    }
}
