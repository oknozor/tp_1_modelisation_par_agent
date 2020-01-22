pub mod core;
pub mod environment;
pub mod particules;
pub mod sma;
pub mod wator;

use crate::core::AgentBehavior;
use environment::Environment;
use particules::agent::Agent;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct AgentRef {
    inner: Arc<Mutex<Box<dyn AgentBehavior>>>,
}

impl AgentRef {
    pub fn collision(&self) -> bool {
        self.inner.lock().unwrap().collision()
    }

    pub fn direction(&self) -> Direction {
        self.inner.lock().unwrap().direction()
    }

    pub fn coordinate(&self) -> Point {
        self.inner.lock().unwrap().coordinate()
    }

    pub fn previous_coordinate(&self) -> Point {
        self.inner.lock().unwrap().previous_coordinate()
    }

    pub fn set_collision(&self, collision: bool) {
        self.inner.lock().unwrap().set_collision(collision)
    }

    pub fn set_direction(&self, direction: Direction) {
        self.inner.lock().unwrap().set_direction(direction)
    }

    pub fn update(&mut self, env: &mut Environment) {
        self.inner.lock().unwrap().update(env)
    }

    pub fn decide(&mut self, env: &mut Environment) {
        self.inner.lock().unwrap().decide(env)
    }

    pub fn from(agent: Agent) -> Self {
        AgentRef {
            inner: Arc::new(Mutex::new(Box::new(agent))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Direction {
    pub x: HDirection,
    pub y: VDirection,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn new(x: HDirection, y: VDirection) -> Direction {
        Direction { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDirection {
    None,
    Right,
    Left,
}

impl HDirection {
    fn invert(&self) -> HDirection {
        match self {
            HDirection::None => HDirection::None,
            HDirection::Right => HDirection::Left,
            HDirection::Left => HDirection::Right,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDirection {
    None,
    Down,
    Up,
}

impl VDirection {
    fn invert(&self) -> VDirection {
        match self {
            VDirection::None => VDirection::None,
            VDirection::Down => VDirection::Up,
            VDirection::Up => VDirection::Down,
        }
    }
}
