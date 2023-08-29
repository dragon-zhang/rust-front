mod utils;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component]
fn App() -> Html {
    let onclick = Callback::from(move |_| {
        alert("Hello, rust-front!");
    });
    html! {
        <div>
            <h1>{"Hello Rust"}</h1>
            <button {onclick}>{ "Click" }</button>
        </div>
    }
}

#[wasm_bindgen]
pub fn start() {
    log("Hello Rust!");
    yew::Renderer::<App>::new().render();
}
