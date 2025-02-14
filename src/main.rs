use rand::prelude::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;
use yew::{classes, html};

static ROWS: usize = 4;
static COLS: usize = 4;
static TILE_SIZE: isize = 125;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_tuple(&self) -> (i8, i8) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CellState {
    New,
    Merged,
    Moved,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    value: u16,
    state: Option<CellState>,
    prev: Option<(isize, isize)>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

type Matrix = [[Cell; COLS]; ROWS];

#[function_component]
fn App() -> Html {
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

fn reset_tile_states(matrix: &mut Matrix) {
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            cell.state = None;
            cell.prev = None;
        }
    }
}

fn generate_start_matrix() -> Matrix {
    let mut start: Matrix = [[Cell {
        value: 0,
        state: None,
        prev: None,
    }; COLS]; ROWS];

    spawn_tile(&mut start);
    spawn_tile(&mut start);

    start
}

fn is_within_bounds(i: usize, j: usize, row_offset: i8, col_offset: i8) -> bool {
    let next_i = (i as i8 + row_offset) as usize;
    let next_j = (j as i8 + col_offset) as usize;

    next_i < ROWS && next_j < COLS
}

fn move_matrix(matrix: &mut Matrix, dir: Direction) {
    reset_tile_states(matrix);

    let (row_offset, col_offset) = dir.to_tuple();

    // track merged tiles
    let mut merged = [[false; COLS]; ROWS];
    let mut has_changed = false;

    let row_range: Box<dyn Iterator<Item = usize>> = if row_offset == 1 {
        Box::new((0..ROWS).rev())
    } else {
        Box::new(0..ROWS)
    };

    for i in row_range {
        let col_range: Box<dyn Iterator<Item = usize>> = if col_offset == 1 {
            Box::new((0..COLS).rev())
        } else {
            Box::new(0..COLS)
        };

        for j in col_range {
            // Skip
            if matrix[i][j].value == 0 {
                continue;
            }

            let mut current_i = i;
            let mut current_j = j;

            // Move
            while is_within_bounds(current_i, current_j, row_offset, col_offset) {
                let next_i = (current_i as i8 + row_offset) as usize;
                let next_j = (current_j as i8 + col_offset) as usize;

                if matrix[next_i][next_j].value == 0 {
                    let mut moving_tile: Cell = matrix[current_i][current_j];
                    moving_tile.state = Some(CellState::Moved);
                    if moving_tile.prev.is_none() {
                        moving_tile.prev = Some((current_i as isize, current_j as isize));
                    }

                    matrix[next_i][next_j] = moving_tile;
                    matrix[current_i][current_j] = Cell {
                        value: 0,
                        state: None,
                        prev: None,
                    };

                    current_i = next_i;
                    current_j = next_j;
                    has_changed = true;
                } else {
                    break;
                }
            }

            // Combine
            if is_within_bounds(current_i, current_j, row_offset, col_offset) {
                let next_i = (current_i as i8 + row_offset) as usize;
                let next_j = (current_j as i8 + col_offset) as usize;

                if matrix[current_i][current_j] == matrix[next_i][next_j]
                    && !merged[current_i][current_j]
                {
                    matrix[current_i][current_j].value = 0;
                    merged[current_i][current_j] = true;

                    matrix[next_i][next_j].value *= 2;
                    matrix[next_i][next_j].state = Some(CellState::Merged);
                    if matrix[next_i][next_j].prev.is_none() {
                        matrix[next_i][next_j].prev = Some((current_i as isize, current_j as isize));
                    }

                    has_changed = true;
                }
            }
        }
    }

    if has_changed {
        spawn_tile(matrix);
    }
}

fn spawn_tile(matrix: &mut Matrix) {
    let mut empty_cells = vec![];

    for i in 0..ROWS {
        for j in 0..COLS {
            if matrix[i][j].value == 0 {
                empty_cells.push((i, j));
            }
        }
    }

    if let Some(&(x, y)) = empty_cells.choose(&mut thread_rng()) {
        matrix[x][y].value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };
        matrix[x][y].state = Some(CellState::New);
    }
}

fn move_left(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Left)
}

fn move_right(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Right)
}

fn move_up(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Up)
}
fn move_down(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Down)
}

fn get_class_for_score(score: u16) -> String {
    return format!("cell-{}", score);
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
