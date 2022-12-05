#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/scripts/map.js")]
extern "C" {
    fn render_map();
    fn init_map();
    fn set_zoom(zoom: u32);
    fn set_center(x: f32, y: f32);
    fn set_text(text: &str);
}

#[function_component(Map)]
pub fn map() -> Html {
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(|| {
            set_zoom(15);
            set_center(10.0, 0.0);
            set_text("marker");
            init_map();
        });
    }

    html!(
        <div id="map" style="width: 600px; height: 600px">
        </div>
    )
}
