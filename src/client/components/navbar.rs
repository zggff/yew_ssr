use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html!(
        <div class="bg-slate-200 flex justify-start px-8 py-2 gap-8">
            <Link<Route> to={Route::Home} classes="btn-blue">{"home"}</Link<Route>>
            <Link<Route> to={Route::Secure} classes="btn-blue">{"secure"}</Link<Route>>
        </div>
    );
}
