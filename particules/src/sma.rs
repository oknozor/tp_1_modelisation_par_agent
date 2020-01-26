use std::sync::Arc;
use std::sync::Mutex;

use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::environment::Cell;
use crate::environment::Environment;
use crate::particules::agent::Agent;
use crate::wator::fish::Fish;
use crate::wator::shark::Shark;
use crate::AgentCommand;
use crate::AgentKind;
use crate::SMA;

use super::particules::agent::Decision;
use super::wator::fish::Decision as FishDecision;
use super::wator::shark::Decision as SharkDecision;
use super::AgentRef;
use super::Direction;
use super::HDirection;
use super::Point;
use super::VDirection;

pub struct Sma {
    pub env: Environment,
    pub agents: Vec<AgentRef>,
    pub(crate) next_generation: Vec<AgentRef>,
    pub turn: i32,
}

impl Sma {
    pub fn tick(&mut self) {
        // Update all agent positions sequentialy
        let env = &mut self.env;
        println!("agent count {}", &self.agents.len());

        for idx in 0..self.agents.len() {
            &mut self.agents[idx].decide(env);
            match self.agents[idx].update(env) {
                AgentCommand::DoNothing => (),
                AgentCommand::Create(agent) => self.next_generation.push(agent),
            };
        }

        &mut self
            .agents
            .iter()
            .filter(|agent| *agent.marked_for_removal.lock().unwrap())
            .for_each(|agent| env.set_cell(agent.coordinate(), Cell::Empty).unwrap());

        println!("env {}", self.get_state().len());
        println!("size {}", self.agents.len());
        &mut self
            .agents
            .iter()
            .for_each(|agent| println!("remove ? {}", *agent.marked_for_removal.lock().unwrap()));
        &mut self
            .agents
            .retain(|agent| !*agent.marked_for_removal.lock().unwrap());
        println!("size after removal{}", self.agents.len());

        while let Some(agent_ref) = self.next_generation.pop() {
            self.agents.push(agent_ref);
        }

        self.env.debug();

        self.shuffle_agents();
        println!("turn {}", self.turn);
        self.turn += 1;
    }

    pub fn new(height: i32, width: i32) {
        let env = Environment::new(height, width, false);
        SMA.write().unwrap().env = env;
    }

    pub fn new_with_fish(
        height: i32,
        width: i32,
        breed_rime_fish: u8,
        breed_time_shark: u8,
        shark_starve_time: u8,
    ) {
        let env = Environment::new_fish_shark(
            height,
            width,
            false,
            breed_rime_fish,
            breed_time_shark,
            shark_starve_time,
        );
        SMA.write().unwrap().env = env;
    }

    fn shuffle_agents(&mut self) {
        // Randomize agents order each turn
        let mut agents = &mut self.agents;
        let slice: &mut [AgentRef] = &mut agents;
        slice.shuffle(&mut thread_rng());

        self.agents = slice.into();
    }

    pub fn add_agent(&mut self, agent: AgentRef) {
        self.next_generation.push(agent);
    }

    pub fn gen_agent(&mut self, coordinate: Point, direction: Direction) -> Result<(), &str> {
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
                marked_for_removal: Arc::new(Mutex::new(false)),
                kind: AgentKind::Other,
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
            marked_for_removal: Arc::new(Mutex::new(false)),
            kind: AgentKind::Other,
        };

        self.agents.push(agent_ref.clone());
        self.env
            .set_cell(coordinate, Cell::Filled(agent_ref))
            .unwrap();
    }

    fn add_fish_unsafe(&mut self, coordinate: Point) {
        let fish = Fish {
            coordinate,
            decision: FishDecision::Stall,
            breed_count_down: self.env.shark_breed_time,
        };

        let fish_ref = AgentRef {
            inner: Arc::new(Mutex::new(Box::new(fish))),
            marked_for_removal: Arc::new(Mutex::new(false)),
            kind: AgentKind::Fish,
        };

        self.agents.push(fish_ref.clone());
        self.env
            .set_cell(coordinate, Cell::Filled(fish_ref))
            .unwrap();
    }

    fn add_shark_unsafe(&mut self, coordinate: Point) {
        let shark = Shark {
            coordinate,
            decision: SharkDecision::Stall,
            breed_count_down: self.env.shark_breed_time,
            starve_time: self.env.shark_starve_time,
        };

        let shark_ref = AgentRef {
            inner: Arc::new(Mutex::new(Box::new(shark))),
            marked_for_removal: Arc::new(Mutex::new(false)),
            kind: AgentKind::Other,
        };

        self.agents.push(shark_ref.clone());
        self.env
            .set_cell(coordinate, Cell::Filled(shark_ref))
            .unwrap();
    }

    pub fn gen_agents(&mut self, density: u8) {
        if density > 100 {
            panic!("Density must be inferior or equal to 100");
        }
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

    pub fn gen_fish_agents(&mut self, fish_density: u8, shark_density: u8) {
        let size = self.env.height * self.env.width;

        let mut vec: Vec<i32> = (0..size).collect();
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);

        println!(
            " about to generate {} fish and {} sharks in a env of size {}",
            fish_density, shark_density, size
        );
        (0..(fish_density as usize)).for_each(|_| {
            let idx = vec.pop().unwrap();

            let x = idx % self.env.width;
            let y = (idx - x) / self.env.width;
            let point = Point { x, y };

            self.add_fish_unsafe(point);
        });

        (0..(shark_density as usize)).for_each(|_| {
            let idx = vec.pop().unwrap();

            let x = idx % self.env.width;
            let y = (idx - x) / self.env.width;
            let point = Point { x, y };

            self.add_shark_unsafe(point);
        });
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
