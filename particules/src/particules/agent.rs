use crate::core::AgentBehavior;
use crate::AgentRef;
use crate::Direction;
use crate::Point;
use crate::environment::Cell;
use crate::environment::Environment;

pub struct Agent {
    pub direction: Direction,
    pub coordinate: Point,
    pub previous_coordinate: Point,
    pub collision: bool,
    pub(crate) decision: Decision,
}

pub(crate) enum Decision {
    KeepCourse,
    ChangeCourseCollision(AgentRef),
    ChangeCourseOutOfBound(Direction),
}

impl AgentBehavior for Agent {
    fn decide(&mut self, environment: &mut Environment) {
        let forward_position = if environment.borderless {
            self.look_ahead_borderless(environment.width, environment.height)
        } else {
            self.look_ahead()
        };

        let out_of_bound_x = environment.is_out_of_bound_x(forward_position.x);
        let out_of_bound_y = environment.is_out_of_bound_y(forward_position.y);

        self.decision = if !out_of_bound_x && !out_of_bound_y {
            let forward_idx = environment.get_index(forward_position);
            let cell_forward = &environment.cells[forward_idx];

            match cell_forward {
                Cell::Empty => Decision::KeepCourse,
                Cell::Filled(agent) => Decision::ChangeCourseCollision(agent.clone()),
            }
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

    fn update(&mut self, environment: &mut Environment) {
        match &self.decision {
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

    fn collision(&self) -> bool {
        self.collision
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn coordinate(&self) -> Point {
        self.coordinate
    }

    fn set_coordinate(&mut self, point: Point) {
        self.coordinate = point
    }

    fn previous_coordinate(&self) -> Point {
        self.previous_coordinate
    }

    fn set_previous_coordinate(&mut self, point: Point) {
        self.previous_coordinate = point
    }

    fn set_collision(&mut self, collision: bool) {
        self.collision = collision
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}
