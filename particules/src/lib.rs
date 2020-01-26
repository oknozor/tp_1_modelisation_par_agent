#![feature(vec_remove_item)]
pub mod core;
pub mod environment;
pub mod particules;
pub mod sma;
pub mod wator;

use crate::core::AgentBehavior;
use crate::sma::Sma;
use crate::wator::fish::Fish;
use crate::wator::shark::Shark;
use environment::Environment;
use lazy_static::lazy_static;
use particules::agent::Agent;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

lazy_static! {
    pub static ref SMA: Arc<RwLock<Sma>> = {
        Arc::new(RwLock::new(Sma {
            env: Environment::new(0, 0, false),
            agents: vec![],
            next_generation: vec![],
            turn: 0,
        }))
    };
}

#[derive(Clone)]
pub enum AgentKind {
    Fish,
    Other
}
#[derive(Clone)]
pub struct AgentRef {
    inner: Arc<Mutex<Box<dyn AgentBehavior + Sync + Send>>>,
    marked_for_removal: Arc<Mutex<bool>>,
    kind: AgentKind
}

pub enum AgentCommand {
    Create(AgentRef),
    DoNothing,
}

impl AgentRef {
    pub fn collision(&self) -> bool {
        self.inner.lock().unwrap().collision()
    }

    pub fn direction(&self) -> Direction {
        self.inner.lock().unwrap().direction()
    }

    pub fn set_coordinate(&self, point: Point) {
        self.inner.lock().unwrap().set_coordinate(point)
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

    pub fn update(&mut self, env: &mut Environment) -> AgentCommand {
        self.inner.lock().unwrap().update(env)
    }

    pub fn decide(&mut self, env: &Environment) {
        self.inner.lock().unwrap().decide(env)
    }

    pub fn get_color(&self) -> (f32, f32, f32) {
        self.inner.lock().unwrap().get_color()
    }

    pub fn mark_for_removal(&mut self) {
        self.marked_for_removal = Arc::new(Mutex::new(true));
    }

    pub fn from_fish(agent: Fish) -> Self {
        AgentRef {
            inner: Arc::new(Mutex::new(Box::new(agent))),
            marked_for_removal: Arc::new(Mutex::new(false)),
            kind: AgentKind::Fish
        }
    }

    pub fn from_shark(agent: Shark) -> Self {
        AgentRef {
            inner: Arc::new(Mutex::new(Box::new(agent))),
            marked_for_removal:  Arc::new(Mutex::new(false)),
            kind: AgentKind::Other
        }
    }

    pub fn from(agent: Agent) -> Self {
        AgentRef {
            inner: Arc::new(Mutex::new(Box::new(agent))),
            marked_for_removal:  Arc::new(Mutex::new(false)),
            kind: AgentKind::Other
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

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
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
