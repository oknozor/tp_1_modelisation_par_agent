use crate::agent::Agent;
use crate::environment::Cell;
use crate::environment::Environment;
use rand::{seq::SliceRandom, thread_rng};
pub struct Sma {
    pub env: Environment,
    pub agents: Vec<Agent>,
}

impl Sma {
    pub fn draw_all(&mut self) {
        for agent in &mut self.agents {
            agent.draw(&mut self.env)
        }
    }

    pub fn tick(&mut self) {
        // Randomize agents order each turn
        let mut agents = &mut self.agents;
        let slice: &mut [Agent] = &mut agents;
        slice.shuffle(&mut thread_rng());

        self.agents = slice.into();

        // Update all agent positions sequentialy
        for agent in &mut self.agents {
            agent.update(&mut self.env)
        }
    }

    pub fn new(height: u32, width: u32) -> Sma {
        let env = Environment::new(height, width);
        Sma {
            env,
            agents: vec![],
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        let already_filled = self
            .agents
            .iter()
            .find(|agent_in_memory| (agent_in_memory.x, agent_in_memory.y) == (agent.x, agent.y));

        if let None = already_filled {
            self.agents.push(agent);
        }
    }

    pub fn get_state(&self) -> &Vec<Cell> {
        &self.env.cells
    }

    pub fn height(&self) -> u32 {
        self.env.height
    }

    pub fn width(&self) -> u32 {
        self.env.width
    }

    pub fn get_index(&self, x: u32, y: u32) -> usize {
        self.env.get_index(x, y)
    }
}
