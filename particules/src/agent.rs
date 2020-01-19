use super::Direction;
use super::HDirection;
use super::VDirection;
use crate::environment::Cell;
use crate::environment::Environment;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    pub direction: Direction,
    pub x: u32,
    pub y: u32,
    pub collision: bool,
}

enum Decision {
    KeepCourse,
    ChangeCourseCollision(Rc<RefCell<Agent>>),
    ChangeCourseOutOfBound(Direction),
}

impl Agent {
    pub fn update_env(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Filled(Rc::new(RefCell::new(self.clone())));
    }

    pub fn clear(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Empty;
    }

    pub fn update(&mut self, environment: &mut Environment) {
        self.collision = false;
        self.clear(environment);

        match self.decide(environment) {
            Decision::ChangeCourseOutOfBound(direction) => {
                self.direction.y = direction.y;
                self.direction.x = direction.x
            }
            Decision::ChangeCourseCollision(agent) => {
                let direction = self.direction;
                self.direction = agent.borrow().direction;
                agent.borrow_mut().direction = direction;
                agent.borrow_mut().collision = true;
            }
            Decision::KeepCourse => (),
        };

        let (x_forward, y_forward) = self.look_ahead();
        self.x = x_forward;
        self.y = y_forward;
        self.collision(environment);
        self.update_env(environment);
    }

    fn collision(&mut self, environment: &Environment) {
        let on_edge_right = self.x == environment.width - 1;
        let on_edge_left = self.x == 0;
        let on_edge_top = self.y == environment.height - 1;
        let on_edge_bottom = self.y == 0;

        let wall_collision = match (self.direction.x, self.direction.y) {
            (HDirection::Right, VDirection::None) => on_edge_right,
            (HDirection::Right, VDirection::Up) => on_edge_right || on_edge_bottom,
            (HDirection::Right, VDirection::Down) => on_edge_right || on_edge_top,
            (HDirection::Left, VDirection::None) => on_edge_left,
            (HDirection::Left, VDirection::Up) => on_edge_left || on_edge_bottom,
            (HDirection::Left, VDirection::Down) => on_edge_left || on_edge_top,
            (HDirection::None, VDirection::Up) => on_edge_bottom,
            (HDirection::None, VDirection::Down) => on_edge_top,
            (_, _) => false,
        };

        if !wall_collision {
            let (x_forward, y_forward)= self.look_ahead();
            let idx = environment.get_index(x_forward, y_forward);

            if let Some(Cell::Filled(_)) = environment.cells.get(idx) {
                self.collision = true;
            }
        } else {
            self.collision = true;
        }
    }

    fn decide(&mut self, environment: &mut Environment) -> Decision {
        let (x_forward, y_forward) = self.look_ahead();

        let out_of_bound_x = self.x == 0 || environment.is_out_of_bound_x(x_forward);
        let out_of_bound_y = self.y == 0 || environment.is_out_of_bound_y(y_forward);

        if !out_of_bound_x && !out_of_bound_y {
            let forward_idx = environment.get_index(x_forward, y_forward);
            let cell_forward = environment.cells[forward_idx].clone();

            return match cell_forward {
                Cell::Empty => Decision::KeepCourse,
                Cell::Filled(agent) => Decision::ChangeCourseCollision(Rc::clone(&agent)),
            };
        } else if out_of_bound_x && !out_of_bound_y {
            Decision::ChangeCourseOutOfBound(Direction::new(
                self.direction.x.invert(),
                self.direction.y,
            ))
        } else if !out_of_bound_x && out_of_bound_y {
            Decision::ChangeCourseOutOfBound(Direction::new(
                self.direction.x,
                self.direction.y.invert(),
            ))
        } else if out_of_bound_x && out_of_bound_y {
            Decision::ChangeCourseOutOfBound(Direction::new(
                self.direction.x.invert(),
                self.direction.y.invert(),
            ))
        } else {
            Decision::KeepCourse
        }
    }

    fn look_ahead(&self) -> (u32, u32) {
        (
            match self.direction.x {
                HDirection::Right => self.x + 1,
                HDirection::Left if self.x != 0 => self.x - 1,
                _ => self.x,
            },
            match self.direction.y {
                VDirection::Up if self.y != 0 => self.y - 1,
                VDirection::Down => self.y + 1,
                _ => self.y,
            },
        ) as (u32, u32)
    }

    pub fn new(x: u32, y: u32, direction: Direction) -> Agent {
        Agent {
            direction,
            x,
            y,
            collision: false,
        }
    }
}
