use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    return html!(
        <div>
            <Link<Route> to={Route::Home}>{"home"}</Link<Route>>
            <Link<Route> to={Route::Secure}>{"secure"}</Link<Route>>
        </div>
    );
}
