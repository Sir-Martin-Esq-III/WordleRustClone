use yew::prelude::*;
use yew::{classes, html};

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub words: String,
}

#[function_component(App)]
fn app() -> Html {
    let row_index = use_state(|| 0);
    let won = use_state(|| false);
    let grid_state = use_state(|| vec![vec!["", "", "", "", ""]; 6]);

    let test_word = "tests";

    html! {
        <div  class={classes!("app")}>
            <h1>{"RUST WORDLE"}</h1>

            <div class={classes!("words-container")}>
                {for grid_state.iter().map(|lettersVec|{html!{
                    <Word_Row/>
                }})}
            </div>



            <div>
                <input type="text"/>
                <button type="button"> {"Press to guess"} </button>
            </div>

        </div>
    }
}

// enum cell_types{
//     CorrectPos="correctPos";
//     incorrectPos="incorrectPos",
//     incorrect="incorrect",
// }

pub struct word_obj {
    pub cellType: String,
    pub value: char,
}

#[derive(Properties, PartialEq)]
pub struct word_row_props {
    pub word_guessed: Vec<String>,
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

fn main() {
    yew::start_app::<App>();
}
