mod components;

use components::atoms::main_title::{Color, MainTitle};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct LangName {
    username: String,
    fav_lang: String,
}

const CSS_FILE: &str = include_str!("main.css");

#[styled_component]
pub fn App() -> Html {
    let name = "etch1000";
    let lang_name = LangName {
        username: name.to_owned(),
        fav_lang: String::from("Rust Programming Language"),
    };

    log!("My name is :", name);
    log!(serde_json::to_string_pretty(&lang_name).ok());

    let class_p = "paragraph";

    let msg: Option<&str> = None;

    let num_list = (1..=10).collect::<Vec<_>>();

    let css_file = Style::new(CSS_FILE).unwrap();

    let main_title_load = Callback::from(|message: String| log!(message));

    html! {
        <>
            <div class={css_file}>

                <MainTitle title="Yew is Cool" color={Color::Pink} on_load={main_title_load} />

                if class_p == "paragraph" {
                    <p>{"We are going Full Stack Now"}</p>
                }

                if let Some(message) = msg {
                    <p>{message}</p>
                } else {
                    <p>{"No messages to show"}</p>
                }

                <ul class="item-list">
                    { to_li(num_list) }
                </ul>
            </div>
        </>
    }
}

fn to_li<T: std::fmt::Display>(list: Vec<T>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}
