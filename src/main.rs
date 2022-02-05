use std::vec;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{classes, html, Properties, TargetCast};

enum cell_types {
    CORRECTPOS,
    INCORRECTPOS,
    INCORRECT,
}

struct grid_cell {
    cell_type: cell_types,
    cell_value: String,
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub words: String,
}

#[function_component(App)]
fn app() -> Html {
    let row_index = 0;
    let won = use_state(|| false);
    let mut current_guess = use_state(|| "".to_string());
    let grid_state = use_state(|| vec!["     ".to_string(); 6]);
    let oninput = {
        let current_guess = current_guess.clone();
        println!("here");
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_guess.set(input.value());
        })
    };

    let onclick = {
        let current_guess = current_guess.clone();
        let grid_state = grid_state.clone();

        Callback::from(move |e: MouseEvent| {
            let l = current_guess.to_string();
            let mut t = grid_state.clone().to_vec();
            t[row_index] = l;
            grid_state.set(t);
        })
    };

    let mut grid = grid_state
        .iter()
        .map(|word: &String| {
            html! {<Word_Row word_guessed={word.clone()}/>}
        })
        .collect::<Vec<_>>();

    html! {
        <div  class={classes!("app")}>
            <h1>{"RUST WORDLE"}</h1>

            <div class={classes!("words-container")}>{grid}</div>
            <div>
                <input {oninput} type="text" minlength="5" maxlength="5"  value={{(*current_guess).clone()}}/>
                <h1>{{(*current_guess).clone()}}</h1>

                <button {onclick} type="button"> {"Press to guess"} </button>
            </div>

        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct word_row_props {
    pub word_guessed: String,
}
#[function_component(Word_Row)]
fn word_row(props: &word_row_props) -> Html {
    let row = props
        .word_guessed
        .split_terminator("")
        .skip(1)
        .map(|letter| {
            html! {<div class="letter-container"><p class="letter">{letter.to_uppercase()}</p></div>}
        })
        .collect::<Vec<_>>();
    println!("{:#?}", props.word_guessed);
    html! {
        <div class={classes!("word-row-container")}>
            {row}
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
