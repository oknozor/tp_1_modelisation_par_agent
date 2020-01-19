use crate::agent::Agent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled(Rc<RefCell<Agent>>),
}

pub struct Environment {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) cells: Vec<Cell>,
}

impl Environment {
    pub fn is_out_of_bound_y(&self, y: u32) -> bool {
        if y > self.height - 1 {
            true
        } else {
            false
        }
    }

    pub fn is_out_of_bound_x(&self, x: u32) -> bool {
        if x > self.width - 1 {
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

        Environment {
            width,
            height,
            cells,
        }
    }
}