use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};
use gloo::console::log;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_state = use_state(|| "default".to_owned());
    let button_click_count_state = use_state(|| 0u32);
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username: String| {
        cloned_username_state.set(username.clone());
        log!("Username was changed to :", username);
    });
    let cloned_button_click_count_state = button_click_count_state.clone();
    let button_clicked = Callback::from(move |_| {
        let counter = *cloned_button_click_count_state;
        cloned_button_click_count_state.set(counter + 1u32);
        log!("Button was clicked");
    });

    html! {
        <div>
            <TextInput name="username" handle_onchange={username_changed} />
            <CustomButton label="Submit" onclick={button_clicked} />
            <p>{"Username : "}{&*username_state}</p>
            <p>{"Button click counter : "}{*button_click_count_state}</p>
        </div>
    }
}
