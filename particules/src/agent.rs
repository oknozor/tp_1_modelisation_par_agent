use super::Direction;
use super::HDirection;
use super::Point;
use super::VDirection;
use crate::environment::Cell;
use crate::environment::Environment;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    pub direction: Direction,
    pub coordinate: Point,
    pub collision: bool,
}

enum Decision {
    KeepCourse,
    ChangeCourseCollision(Rc<RefCell<Agent>>),
    ChangeCourseOutOfBound(Direction),
}

impl Agent {
    pub fn update_env(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.coordinate);
        environment.cells[idx] = Cell::Filled(Rc::new(RefCell::new(self.clone())));
    }

    pub fn clear(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.coordinate);
        environment.cells[idx] = Cell::Empty;
    }

    pub fn update(&mut self, environment: &mut Environment) {
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

        let forward_position = self.look_ahead();
        self.coordinate= forward_position;
        self.collision(environment);
        self.update_env(environment);
    }

    fn collision(&mut self, environment: &Environment) {
        let on_edge_right = self.coordinate.x == environment.width - 1;
        let on_edge_left = self.coordinate.x == 0;
        let on_edge_top = self.coordinate.y == environment.height - 1;
        let on_edge_bottom = self.coordinate.y == 0;

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
            let forward_position= self.look_ahead();
            let idx = environment.get_index(forward_position);

            if let Some(Cell::Filled(_)) = environment.cells.get(idx) {
                self.collision = true;
            } else {
                self.collision = false;
            }
        } else {
            self.collision = true;
        }
    }

    fn decide(&mut self, environment: &mut Environment) -> Decision {
        let forward_position = self.look_ahead();

        let out_of_bound_x = environment.is_out_of_bound_x(forward_position.x);
        let out_of_bound_y = environment.is_out_of_bound_y(forward_position.y);

        if !out_of_bound_x && !out_of_bound_y {
            let forward_idx = environment.get_index(forward_position);
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

    fn look_ahead(&self) -> Point {
        Point {
            x: match self.direction.x {
                HDirection::Right => self.coordinate.x + 1,
                HDirection::Left => self.coordinate.x - 1,
                _ => self.coordinate.x,
            },
            y: match self.direction.y {
                VDirection::Up => self.coordinate.y - 1,
                VDirection::Down => self.coordinate.y + 1,
                _ => self.coordinate.y,
            },
        }
    }

    pub fn new(x: i32, y: i32, direction: Direction) -> Agent {
        Agent {
            direction,
            coordinate: Point { x, y },
            collision: false,
        }
    }
}
