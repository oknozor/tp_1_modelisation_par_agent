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
    pub fn tick(&mut self) -> String {
        for agent in &mut self.agents {
            agent.update(&mut self.env)
        }
        self.env.render()
    }

    pub fn new() -> Sma {
        let env = Environment {
            cells: vec![
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            height: 4,
            width: 4,
        };

        let agents = vec![Agent {
            x: 0,
            y: 0,
            v_direction: VDirection::Up,
            h_direction: HDirection::Right,
        }];

        Sma { env, agents }
    }
}
