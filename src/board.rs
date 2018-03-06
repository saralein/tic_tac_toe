#[derive(Debug, PartialEq, Clone)]
pub enum CellState {
    Empty,
    Cross,
    Nought,
}

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    cells: Vec<CellState>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            size,
            cells: vec![CellState::Empty; (size * size)],
        }
    }

    pub fn get_cells(&self) -> &Vec<CellState> {
        &self.cells
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn update(&mut self, cell_move: usize, state: CellState) {
        self.cells[cell_move] = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_empty_board() {
        let size = 3;
        let board = Board::new(size);
        assert_eq!(vec![CellState::Empty; (size * size)], board.cells);
        assert_eq!(3, board.size);
    }

    #[test]
    fn it_gets_board_cells() {
        let size = 3;
        let board = Board::new(size);
        assert_eq!(board.cells, *board.get_cells())
    }

    #[test]
    fn it_gets_size_of_board() {
        let size = 3;
        let board = Board::new(size);
        assert_eq!(board.size, board.get_size());
    }

    #[test]
    fn it_sets_board_cell_state() {
        let size = 3;
        let mut board = Board::new(size);
        board.update(0, CellState::Cross);
        board.update(5, CellState::Nought);
        assert_eq!(CellState::Cross, board.cells[0]);
        assert_eq!(CellState::Nought, board.cells[5]);
    }
}
