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
    pub fn is_out_of_bound_v(&self, y: u32) -> bool {
        if y > self.height -1 {
            true
        } else {
            false
        }
    }

    pub fn is_out_of_bound_h(&self, x: u32) -> bool {
        if x > self.width -1 {
            true
        } else {
            false
        }
    }

    pub fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = vec![];
        let size = width * height;

        for _ in 0..size {
            cells.push(Cell::Empty);
        }

        Environment{ width, height, cells }
    }
}

#[cfg(test)]
pub mod test {
    use crate::environment::Environment;

    #[test]
    fn should_create_env_filled_with_empty_cells() {
        let env = Environment::new(5, 5);

        assert_eq!(env.cells.len(), 25);
    }
    #[test]
    fn should_return_out_of_bound() {
        let env = Environment::new(5, 5);

        assert!(env.is_out_of_bound_h(0));
        assert!(env.is_out_of_bound_v(0));

        assert!(env.is_out_of_bound_h(5));
        assert!(env.is_out_of_bound_v(5));
    }

}
