mod utils;

use std::fmt::Display;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initialize() {
    wasm_logger::init(wasm_logger::Config::default());
    utils::set_panic_hook();
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,

    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        [self.height - 1, 0, 1]
            .iter()
            .flat_map(|dx| [self.height - 1, 0, 1].map(|dy| (*dx, dy)))
            .fold(0, |count, (dx, dy)| {
                if dx == 0 && dy == 0 {
                    return count;
                }

                let x_to_check = (x + dx) % self.width;
                let y_to_check = (y + dy) % self.height;
                let idx = self.get_index(x_to_check, y_to_check);

                count
                    + match self.cells[idx] {
                        Cell::Alive => 1,
                        Cell::Dead => 0,
                    }
            })
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (cell, _) => cell,
                };

                next_cells[idx] = next_cell;
            }
        }

        self.cells = next_cells;
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.as_slice().chunks(self.width) {
            for &cell in row {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[rustfmt::skip]
    const CELLS_3X3_DEAD: [Cell;9] = [
        Cell::Dead, Cell::Dead, Cell::Dead ,
        Cell::Dead, Cell::Dead, Cell::Dead ,
        Cell::Dead, Cell::Dead, Cell::Dead ,
    ];

    #[rustfmt::skip]
    const CELLS_3X3_LIVING_TOP_BOTTOM: [Cell;9] = [
        Cell::Alive, Cell::Alive, Cell::Alive,
        Cell::Dead , Cell::Dead , Cell::Dead ,
        Cell::Alive, Cell::Alive, Cell::Alive,
    ];

    #[rustfmt::skip]
    const CELLS_5X5_BLINKER: [Cell;25] = [
        Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead ,
        Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead ,
        Cell::Dead , Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead ,
        Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead ,
        Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead ,
    ];

    #[rustfmt::skip]
    const CELLS_5X5_BLINKER_BLINKED: [Cell;25] = [
        Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead ,
        Cell::Dead , Cell::Dead , Cell::Alive, Cell::Dead , Cell::Dead ,
        Cell::Dead , Cell::Dead , Cell::Alive, Cell::Dead , Cell::Dead ,
        Cell::Dead , Cell::Dead , Cell::Alive, Cell::Dead , Cell::Dead ,
        Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead , Cell::Dead ,
    ];

    fn create_universe_3x3_dead() -> Universe {
        Universe {
            width: 3,
            height: 3,
            cells: CELLS_3X3_DEAD.into(),
        }
    }

    fn create_universe_3x3_living_top_and_bottom_rows() -> Universe {
        Universe {
            width: 3,
            height: 3,
            cells: CELLS_3X3_LIVING_TOP_BOTTOM.into(),
        }
    }

    #[test]
    fn get_index_at_0_0_should_be_0() {
        let universe = create_universe_3x3_dead();
        assert_eq!(0, universe.get_index(0, 0));
    }

    #[test]
    fn get_index_at_1_0_should_be_0() {
        let universe = create_universe_3x3_dead();
        assert_eq!(1, universe.get_index(1, 0));
    }

    #[test]
    fn get_index_at_0_1_should_be_3() {
        let universe = create_universe_3x3_dead();
        assert_eq!(3, universe.get_index(0, 1));
    }

    #[test]
    fn get_index_at_1_1_should_be_4() {
        let universe = create_universe_3x3_dead();
        assert_eq!(4, universe.get_index(1, 1));
    }

    #[test]
    fn live_neighbor_count_at_0_0_with_all_dead_cells_should_be_0() {
        let universe = create_universe_3x3_dead();
        let count = universe.live_neighbor_count(0, 0);

        assert_eq!(0, count);
    }

    #[test]
    fn live_neighbor_count_at_2_0_with_all_dead_cells_should_be_0() {
        let universe = create_universe_3x3_dead();
        let count = universe.live_neighbor_count(2, 0);

        assert_eq!(0, count);
    }

    #[test]
    fn live_neighbor_count_at_0_2_with_all_dead_cells_should_be_0() {
        let universe = create_universe_3x3_dead();
        let count = universe.live_neighbor_count(0, 2);

        assert_eq!(0, count);
    }

    #[test]
    fn live_neighbor_count_at_2_2_with_all_dead_cells_should_be_0() {
        let universe = create_universe_3x3_dead();
        let count = universe.live_neighbor_count(2, 2);

        assert_eq!(0, count);
    }

    #[test]
    fn live_neighbor_count_at_0_0_with_living_top_and_bottom_rows_should_be_5() {
        let universe = create_universe_3x3_living_top_and_bottom_rows();
        let count = universe.live_neighbor_count(0, 0);

        assert_eq!(5, count);
    }

    #[test]
    fn live_neighbor_count_at_1_1_with_living_top_and_bottom_rows_should_be_6() {
        let universe = create_universe_3x3_living_top_and_bottom_rows();
        let count = universe.live_neighbor_count(1, 1);

        assert_eq!(6, count);
    }

    #[test]
    fn live_neighbor_count_at_2_2_with_living_top_and_bottom_rows_should_be_5() {
        let universe = create_universe_3x3_living_top_and_bottom_rows();
        let count = universe.live_neighbor_count(2, 2);

        assert_eq!(5, count);
    }

    #[test]
    fn tick_with_all_dead_cells_should_not_change() {
        let mut universe = create_universe_3x3_dead();
        universe.tick();

        assert_eq!(CELLS_3X3_DEAD.to_vec(), universe.cells)
    }

    #[test]
    fn tick_with_living_top_and_bottom_rows_should_all_die_from_overpopulation() {
        let mut universe = create_universe_3x3_living_top_and_bottom_rows();
        universe.tick();

        assert_eq!(CELLS_3X3_DEAD.to_vec(), universe.cells)
    }

    #[test]
    fn tick_with_blinker_pattern_should_blink_back_and_forth() {
        let mut universe = Universe {
            width: 5,
            height: 5,
            cells: CELLS_5X5_BLINKER.to_vec(),
        };

        universe.tick();
        assert_eq!(CELLS_5X5_BLINKER_BLINKED.to_vec(), universe.cells);

        universe.tick();
        assert_eq!(CELLS_5X5_BLINKER.to_vec(), universe.cells);
    }
}
