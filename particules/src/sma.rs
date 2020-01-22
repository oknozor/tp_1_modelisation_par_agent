use super::particules::agent::Decision;
use super::AgentRef;
use super::Direction;
use super::HDirection;
use super::Point;
use super::VDirection;
use crate::environment::Cell;
use crate::environment::Environment;
use crate::particules::agent::Agent;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;
use rayon::prelude::*;

pub struct Sma {
    pub env: Environment,
    pub agents: Vec<AgentRef>,
}

impl Sma {
    pub fn tick(&mut self) {
        // Update all agent positions sequentialy
        let env = &mut self.env;
        for agent in &mut self.agents {
            agent.decide(env);
            agent.update(env);
        }
        self.shuffle_agents();
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

    pub fn add_agent(&mut self, coordinate: Point, direction: Direction) -> Result<(), &str> {
        let filled = self
            .agents
            .iter()
            .find(|agent_in_memory| (agent_in_memory.coordinate()) == coordinate);

        if let None = filled {
            let agent = Agent {
                collision: false,
                coordinate,
                previous_coordinate: coordinate,
                direction,
                decision: Decision::KeepCourse,
            };

            let agent_ref = AgentRef {
                inner: Arc::new(Mutex::new(Box::new(agent))),
            };

            self.agents.push(agent_ref.clone());
            self.env
                .set_cell(coordinate, Cell::Filled(agent_ref))
                .unwrap();
            Ok(())
        } else {
            Err("Agent already stored at this location!")
        }
    }

    fn add_agent_unsafe(&mut self, coordinate: Point, direction: Direction) {
        let agent = Agent {
            collision: false,
            coordinate,
            previous_coordinate: coordinate,
            direction,
            decision: Decision::KeepCourse,
        };

        let agent_ref = AgentRef {
            inner: Arc::new(Mutex::new(Box::new(agent))),
        };

        self.agents.push(agent_ref.clone());
        self.env
            .set_cell(coordinate, Cell::Filled(agent_ref))
            .unwrap();
    }

    pub fn gen_agents(&mut self, density: u8) {
        if density > 100 {
            panic!("Density must be inferior or equal to 100");
        }
        let now = SystemTime::now();
        let size = self.env.height * self.env.width;

        let agent_count = (size as f32 / 100 as f32) * density as f32;
        let agent_count = agent_count as i32;

        let mut vec: Vec<i32> = (0..size).collect();
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);

        println!(
            " about to generate {} agents in a env of size {}",
            agent_count, size
        );

        (0..(agent_count as usize)).for_each(|i| {
            let direction = Sma::pick_direction();
            let idx = vec[i];

            let x = idx % self.env.width;
            let y = (idx - x) / self.env.width;
            let point = Point { x, y };

            self.add_agent_unsafe(point, direction);
        })
    }

    fn pick_direction() -> Direction {
        let mut rng = thread_rng();

        loop {
            let x_dir = match rng.gen_range(0, 3) {
                0 => HDirection::Right,
                1 => HDirection::Left,
                _ => HDirection::None,
            };

            let y_dir = match rng.gen_range(0, 3) {
                0 => VDirection::Up,
                1 => VDirection::Down,
                _ => VDirection::None,
            };

            let direction = Direction { x: x_dir, y: y_dir };

            if (direction.x, direction.y) != (HDirection::None, VDirection::None) {
                return direction;
            } else {
                continue;
            }
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
