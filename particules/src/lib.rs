pub mod agent;
pub mod environment;
pub mod sma;

use agent::Agent;
use environment::Environment;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AgentRef {
    inner: Rc<RefCell<Agent>>,
}

impl AgentRef {
    pub fn collision(&self) -> bool {
        self.inner.borrow().collision
    }

    pub fn direction(&self) -> Direction {
        self.inner.borrow().direction
    }

    pub fn coordinate(&self) -> Point {
        self.inner.borrow().coordinate
    }

    pub fn previous_coordinate(&self) -> Point {
        self.inner.borrow().previous_coordinate
    }

    pub fn set_collision(&self, collision: bool) {
        self.inner.borrow_mut().collision = collision
    }

    pub fn set_direction(&self, direction: Direction) {
        self.inner.borrow_mut().direction = direction
    }

    pub fn update(&mut self, env: &mut Environment) {
        self.inner.borrow_mut().update(env)
    }

    pub fn decide(&mut self, env: &mut Environment) {
        self.inner.borrow_mut().decide(env)
    }

    pub fn from(agent: Agent) -> Self {
        AgentRef {
            inner: Rc::new(RefCell::new(agent)),
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