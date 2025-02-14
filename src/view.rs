use crate::{game::*, models::*};

use yew::prelude::*;
use yew::{classes, html};

#[function_component]
pub(crate) fn App() -> Html {
    let matrix: UseStateHandle<Matrix> = use_state(|| generate_start_matrix());

    let on_key_down = {
        let matrix = matrix.clone();
        Callback::from(move |event: KeyboardEvent| {
            let mut current_matrix = *matrix;
            match event.key().as_str() {
                "ArrowLeft" => move_left(&mut current_matrix),
                "ArrowRight" => move_right(&mut current_matrix),
                "ArrowUp" => move_up(&mut current_matrix),
                "ArrowDown" => move_down(&mut current_matrix),
                _ => return,
            }
            matrix.set(current_matrix);
        })
    };

    let container_ref = use_node_ref();

    {
        let container_ref = container_ref.clone();
        use_effect(move || {
            if let Some(element) = container_ref.cast::<web_sys::HtmlElement>() {
                element.focus().unwrap();
            }
        });
    }

    html! {
        <div class="container" tabindex="0" ref={container_ref} onkeydown={on_key_down}>
            <div class="field">
                { for matrix.iter().enumerate().map(|(i, row)| render_row(row, i)) }
            </div>
        </div>
    }
}

fn render_row(row: &[Cell], i: usize) -> Html {
    html! {
        for row.iter().enumerate().map(|(j, cell)| render_tile(cell, i, j))
    }
}

fn render_tile(cell: &Cell, i: usize, j: usize) -> Html {
    let mut classes = classes!("inner-cell", get_class_for_score(cell.value));
    let mut extra_styles = String::new();

    if let Some(state) = cell.state {
        match state {
            CellState::New => classes.push("cell-new"),
            CellState::Merged => classes.push("cell-merged cell-moved"),
            CellState::Moved => classes.push("cell-moved"),
        }
    }

    if let Some((prev_i, prev_j)) = cell.prev {
        let from_x = (prev_j - j as isize) * TILE_SIZE;
        let from_y = (prev_i - i as isize) * TILE_SIZE;

        extra_styles.push_str(&format!("--from-x: {}px; --from-y: {}px;", from_x, from_y))
    };

    html! {
        <div class="cell">
            <div class={classes}
                style={extra_styles}>
                {
                    if cell.value == 0 { "".to_string() }
                    else { cell.value.to_string() }
                }
            </div>
        </div>
    }
}
