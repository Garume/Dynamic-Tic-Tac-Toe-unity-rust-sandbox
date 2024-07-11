use std::fmt::Display;
use std::sync::{Arc, Mutex};

use crate::domain::game::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CellType {
    Empty,
    X,
    O,
}

impl CellType {
    pub fn next(&self) -> CellType {
        match self {
            CellType::Empty => CellType::Empty,
            CellType::X => CellType::O,
            CellType::O => CellType::X,
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub position: Vec2,
    pub history_count: usize,
    listeners: Arc<Mutex<Vec<Arc<dyn Fn(&Cell) + Send + Sync>>>>,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.cell_type {
            CellType::Empty => "_",
            CellType::X => "X",
            CellType::O => "O",
        };
        write!(f, "{}", c)
    }
}

impl Cell {
    pub fn new(position: Vec2) -> Cell {
        Cell {
            cell_type: CellType::Empty,
            position,
            history_count: 0,
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn put(&mut self, cell_type: &CellType) {
        self.cell_type = *cell_type;
        self.notify();
    }

    pub fn deactivate(&mut self) {
        self.history_count = 0;
        self.cell_type = CellType::Empty;
        self.notify();
    }

    pub fn add_listener(&mut self, listener: Arc<dyn Fn(&Cell) + Send + Sync>) {
        self.listeners.lock().unwrap().push(listener);
    }

    pub fn notify(&self) {
        let listeners = self.listeners.lock();
        if listeners.is_err() {
            return;
        }
        for listener in listeners.unwrap().iter() {
            listener(self);
        }
    }
}
