use std::fmt::Display;
use std::sync::Arc;

use crate::domain::board::Board;
use crate::domain::cell::{Cell, CellType};

pub struct Game {
    pub board: Board,
    pub current_cell: CellType,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum GameStatus {
    Continue,
    WinO,
    WinX,
    Draw,
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum GameError {
    CellOccupied,
    GameFinished,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Game {
    pub fn new(size: u32, first_cell_type: CellType) -> Game {
        Game {
            board: Board::new(size as usize),
            current_cell: first_cell_type,
        }
    }

    pub fn bind<F>(&mut self, listener: F)
    where
        F: Fn(&Cell) + Send + Sync + 'static,
    {
        let listener = Arc::new(listener);
        self.board.cells.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                cell.add_listener(listener.clone());
            });
        });
    }

    pub fn next(&mut self, pos: Vec2) -> Result<GameStatus, GameError> {
        if let Some(winner) = self.board.judge() {
            return match winner {
                CellType::O => Ok(GameStatus::WinO),
                CellType::X => Ok(GameStatus::WinX),
                _ => Err(GameError::GameFinished),
            };
        }

        if let Err(err) = &self.board.put(pos, &self.current_cell) {
            return Err(*err);
        }

        self.board.get_with_history(self.board.size)
            .iter_mut()
            .filter(|cell| cell.cell_type != CellType::Empty)
            .for_each(|cell| cell.deactivate());

        if let Some(winner) = self.board.judge() {
            return match winner {
                CellType::O => Ok(GameStatus::WinO),
                CellType::X => Ok(GameStatus::WinX),
                _ => Err(GameError::GameFinished),
            };
        }

        self.current_cell = self.current_cell.next();

        if self.board.is_full() {
            return Ok(GameStatus::Draw);
        }
        Ok(GameStatus::Continue)
    }
}
