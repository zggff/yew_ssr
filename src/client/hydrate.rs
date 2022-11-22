fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        #[cfg(debug_assertions)]
        {
            yew::Renderer::<client::App>::new().render();
            log::info!("debug");
        }
        #[cfg(not(debug_assertions))]
        {
            log::info!("release");
            yew::Renderer::<client::App>::new().hydrate();
        }
    }
}
