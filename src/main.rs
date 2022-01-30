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
    let current_guess = use_state(|| "".to_string());
    let grid_state: UseStateHandle<Vec<Vec<String>>> =
        use_state(|| vec![vec!["".to_string(); 5]; 6]);

    let test_word = "tests";

    let oninput = {
        let current_guess = current_guess.clone();
        println!("here");
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_guess.set(input.value());
        })
    };

    // let onclick = {
    //     let mut grid_state = grid_state.clone();
    //     let new_data = vec!["G".to_string(); 5];
    //     grid_state[0] = new_data;

    //     Callback::from(move |e: MouseEvent| {
    //         grid_state.set(grid_state.to_vec());
    //     })
    // };

    html! {
        <div  class={classes!("app")}>
            <h1>{"RUST WORDLE"}</h1>

            <div class={classes!("words-container")}>
                {for grid_state.iter().map(|lettersVec|{html!{
                    <Word_Row word_guessed={lettersVec.clone()}/>
                }})}
            </div>
            <div>
                <input {oninput} type="text" minlength="5" maxlength="5"  value={{(*current_guess).clone()}}/>
                <h1>{{(*current_guess).clone()}}</h1>

                <button type="button"> {"Press to guess"} </button>
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
