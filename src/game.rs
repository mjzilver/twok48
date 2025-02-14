use rand::{seq::SliceRandom, thread_rng};

use crate::models::*;

fn is_within_bounds(i: usize, j: usize, row_offset: i8, col_offset: i8) -> bool {
    let next_i = (i as i8 + row_offset) as usize;
    let next_j = (j as i8 + col_offset) as usize;

    next_i < ROWS && next_j < COLS
}

pub fn move_matrix(matrix: &mut Matrix, dir: Direction) {
    reset_tile_states(matrix);

    let (row_offset, col_offset) = dir.to_tuple();
    let mut merged = [[false; COLS]; ROWS];
    let mut has_changed = false;

    let row_range = get_iteration_range(row_offset, ROWS);
    for i in row_range {
        let col_range = get_iteration_range(col_offset, COLS);
        for j in col_range {
            if matrix[i][j].value == 0 {
                continue;
            }

            let (mut current_i, mut current_j) = (i, j);
            if move_tile(
                matrix,
                &mut current_i,
                &mut current_j,
                row_offset,
                col_offset,
            ) {
                has_changed = true;
            }
            if merge_tiles(
                matrix,
                &mut merged,
                current_i,
                current_j,
                row_offset,
                col_offset,
            ) {
                has_changed = true;
            }
        }
    }

    if has_changed {
        spawn_tile(matrix);
    }
}

fn get_iteration_range(offset: i8, size: usize) -> Box<dyn Iterator<Item = usize>> {
    if offset == 1 {
        Box::new((0..size).rev())
    } else {
        Box::new(0..size)
    }
}

fn move_tile(
    matrix: &mut Matrix,
    current_i: &mut usize,
    current_j: &mut usize,
    row_offset: i8,
    col_offset: i8,
) -> bool {
    let mut has_moved = false;

    while is_within_bounds(*current_i, *current_j, row_offset, col_offset) {
        let next_i = (*current_i as i8 + row_offset) as usize;
        let next_j = (*current_j as i8 + col_offset) as usize;

        if matrix[next_i][next_j].value == 0 {
            let mut moving_tile = matrix[*current_i][*current_j];
            moving_tile.state = Some(CellState::Moved);
            if moving_tile.prev.is_none() {
                moving_tile.prev = Some((*current_i as isize, *current_j as isize));
            }

            matrix[next_i][next_j] = moving_tile;
            matrix[*current_i][*current_j] = Cell {
                value: 0,
                state: None,
                prev: None,
            };

            *current_i = next_i;
            *current_j = next_j;
            has_moved = true;
        } else {
            break;
        }
    }

    has_moved
}

fn merge_tiles(
    matrix: &mut Matrix,
    merged: &mut [[bool; COLS]; ROWS],
    current_i: usize,
    current_j: usize,
    row_offset: i8,
    col_offset: i8,
) -> bool {
    if is_within_bounds(current_i, current_j, row_offset, col_offset) {
        let next_i = (current_i as i8 + row_offset) as usize;
        let next_j = (current_j as i8 + col_offset) as usize;

        if matrix[current_i][current_j] == matrix[next_i][next_j] && !merged[current_i][current_j] {
            matrix[current_i][current_j].value = 0;
            merged[current_i][current_j] = true;

            let next_cell = &mut matrix[next_i][next_j];
            next_cell.value *= 2;
            next_cell.state = Some(CellState::Merged);
            if next_cell.prev.is_none() {
                next_cell.prev = Some((current_i as isize, current_j as isize));
            }

            return true;
        }
    }
    false
}

pub fn spawn_tile(matrix: &mut Matrix) {
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

pub fn move_left(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Left)
}

pub fn move_right(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Right)
}

pub fn move_up(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Up)
}

pub fn move_down(matrix: &mut Matrix) {
    move_matrix(matrix, Direction::Down)
}

pub fn get_class_for_score(score: u16) -> String {
    return format!("cell-{}", score);
}
