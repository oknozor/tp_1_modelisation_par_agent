use super::AgentRef;
use super::Direction;
use super::HDirection;
use super::Point;
use super::VDirection;
use crate::environment::Cell;
use crate::environment::Environment;

#[derive(Debug)]
pub struct Agent {
    pub direction: Direction,
    pub coordinate: Point,
    pub previous_coordinate: Point,
    pub collision: bool,
}

enum Decision {
    KeepCourse,
    ChangeCourseCollision(AgentRef),
    ChangeCourseOutOfBound(Direction),
}

impl Agent {
    pub fn update(&mut self, environment: &mut Environment) {
        match self.decide(environment) {
            Decision::ChangeCourseOutOfBound(direction) => {
                self.direction.y = direction.y;
                self.direction.x = direction.x;
                self.collision = true;
            }
            Decision::ChangeCourseCollision(agent) if !environment.borderless => {
                let direction = self.direction;
                self.direction = agent.direction();
                agent.set_direction(direction);
                agent.set_collision(true);
                self.collision = true;
            }
            _ => {
                self.collision = false;
                self.move_forward(environment);
            }
        };
    }

    pub fn move_forward(&mut self, environment: &mut Environment) {
        self.previous_coordinate = self.coordinate;

        let forward_position = if environment.borderless {
            self.look_ahead_borderless(environment.width, environment.height)
        } else {
            self.look_ahead()
        };

        match environment.get_cell(forward_position) {
            Some(Cell::Empty) => {
                let agent = environment.get_cell(self.coordinate).unwrap().clone();
                self.coordinate = forward_position;

                environment
                    .set_cell(self.previous_coordinate, Cell::Empty)
                    .unwrap();
                environment.set_cell(self.coordinate, agent).unwrap();
            }
            _ => (),
        }
    }

    fn decide(&mut self, environment: &mut Environment) -> Decision {
        let forward_position = if environment.borderless {
            self.look_ahead_borderless(environment.width, environment.height)
        } else {
            self.look_ahead()
        };

        let out_of_bound_x = environment.is_out_of_bound_x(forward_position.x);
        let out_of_bound_y = environment.is_out_of_bound_y(forward_position.y);

        if !out_of_bound_x && !out_of_bound_y {
            let forward_idx = environment.get_index(forward_position);
            let cell_forward = &environment.cells[forward_idx];

            return match cell_forward {
                Cell::Empty => Decision::KeepCourse,
                Cell::Filled(agent) => Decision::ChangeCourseCollision(agent.clone()),
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

    fn look_ahead_borderless(&self, width: i32, height: i32) -> Point {
        Point {
            x: match self.direction.x {
                HDirection::Right => {
                    if self.coordinate.x + 1 > width - 1 {
                        0
                    } else {
                        self.coordinate.x + 1
                    }
                }
                HDirection::Left => {
                    if self.coordinate.x - 1 < 0 {
                        width - 1
                    } else {
                        self.coordinate.x - 1
                    }
                }
                _ => self.coordinate.x,
            },
            y: match self.direction.y {
                VDirection::Up => {
                    if self.coordinate.y - 1 < 0 {
                        height - 1
                    } else {
                        self.coordinate.y - 1
                    }
                }
                VDirection::Down => {
                    if self.coordinate.y + 1 > height - 1 {
                        0
                    } else {
                        self.coordinate.y + 1
                    }
                }
                _ => self.coordinate.y,
            },
        }
    }

    pub fn new(x: i32, y: i32, direction: Direction) -> Agent {
        Agent {
            direction,
            coordinate: Point { x, y },
            previous_coordinate: Point { x, y },
            collision: false,
        }
    }
}
