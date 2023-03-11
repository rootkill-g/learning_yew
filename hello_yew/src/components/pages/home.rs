use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Hello));

    html! {
        <div>
        <h1>{"Hello This is Home"}</h1>
        <button {onclick}>{"Go check Hello"}</button>
        </div>
    }
}
