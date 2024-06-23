use crate::map::{Cell, Map};

impl Map {
    pub fn collect_resource(&mut self, x: usize, y: usize) -> Option<Cell> {
        let cell = self.grid[y][x];
        match cell {
            Cell::Energy | Cell::Mineral | Cell::Scientific => {
                self.grid[y][x] = Cell::Empty;
                Some(cell)
            }
            _ => None,
        }
    }
}
