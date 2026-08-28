use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct SeekArgs {
    seconds: f64,
}

#[derive(Serialize, Deserialize)]
struct LoadArgs {
    path: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct TrackMetadata {
    title: String,
    artist: String,
    cover_base64: Option<String>,
}

/// Documentation for [`Mdplayer`]
#[component]
pub fn Mdplayer() -> impl IntoView {

    let (is_playing, set_is_playing) = signal(false);
    let (title, set_title) = signal("Titulo".to_string());
    let (artist, set_artist) = signal("Nom Artista".to_string());
    let (cover_url, set_cover_url) = signal("/public/cover.png".to_string());

    // Acción para alternar Play / Pausa
    let toggle_play = move |_| {
        leptos::task::spawn_local(async move {
            let res = invoke("play_pause", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            if let Ok(state) = serde_wasm_bindgen::from_value::<bool>(res) {
                set_is_playing.set(state);
            }
        });
    };

    // Acción para adelantar (+10s) o retrasar (-10s)
    let seek = move |secs: f64| {
        move |_| {
            leptos::task::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&SeekArgs { seconds: secs }).unwrap();
                let _ = invoke("seek_audio", args).await;
            });
        }
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
                    <button class="btn-trsp">"◀"</button>
                    <button class="btn" on:click=toggle_play>
                        {move || if is_playing.get() { "▐▐" } else { "▷" }}
                    </button>
                    <button class="btn-trsp">"▶"</button>
                    <button class="btn-trsp" on:click=seek(10.0)>"10s ↻"</button>
                    <div>----- barra tiempo ---</div>
                    <p>...</p>
                </div>
            </div>
                
            <div id="contenedor">
                <div  class="contenedor-loader">
                    <div class="loader"></div>
                </div>
            </div>
        </div>
    }
}
