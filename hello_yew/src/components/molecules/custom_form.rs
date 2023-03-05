use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};
use gloo::console::log;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_state = use_state(|| "default".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username: String| {
        cloned_username_state.set(username.clone());
        log!("Username was changed to :", username);
    });

    html! {
        <form>
            <TextInput name="username" handle_onchange={username_changed} />
            <CustomButton label="Submit" />
            <p>{"Username : "}{&*username_state}</p>
        </form>
    }
}
