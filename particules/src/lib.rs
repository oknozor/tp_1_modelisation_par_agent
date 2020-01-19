pub mod agent;
pub mod environment;
pub mod sma;

use agent::Agent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Direction {
    pub x: HDirection,
    pub y: VDirection,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
