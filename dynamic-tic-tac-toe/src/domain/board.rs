use std::fmt::Display;

use crate::domain::cell::{Cell, CellType};
use crate::domain::game::{GameError, Vec2};

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub size: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{}", self.cells[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl Board {
    pub fn new(size: usize) -> Board {
        let cells = (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| Cell::new(Vec2 { x: j as i32, y: i as i32 }))
                    .collect()
            })
            .collect();
        Board {
            cells,
            size,
        }
    }

    pub fn put(&mut self, pos: Vec2, cell_type: &CellType) -> Result<(), GameError> {
        if pos.x >= self.size as i32 || pos.y >= self.size as i32 {
            return Err(GameError::CellOccupied);
        }

        if self.cells[pos.x as usize][pos.y as usize].cell_type == CellType::Empty {
            self.increment_history(*cell_type);
            self.cells[pos.x as usize][pos.y as usize].put(cell_type);
            Ok(())
        } else {
            Err(GameError::CellOccupied)
        }
    }

    pub fn increment_history(&mut self, cell_type: CellType) {
        self.cells
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .filter(|cell| cell.cell_type == cell_type)
            .for_each(|cell| cell.history_count += 1);
    }

    pub fn get_with_history(&mut self, history_count: usize) -> Vec<&mut Cell> {
        self.cells
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .filter(|cell| cell.history_count >= history_count)
            .collect()
    }

    pub fn judge(&self) -> Option<CellType> {
        for &cell_type in &[CellType::X, CellType::O] {
            for i in 0..self.size {
                if self.cells[i].iter().all(|cell| cell.cell_type == cell_type) {
                    return Some(cell_type);
                }
                if self.cells.iter().all(|row| row[i].cell_type == cell_type) {
                    return Some(cell_type);
                }
            }
            if (0..self.size).all(|i| self.cells[i][i].cell_type == cell_type) {
                return Some(cell_type);
            }
            if (0..self.size).all(|i| self.cells[i][self.size - i - 1].cell_type == cell_type) {
                return Some(cell_type);
            }
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|cell| cell.cell_type != CellType::Empty))
    }
}