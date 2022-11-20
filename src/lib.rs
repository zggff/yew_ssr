use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    prelude::*,
};

mod components;
use components::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button class="bg-blue-200 w-full mt-4 p-2 mx-4 rounded" {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1 class="text-center bg-blue-200">{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Layout)]
pub fn layout() -> Html {
    html!(
        <>
            <Navbar/>
            <div class="p-4">
                <Switch<Route> render={switch} />
            </div>
        </>
    )
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout/>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: String,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);
    return html!(
            <Router history={history}>
                <Layout/>
            </Router>
    );
}
