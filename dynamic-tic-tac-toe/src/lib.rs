use std::ffi::c_int;

use crate::domain::cell::{Cell, CellType};
use crate::domain::game::{Game, GameStatus, Vec2};

pub mod domain;

pub mod domains {
    pub use crate::domain::cell::Cell;
    pub use crate::domain::cell::CellType;
    pub use crate::domain::game::Game;
    pub use crate::domain::game::GameStatus;
    pub use crate::domain::game::Vec2;
}


#[no_mangle]
pub extern "C" fn create_game(size: c_int, first_cell_type: CellType) -> FfiGame {
    let game = Game::new(size as u32, first_cell_type);
    FfiGame {
        game: Box::into_raw(Box::new(game))
    }
}

#[no_mangle]
pub extern "C" fn next(game: FfiGame, pos: Vec2) -> GameStatus {
    let game = unsafe { &mut *game.game };
    if let Ok(status) = game.next(pos) {
        status
    } else {
        GameStatus::Continue
    }
}

#[no_mangle]
pub extern "C" fn bind(game: FfiGame, listener: extern "C" fn(FfiCell)) {
    let listener_wrapper = move |cell: &Cell| {
        let ffi_cell = FfiCell {
            cell_type: cell.cell_type,
            position: cell.position,
            history_count: cell.history_count as c_int,
        };
        listener(ffi_cell);
    };
    let game = unsafe { &mut *game.game };
    game.bind(listener_wrapper)
}

#[repr(C)]
pub struct FfiGame {
    game: *mut Game,
}

#[repr(C)]
pub struct FfiCell {
    cell_type: CellType,
    position: Vec2,
    history_count: c_int,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        let game = Game::new(3, CellType::O);
        assert_eq!(game.board.size, 3);
        assert_eq!(game.current_cell, CellType::O);

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(game.board.cells[i][j].cell_type, CellType::Empty);
            }
        }
    }

    #[test]
    fn one_turn() {
        let mut game = Game::new(3, CellType::O);
        game.next(Vec2 { x: 0, y: 0 });
        assert_eq!(game.board.cells[0][0].cell_type, CellType::O);
        assert_eq!(game.current_cell, CellType::X);
    }

    #[test]
    fn win() {
        let mut game = Game::new(3, CellType::O);
        game.next(Vec2 { x: 0, y: 0 });
        game.next(Vec2 { x: 0, y: 1 });
        game.next(Vec2 { x: 1, y: 0 });
        game.next(Vec2 { x: 0, y: 2 });
        let result = game.next(Vec2 { x: 2, y: 0 }).unwrap();
        assert_eq!(result, GameStatus::WinO);
    }

    #[test]
    #[should_panic]
    fn put_to_occupied_cell() {
        let mut game = Game::new(3, CellType::O);
        game.next(Vec2 { x: 0, y: 0 });
        game.next(Vec2 { x: 0, y: 0 });
    }

    #[test]
    fn deactivation() {
        let mut game = Game::new(3, CellType::O);
        game.next(Vec2 { x: 0, y: 0 });
        assert_eq!(game.board.cells[0][0].cell_type, CellType::O);
        game.next(Vec2 { x: 0, y: 1 });
        println!("{}", game);
        game.next(Vec2 { x: 1, y: 0 });
        game.next(Vec2 { x: 0, y: 2 });
        game.next(Vec2 { x: 1, y: 1 });
        game.next(Vec2 { x: 2, y: 0 });
        game.next(Vec2 { x: 2, y: 1 });
        assert_eq!(game.board.cells[0][0].cell_type, CellType::Empty);
    }

    #[test]
    fn bind() {
        let mut game = Game::new(3, CellType::O);
        game.bind(move |cell| {
            println!("Cell is changed to {} pos: {:?}", cell, cell.position);
        });
        game.next(Vec2 { x: 0, y: 0 });
        game.next(Vec2 { x: 0, y: 1 });
        game.next(Vec2 { x: 1, y: 0 });
        game.next(Vec2 { x: 0, y: 2 });
        game.next(Vec2 { x: 1, y: 1 });
        game.next(Vec2 { x: 2, y: 0 });
        game.next(Vec2 { x: 2, y: 1 });

        println!("{}", game);
    }
}