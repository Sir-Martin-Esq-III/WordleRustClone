use yew::prelude::*;
use yew::{classes, html};

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub words: String,
}

#[function_component(Grid)]
fn grid() -> Html {
    let counter = use_state(|| 0);
    let row_index = use_state(|| 0);
    let test_word = "tests";

    html! {
        <div  class={classes!("app")}>
            <h1>{"RUST WORDLE"}</h1>

            <div class={classes!("words-container")}>
                <Word_Row/>
            </div>

            <div>
                <input type="text"/>
            </div>

        </div>
    }
}
