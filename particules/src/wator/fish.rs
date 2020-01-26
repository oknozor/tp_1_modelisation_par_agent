use rand::{Rng, thread_rng};

use crate::{AgentCommand, AgentRef, Direction};
use crate::core::AgentBehavior;
use crate::environment::{Cell, Environment};
use crate::Point;

pub struct Fish {
    pub coordinate: Point,
    pub decision: Decision,
    pub breed_count_down: u8,
}

pub enum Decision {
    Stall,
    Move(Point),
    MoveAndBreed(Point),
}

impl AgentBehavior for Fish {
    fn decide(&mut self, environment: &Environment) {
        let empty_neighbors = vec![
            self.peek(environment, 0, 1),
            self.peek(environment, 0, -1),
            self.peek(environment, 1, 0),
            self.peek(environment, 1, -1),
            self.peek(environment, 1, 1),
            self.peek(environment, -1, 1),
            self.peek(environment, -1, 0),
            self.peek(environment, -1, -1),
        ]
        .iter()
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .collect::<Vec<Point>>();

        if empty_neighbors.is_empty() {
            self.decision = Decision::Stall;
        } else {
            let idx = thread_rng().gen_range(0, empty_neighbors.len());
            if self.breed_count_down == 0 {
                self.decision = Decision::MoveAndBreed(empty_neighbors[idx]);
                self.breed_count_down = environment.fish_breed_time;
            } else {
                self.decision = Decision::Move(empty_neighbors[idx]);
            }
            self.breed_count_down -= 1;
        }
    }

    fn update(&mut self, environment: &mut Environment) -> AgentCommand {
        match &self.decision {
            Decision::Stall => AgentCommand::DoNothing,
            Decision::MoveAndBreed(position) => {
                let child = Fish {
                    coordinate: self.coordinate,
                    decision: Decision::Stall,
                    breed_count_down: environment.fish_breed_time,
                };
                let child_ref = AgentRef::from_fish(child);
                environment.swap(self.coordinate, *position);
                environment.set_cell(self.coordinate, Cell::Filled(child_ref.clone())).unwrap();
                self.coordinate = *position;
                AgentCommand::Create(child_ref)
            }
            Decision::Move(position) => {
                environment.swap(self.coordinate, *position);
                self.coordinate = *position;
                AgentCommand::DoNothing
            }
        }
    }

    fn collision(&self) -> bool {
        unimplemented!("collision")
    }

    fn direction(&self) -> Direction {
        unimplemented!("direction")
    }

    fn coordinate(&self) -> Point {
        self.coordinate
    }

    fn set_coordinate(&mut self, point: Point) {
        self.coordinate = point;
    }

    fn previous_coordinate(&self) -> Point {
        unimplemented!("previous")
    }

    fn set_previous_coordinate(&mut self, _: Point) {
        unimplemented!("set previous ")
    }

    fn set_collision(&mut self, _: bool) {
        unimplemented!("set collision")
    }

    fn set_direction(&mut self, _: Direction) {
        unimplemented!("set direction")
    }

    fn get_color(&self) -> (f32, f32, f32) {
        (0.0, 1.0, 0.0)
    }

}
