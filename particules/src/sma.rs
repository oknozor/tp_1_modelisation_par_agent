use super::AgentRef;
use super::Direction;
use super::HDirection;
use super::Point;
use super::VDirection;
use crate::agent::Agent;
use crate::environment::Cell;
use crate::environment::Environment;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sma {
    pub env: Environment,
    pub agents: Vec<AgentRef>,
}

impl Sma {
    pub fn tick(&mut self) {
        self.shuffle_agents();
        println!("{}", self.agents.len());
        
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
    
    pub fn add_agent(&mut self, coordinate: Point, direction: Direction) -> Result<(), ()> {
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
            };
            
            let agent_ref = AgentRef {
                inner: Rc::new(RefCell::new(agent)),
            };
            
            self.agents.push(agent_ref.clone());
            self.env
            .set_cell(coordinate, Cell::Filled(agent_ref))
            .unwrap();
            Ok(())
        } else {
            Err(())
        }
    }
    
    pub fn gen_agents(&mut self, density: u8) {
        if density > 100 {
            panic!("Density must be inferior or equal to 100");
        }
        
        let size = self.env.height * self.env.width;
        let size = (size / 100) * density as i32;
        let mut rng = thread_rng();

        let mut count =  0;
        while count <  size {
            let pick = rng.gen_range(0, 100);
            if pick <= density {
                
                let x = rng.gen_range(0, self.env.height);
                let y = rng.gen_range(0, self.env.width);
                let direction = Sma::pick_direction();
                
                let point = Point { x, y };
                
                if let Ok(()) = self.add_agent(point, direction) {
                    count+=1;
                    println!("agents generated {}/{}", count, size);
                    
                }
            }
        }
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
            
            if (direction.x, direction.y) != (HDirection::None, VDirection::None)  {
                return direction
            } else {
                continue
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
