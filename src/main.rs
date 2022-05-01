use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{classes, html, Properties, TargetCast};

#[derive(Debug, Clone,)]
enum cell_types {
    CORRECTPOS,
    INCORRECTPOS,
    INCORRECT,
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

fn determineCellType(word_to_guess:&str,letter:&char, idx:&usize)->cell_types{
    let found= word_to_guess.find(*letter);
    match found{
        Some(found)=>{
            if found==*idx{
                return cell_types::CORRECTPOS;
            }
            return cell_types::INCORRECTPOS;
        },
        None=>{
            return cell_types::INCORRECT
        }
    }

}

#[function_component(App)]
fn app() -> Html {
    let row_index = use_state(|| 0);
    let won = use_state(|| false);
    let word_to_guess = use_state(|| "train");
    let  current_guess = use_state(|| "".to_string());

    let grid_state = use_state(|| {
        vec![
            grid_cell {
                cell_type: cell_types::INCORRECT,
                cell_value: " ".to_string(),
            };
            30
        ]
    });

    let grid = grid_state
        .iter()
        .map(|cell| {
            let cell_type= cell.cell_type.clone();
            match cell_type{
                cell_types::INCORRECT=>html! {<div class="letter-container"><p class="letter">{cell.cell_value.clone()}</p></div>},
                cell_types::CORRECTPOS=>html! {<div class="letter-container-correct"><p class="letter">{cell.cell_value.clone()}</p></div>},
                cell_types::INCORRECTPOS=>html! {<div class="letter-container-incorrect"><p class="letter">{cell.cell_value.clone()}</p></div>},
            }           
        })
        .collect::<Vec<_>>();

    let oninput = {
        println!("{:?}", *grid_state);
        let current_guess = current_guess.clone();
        println!("here");
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_guess.set(input.value());
        })
    };

    let handleguess = {
        let current_guess = current_guess.clone();
        Callback::from(move |_| {
            let mut t = grid_state.clone().to_vec();
            for (i,c) in current_guess.chars().enumerate(){
                let cell_t=determineCellType(&word_to_guess,&c,&i);
                let l= grid_cell { cell_type:cell_t, cell_value:c.to_ascii_uppercase().to_string() };
                t[i+(*row_index*5)] = l;
            }
            row_index.set(*row_index + 1);
            grid_state.set(t);
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
            <button onclick={handleguess} type="button"> {"Press to guess"} </button>

        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
