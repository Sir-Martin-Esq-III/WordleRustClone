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
    let test = use_state(|| {
        vec![
            "t".to_string(),
            "t".to_string(),
            "t".to_string(),
            "t".to_string(),
        ]
    });
    let grid_state = use_state(|| vec![vec!["\n".to_string(); 5]; 6]);

    let oninput = {
        let current_guess = current_guess.clone();
        println!("here");
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_guess.set(input.value());
        })
    };

    let onclick = {
        let test = test.clone();
        let current_guess = current_guess.clone();
        let grid_state = grid_state.clone();

        Callback::from(move |e: MouseEvent| {
            // let f: String = String::from("TESTS");
            // let t = f.split("").collect();
            // let l = current_guess
            //     .split("")
            //     .map(|letter| letter.to_string())
            //     .collect();

            // test.set(l)
            let l = vec![vec!["G".to_string(); 5]; 6];
            grid_state.set(l);
        })
    };
    let mut grid = grid_state
        .iter()
        .map(|row: &Vec<String>| {
            html! {<Word_Row word_guessed={row.clone()}/>}
        })
        .collect::<Vec<_>>();

    html! {
        <div  class={classes!("app")}>
            <h1>{"RUST WORDLE"}</h1>
            <div class={classes!("words-container")}>

            <h1>{&grid_state[0][0]}</h1>
            <div>{grid}</div>
            </div>
            <div>
            {for test.iter().map(|letter|{html!{
               <h1>{letter}</h1>
            }})}
            </div>

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
    pub word_guessed: Vec<String>,
}
#[function_component(Word_Row)]
fn word_row(props: &word_row_props) -> Html {
    println!("{:#?}", props.word_guessed);
    let word_guesses = use_state(|| props.word_guessed.clone());
    html! {
        <div class={classes!("word-row-container")}>
        {for word_guesses.iter().map(|letter|{html!{<div class="letter-container"><p class="letter">{letter}</p></div>}})}
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
