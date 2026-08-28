use leptos::context::provide_context;
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


/// Documentation for [`Mdplayer`]
#[component]
pub fn Mdplayer() -> impl IntoView {

    view! {
        <div>
            <div class="cont">
                <div class="circulo">
                    <img src="/public/cover.png" alt="alttern" class="port-giratorio after"/>
                </div>
                <img src="/public/cover.png" alt="cover" class="cover"/>
                
                <div class="btn-cont">
                    <p> Titulo </p>
                    <p> Nom Artista </p>
                    <button class="btn-trsp">"↻"</button>
                    <button class="btn-trsp">"◀" </button>
                    <button class="btn">"▐▐"</button>
                    <button class="btn">"▷"</button>
                    <button class="btn-trsp">"▶"</button>
                    <button class="btn-trsp">"↺"</button>
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
