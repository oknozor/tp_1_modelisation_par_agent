use rand::{thread_rng, Rng};

use crate::core::AgentBehavior;
use crate::environment::{Cell, Environment};
use crate::Point;
use crate::{AgentCommand, AgentRef, Direction};

pub struct Shark {
    pub coordinate: Point,
    pub decision: Decision,
    pub breed_count_down: u8,
    pub starve_time: u8,
}

pub enum Decision {
    Stall,
    Move(Point),
    MoveAndBreed(Point),
    Eat(Point, AgentRef),
    EatAndBreed(Point, AgentRef),
}

impl AgentBehavior for Shark {
    fn decide(&mut self, environment: &Environment) {
        let neighbors = vec![
            self.peek_cell(environment, 0, 1),
            self.peek_cell(environment, 0, -1),
            self.peek_cell(environment, 1, 0),
            self.peek_cell(environment, -1, 0),
            self.peek_cell(environment, 1, 1),
            self.peek_cell(environment, 1, -1),
            self.peek_cell(environment, -1, 1),
            self.peek_cell(environment, -1, -1),
        ]
        .iter()
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .filter(|(cell, _)| cell.is_fish() || cell.is_empty_cell())
        .collect::<Vec<(&Cell, Point)>>();


        if neighbors.is_empty() {
            self.decision = Decision::Stall
        } else {
            let fish_neighbors: Vec<&(&Cell, Point)> = neighbors
                .iter()
                .filter(|(cell, _)| cell.is_fish())
                .collect();
            println!("fish neigh {}", fish_neighbors.len());
            
            let has_empty_neighbors = neighbors.len() - fish_neighbors.len() != 0;

            println!("empty neigh {}", fish_neighbors.len());

            if !fish_neighbors.is_empty() {
                let idx = thread_rng().gen_range(0, fish_neighbors.len());

                let cell = fish_neighbors[idx].0;

                let fish = match cell {
                    Cell::Filled(agent_ref) if cell.is_fish() => agent_ref,
                    _ => unreachable!("Expected a fish to eat"),
                };

                if self.breed_count_down == 0 {
                    self.decision = Decision::EatAndBreed(fish_neighbors[idx].1, fish.clone());
                    self.breed_count_down = environment.shark_breed_time;
                } else {
                    self.decision = Decision::Eat(fish_neighbors[idx].1, fish.clone());
                }
            } else if has_empty_neighbors {

                let idx = thread_rng().gen_range(0, neighbors.len());
                if self.breed_count_down == 0 {
                    self.decision = Decision::MoveAndBreed(neighbors[idx].1);
                    self.breed_count_down = environment.shark_breed_time;
                } else {
                    self.decision = Decision::Move(neighbors[idx].1);
                }
            }
        }

        self.starve_time -= 1;
        self.breed_count_down -= 1;
    }

    fn update(&mut self, environment: &mut Environment) -> AgentCommand {
        if self.starve_time == 0 {
            let self_ref = environment.get_mut_cell(self.coordinate);
            if let Some(Cell::Filled(self_ref)) = self_ref {
                self_ref.clone().mark_for_removal();
                self.decision = Decision::Stall;
                return AgentCommand::DoNothing;
            } else {
                unreachable!("Self removal Error");
            };
        };

         match &mut self.decision {
            Decision::Stall => AgentCommand::DoNothing,
            Decision::MoveAndBreed(position) => {
                println!("MOVE AND BREED");


                let child = Shark {
                    coordinate: self.coordinate,
                    decision: Decision::Stall,
                    breed_count_down: environment.shark_breed_time,
                    starve_time: environment.shark_starve_time,
                };

                let child_position = self.coordinate;
                let child_ref = AgentRef::from_shark(child);

                environment.set_agent_cell(self.coordinate, *position);

                environment
                    .set_cell(child_position, Cell::Filled(child_ref.clone()))
                    .unwrap();

                AgentCommand::Create(child_ref)
            }
            Decision::Move(position) => {
                environment.set_agent_cell(self.coordinate, *position);
                AgentCommand::DoNothing
            }
            Decision::Eat(position, fish) => {
                println!("EAT");

                environment.set_agent_cell(self.coordinate, *position);
                self.starve_time = environment.shark_starve_time;
                fish.mark_for_removal();

                AgentCommand::DoNothing
            }
            Decision::EatAndBreed(position, fish) => {
                println!("EAT AND BREED");

                environment.set_cell(*position, Cell::Empty).unwrap();
                let child = Shark {
                    coordinate: self.coordinate,
                    decision: Decision::Stall,
                    breed_count_down: environment.shark_breed_time,
                    starve_time: environment.shark_starve_time,
                };
                let child_ref = AgentRef::from_shark(child);
                let child_position = self.coordinate;

                environment.set_agent_cell(self.coordinate, *position);

                environment
                    .set_cell(child_position, Cell::Filled(child_ref.clone()))
                    .unwrap();
                self.coordinate = *position;
                self.starve_time = environment.shark_starve_time;
                fish.mark_for_removal();

                AgentCommand::Create(child_ref)
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
        (1.0, 0.0, 0.0)
    }
}
