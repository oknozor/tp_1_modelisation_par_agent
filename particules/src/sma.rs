use super::AgentRef;
use super::Direction;
use super::Point;
use crate::agent::Agent;
use crate::environment::Cell;
use crate::environment::Environment;
use rand::{seq::SliceRandom, thread_rng};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sma {
    pub env: Environment,
    pub agents: Vec<AgentRef>,
}

impl Sma {
    pub fn tick(&mut self) {
        self.shuffle_agents();
        // Update all agent positions sequentialy
        for agent in &mut self.agents {
            agent.update(&mut self.env)
        }
    }

    pub fn new(height: i32, width: i32) -> Sma {
        let env = Environment::new(height, width, false);
        Sma {
            env,
            agents: vec![],
        }
    }

    fn shuffle_agents(&mut self) {
        // Randomize agents order each turn
        let mut agents = &mut self.agents;
        let slice: &mut [AgentRef] = &mut agents;
        slice.shuffle(&mut thread_rng());

        self.agents = slice.into();
    }

    pub fn add_agent(&mut self, coordinate: Point, direction: Direction) {
        let already_filled = self
            .agents
            .iter()
            .find(|agent_in_memory| (agent_in_memory.coordinate()) == coordinate);

        if let None = already_filled {
            let agent = Agent {
                collision: false,
                coordinate,
                previous_coordinate: coordinate,
                direction,
            };

            let agent_ref = AgentRef {
                inner: Rc::new(RefCell::new(agent)),
            };
            self.agents.push(agent_ref.clone());
            self.env
                .set_cell(coordinate, Cell::Filled(agent_ref))
                .unwrap();
        }
    }

    pub fn get_state(&self) -> &Vec<Cell> {
        &self.env.cells
    }

    pub fn get_index(&self, point: Point) -> usize {
        self.env.get_index(point)
    }

    pub fn set_borderless(&mut self, value: bool) {
        self.env.borderless = value;
    }
}
