use super::HDirection;
use super::VDirection;
use crate::environment::Cell;
use crate::environment::Environment;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    pub v_direction: VDirection,
    pub h_direction: HDirection,
    pub x: u32,
    pub y: u32,
    pub collision: bool,
}

impl Agent {
    pub fn draw(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Filled(Rc::new(RefCell::new(self.clone())));
    }

    pub fn clear(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Empty;
    }

    pub fn update(&mut self, environment: &mut Environment) {
        self.decide(environment);

        let (x_forward, y_forward) = self.look_ahead();

        self.y = y_forward;
        self.x = x_forward;
        self.draw(environment);
    }

    fn decide(&mut self, environment: &mut Environment) {
        self.clear(environment);
        self.collision = false;

        let (x_forward, y_forward) = self.look_ahead();

        let out_of_bound_h = self.x == 0 || environment.is_out_of_bound_h(x_forward);
        let out_of_bound_v = self.y == 0 || environment.is_out_of_bound_v(y_forward);

        if !out_of_bound_h && !out_of_bound_v {
            let forward_idx = environment.get_index(x_forward, y_forward);
            let cell_forward = environment.cells[forward_idx].clone();

            match cell_forward {
                Cell::Empty => (),
                Cell::Filled(agent) => {
                    self.collision = true;
                    agent.borrow_mut().collision = true;

                    // Swap agents directions
                    let direction_h = self.v_direction;
                    let direction_v = self.h_direction;
                    self.h_direction = agent.borrow().h_direction;
                    self.v_direction = agent.borrow().v_direction;
                    agent.borrow_mut().v_direction = direction_h;
                    agent.borrow_mut().h_direction = direction_v;
                }
            };
        }

        if out_of_bound_v {
            self.h_direction = self.h_direction.invert();
        }

        if out_of_bound_h {
            self.v_direction = self.v_direction.invert();
        }
    }

    fn look_ahead(&self) -> (u32, u32) {
        (
            match self.v_direction {
                VDirection::Up if self.x != 0 => self.x - 1,
                VDirection::Down => self.x + 1,
                _ => self.x,
            },
            match self.h_direction {
                HDirection::Right => self.y + 1,
                HDirection::Left if self.y != 0 => self.y - 1,
                _ => self.y,
            },
        ) as (u32, u32)
    }
}
