use std::ops::Deref;

use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};
use gloo::console::log;
use yew::prelude::*;

#[derive(Default, Clone)]
struct Data {
    username: String,
    click_counter: u32,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username: String| {
        let mut data = cloned_state.deref().clone();
        data.username = username.clone();
        cloned_state.set(data);
        log!("Username was changed to :", username);
    });

    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.click_counter += 1;
        cloned_state.set(data);
        log!("Button was clicked");
    });

    html! {
        <div>
            <TextInput name="username" handle_onchange={username_changed} />
            <CustomButton label="Submit" onclick={button_clicked} />
            <p>{"Username : "}{&state.username}</p>
            <p>{"Button click counter : "}{state.click_counter}</p>
        </div>
    }
}
