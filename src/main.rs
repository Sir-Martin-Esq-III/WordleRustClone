use yew::prelude::*;
use yew::{classes, html};

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub words: String,
}

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    html! {
        <div  class={classes!("app")}>
            <h1>{"RUST WORDLE"}</h1>
            <div class={classes!("words-container")}>
                <Word_Row/>
                <Word_Row/>
                <Word_Row/>
                <Word_Row/>
                <Word_Row/>
            </div>

        </div>
    }
}

#[function_component(Word_Row)]
fn word_row() -> Html {
    let word_guesses = use_state(|| vec!["t", "e", "s", "t", "s"]);
    html! {
        <div class={classes!("word-row-container")}>
        {for word_guesses.iter().map(|letter|{html!{<div class={classes!("letter-container")}><p class={classes!("letter")}>{letter}</p></div>}})}
        </div>
    }
}

#[function_component(Test)]
fn test(props: &AppProps) -> Html {
    let text = use_state(|| "test");

    html! {
        <div>
            <p>{&props.words}</p>
            <input type="text" value={*text}/>
            <p>{*text}</p>
        </div>

    }
}

fn main() {
    yew::start_app::<App>();
}
