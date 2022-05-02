use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{classes, html, Properties, TargetCast};

#[derive(Debug, Clone)]
enum cell_types {
    CORRECTPOS,
    INCORRECTPOS,
    INCORRECT,
    EMPTY,
}

#[derive(Debug, Clone)]
struct grid_cell {
    pub cell_type: cell_types,
    pub cell_value: String,
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub words: String,
}

fn deterimine_cell_type(word_to_guess: &str, letter: &char, idx: &usize) -> cell_types {
    let found = word_to_guess.find(*letter);
    match found {
        Some(found) => {
            if found == *idx {
                return cell_types::CORRECTPOS;
            }
            return cell_types::INCORRECTPOS;
        }
        None => return cell_types::INCORRECT,
    }
}

#[function_component(App)]
fn app() -> Html {
    let row_index = use_state(|| 0);
    let won = use_state(|| false);
    let word_to_guess = use_state(|| "train");
    let current_guess = use_state(|| "".to_string());

    let grid_state = use_state(|| {
        vec![
            grid_cell {
                cell_type: cell_types::EMPTY,
                cell_value: " ".to_string(),
            };
            30
        ]
    });

    let grid = grid_state
        .iter()
        .map(|cell| {
            let cell_type = cell.cell_type.clone();
            let class_name: &str;
            match cell_type {
                cell_types::INCORRECT => class_name = "letter-container-incorrect withval",
                cell_types::CORRECTPOS => class_name = "letter-container-correct withval",
                cell_types::INCORRECTPOS => class_name = "letter-container-incorrectPos withval",
                _ => class_name = "letter-container",
            }
            html! {<div class={class_name}><p class="letter">{cell.cell_value.clone()}</p></div>}
        })
        .collect::<Vec<_>>();

    let oninput = {
        let current_guess = current_guess.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_guess.set(input.value());
        })
    };

    let handleguess = {
        let current_guess = current_guess.clone();
        let won = won.clone();
        let grid_state = grid_state.clone();
        let row_index = row_index.clone();
        Callback::from(move |_| {
            if *current_guess == *word_to_guess {
                won.set(true)
            }
            let mut t = grid_state.clone().to_vec();
            for (i, c) in current_guess.chars().enumerate() {
                let cell_t = deterimine_cell_type(&word_to_guess, &c, &i);
                let l = grid_cell {
                    cell_type: cell_t,
                    cell_value: c.to_ascii_uppercase().to_string(),
                };
                t[i + (*row_index * 5)] = l;
            }
            row_index.set(*row_index + 1);
            grid_state.set(t);
        })
    };

    let handelReset = {
        let won = won.clone();
        let row_index = row_index.clone();
        let grid_state = grid_state.clone();
        Callback::from(move |_: MouseEvent| {
            won.set(false);
            row_index.set(0);
            grid_state.set(vec![
                grid_cell {
                    cell_type: cell_types::EMPTY,
                    cell_value: " ".to_string(),
                };
                30
            ]);
        })
    };

    html! {
        <div  class={classes!("app")}>
            <h1>{"RUST WORDLE"}</h1>

            <div class={classes!("words-container")}><div class={classes!("word-row-container")}>{grid}</div></div>
            <div>
                <input {oninput} type="text" minlength="5" maxlength="5"  value={{(*current_guess).clone()}}/>
                <h1>{{(*current_guess).clone()}}</h1>
            </div>
            <button onclick={handleguess} disabled={*won} type="button"> {"Press to guess"} </button>
            if *won==true{
                <Modal reset={&handelReset}/>
            }

        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct modalProps {
    pub reset: Callback<MouseEvent>,
}

#[function_component(Modal)]
pub fn modal(props: &modalProps) -> Html {
    html! {
        <div class="modal">
            <div class="content">
                <h1>{"Congrats You won!"}</h1>
                <button onclick={&props.reset}>{"click here to reset"}</button>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
