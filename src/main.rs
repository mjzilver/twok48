use rand::prelude::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;
use yew::{classes, html};

static ROWS: usize = 4;
static COLS: usize = 4;

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
    dir: Option<Direction>
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

type Matrix = [[Cell; COLS]; ROWS];

#[function_component]
fn App() -> Html {
    let matrix: UseStateHandle<Matrix> = use_state(|| {
        generate_start_matrix()
    });

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
                {
                    matrix.iter().map(|row| {
                        html! {
                            <div class="row">
                                {
                                    row.iter().map(|cell| {
                                        let mut classes = classes!("cell", get_class_for_score(cell.value));

                                        if let Some(state) = cell.state {
                                            match state  {
                                                CellState::New => classes.push("cell-new"),
                                                CellState::Merged => classes.push("cell-merged"),
                                                _ => {},
                                            }
                                        }
              
                                        if let Some(direction) = cell.dir {
                                            match direction {
                                                Direction::Left => classes.push("cell-moved-left"),
                                                Direction::Right => classes.push("cell-moved-right"),
                                                Direction::Up => classes.push("cell-moved-up"),
                                                Direction::Down => classes.push("cell-moved-down"),
                                            }
                                        }    
    
                                        html! {
                                            <div class={classes}>
                                                { cell.value }
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }    
}

fn reset_tile_states(matrix: &mut Matrix) {
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            cell.state = None;
            cell.dir = None;
        }
    }
}

fn generate_start_matrix() -> Matrix {
    let mut start: Matrix = [[Cell { value: 0, state: None, dir: None}; COLS]; ROWS];

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
                    matrix[next_i][next_j] = matrix[current_i][current_j];
                    matrix[current_i][current_j] = Cell { value: 0, state: None, dir: None };
                    matrix[next_i][next_j].state = Some(CellState::Moved);
                    matrix[next_i][next_j].dir = Some(dir);

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
