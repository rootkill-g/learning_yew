use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::router::Route;

#[allow(non_snake_case)]
#[function_component(Hello)]
pub fn hellO() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{"Hello this is Hello Page"}</h1>
            <button {onclick}>{"Go Home"}</button>
        </div>
    }
}
