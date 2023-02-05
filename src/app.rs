use yew::prelude::*;
use yew_router::{prelude::*};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/todo")]
    Todo,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Todo)]
fn home() -> Html {
    let link_todo_button = {
        let navigator = use_navigator().unwrap();

        let onclick = Callback::from(move |_| navigator.push(&Route::Todo));
        html! {
            <button {onclick}>{"SignIn"}</button>
        }
    };
    let link_todo2_button = {
        let navigator = use_navigator().unwrap();

        let onclick = Callback::from(move |_| navigator.push(&Route::Todo));
        html! {
            <button {onclick}>{"SignIn"}</button>
        }
    };
    html! {
        <div class="home">
            {link_todo_button}
            {link_todo2_button}

        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let button1 = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"button1"}</button>
        }
    };
    let button2 = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"button2"}</button>
        }
    };
    html! {
        <div class="home">
            {button1}
            {button2}
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home /> 
        },
        Route::Todo => html! {
            <Todo />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
