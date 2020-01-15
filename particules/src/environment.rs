use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled,
}

pub struct Environment {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Environment {
    pub fn is_out_of_bound(&self, row: u32, column: u32) -> bool {
        if row > self.width || column > self.height {
            false
        } else {
            true
        }
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Empty { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
