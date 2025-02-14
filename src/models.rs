use crate::game::spawn_tile;

pub static ROWS: usize = 4;
pub static COLS: usize = 4;
pub static TILE_SIZE: isize = 125;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn to_tuple(&self) -> (i8, i8) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CellState {
    New,
    Merged,
    Moved,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub value: u16,
    pub state: Option<CellState>,
    pub prev: Option<(isize, isize)>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub type Matrix = [[Cell; COLS]; ROWS];

pub fn generate_start_matrix() -> Matrix {
    let mut start: Matrix = [[Cell {
        value: 0,
        state: None,
        prev: None,
    }; COLS]; ROWS];

    spawn_tile(&mut start);
    spawn_tile(&mut start);

    start
}

pub fn reset_tile_states(matrix: &mut Matrix) {
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            cell.state = None;
            cell.prev = None;
        }
    }
}
