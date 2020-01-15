use crate::agent::Agent;
use crate::agent::HDirection;
use crate::agent::VDirection;
use crate::environment::Cell;
use crate::environment::Environment;

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

    fn clear_all(&mut self) {
        for agent in &mut self.agents {
            agent.clear(&mut self.env)
        }
    }

    pub fn tick(&mut self) {
        self.clear_all();
        for agent in &mut self.agents {
            agent.update(&mut self.env)
        }
        self.draw_all();
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

    pub fn new() -> Sma {
        let env = Environment::new(5, 5);


        let agents = vec![
        Agent {
            x: 0,
            y: 0,
            v_direction: VDirection::None,
            h_direction: HDirection::Right,
        },
        Agent {
            x: 4,
            y: 4,
            v_direction: VDirection::Up,
            h_direction: HDirection::None,
        },
        Agent {
            x: 0,
            y: 4,
            v_direction: VDirection::Up,
            h_direction: HDirection::Right,
        }
        ];

        Sma { env, agents }
    }
}