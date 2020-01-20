use super::Point;
use crate::agent::Agent;
use crate::environment::Cell;
use crate::environment::Environment;
use rand::{seq::SliceRandom, thread_rng};
use std::cell::RefCell;
use std::rc::Rc;

type AgentRef = Rc<RefCell<Agent>>;
pub struct Sma {
    pub env: Environment,
    pub agents: Vec<AgentRef>,
}

impl Sma {
    /// Update environment cells according to agent positions
    pub fn draw_all(&mut self) {
        for agent in &mut self.agents {
            agent.borrow_mut().update_env(&mut self.env)
        }
    }

    pub fn tick(&mut self) {
        self.shuffle_agents();
        // Update all agent positions sequentialy
        for agent in &mut self.agents {
            agent.borrow_mut().update(&mut self.env)
        }
    }

    pub fn shuffle_agents(&mut self) {
        // Randomize agents order each turn
        let mut agents = &mut self.agents;
        let slice: &mut [AgentRef] = &mut agents;
        slice.shuffle(&mut thread_rng());

        self.agents = slice.into();
    }

    pub fn new(height: i32, width: i32) -> Sma {
        let env = Environment::new(height, width, false);
        Sma {
            env,
            agents: vec![],
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        let already_filled = self
            .agents
            .iter()
            .find(|agent_in_memory| (agent_in_memory.borrow().coordinate) == agent.coordinate);

        if let None = already_filled {
            self.agents.push(Rc::new(RefCell::new(agent)));
        }
    }


    pub fn get_state(&self) -> &Vec<Cell> {
        &self.env.cells
    }

    pub fn height(&self) -> i32 {
        self.env.height
    }

    pub fn width(&self) -> i32 {
        self.env.width
    }

    pub fn get_index(&self, point: Point) -> usize {
        self.env.get_index(point)
    }

    pub fn set_borderless(&mut self, value: bool) {
        self.env.borderless = value;
    }
}
