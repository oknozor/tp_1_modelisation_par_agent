use super::AgentRef;
use super::Point;

#[derive(Clone)]
pub enum Cell {
    Empty,
    Filled(AgentRef),
}

pub struct Environment {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) cells: Vec<Cell>,
    pub(crate) borderless: bool,
}

impl Environment {
    pub fn is_out_of_bound_y(&self, y: i32) -> bool {
        if self.borderless {
            false
        } else if y > self.height - 1 || y < 0 {
            true
        } else {
            false
        }
    }

    pub fn is_out_of_bound_x(&self, x: i32) -> bool {
        if self.borderless {
            false
        } else if x > self.width - 1 || x < 0 {
            true
        } else {
            false
        }
    }

    fn out_of_bound(&self, point: Point) -> bool {
        self.is_out_of_bound_x(point.x) || self.is_out_of_bound_y(point.y)
    }

    pub fn get_index(&self, point: Point) -> usize {
        (point.y * self.width + point.x) as usize
    }

    pub fn get_cell(&self, point: Point) -> Option<&Cell> {
        if self.out_of_bound(point) {
            None
        } else {
            let idx = self.get_index(point);
            Some(&self.cells[idx])
        }
    }

    pub fn set_cell(&mut self, point: Point, cell: Cell) -> Result<(), &str> {
        if self.out_of_bound(point) {
            Err("Cannot set cell out of bounds!")
        } else {
            let idx = self.get_index(point);
            self.cells[idx] = cell;
            Ok(())
        }
    }

    pub fn new(width: i32, height: i32, borderless: bool) -> Self {
        let mut cells = vec![];
        let size = width * height;

        for _ in 0..size {
            cells.push(Cell::Empty);
        }

        Environment {
            width,
            height,
            cells,
            borderless,
        }
    }
}
