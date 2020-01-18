use crate::environment::Cell;
use crate::environment::Environment;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    pub v_direction: VDirection,
    pub h_direction: HDirection,
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
}

impl Agent {
    pub fn draw(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Filled(Rc::new(RefCell::new(self.clone())));
    }

    pub fn clear(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Empty;
    }

    pub fn update(&mut self, environment: &mut Environment) {
        self.decide(environment);

        let (x_forward, y_forward) = self.look_ahead();

        self.y = y_forward;
        self.x = x_forward;
        self.draw(environment);
    }

    fn decide(&mut self, environment: &mut Environment) {
        self.clear(environment);
        self.color = Color::Black;

        let (x_forward, y_forward) = self.look_ahead();

        let out_of_bound_h = self.x == 0 || environment.is_out_of_bound_h(x_forward);
        let out_of_bound_v = self.y == 0 || environment.is_out_of_bound_v(y_forward);

        if !out_of_bound_h && !out_of_bound_v {
            let forward_idx = environment.get_index(x_forward, y_forward);
            let cell_forward = environment.cells[forward_idx].clone();

            match cell_forward {
                Cell::Empty => (),
                Cell::Filled(agent) => {
                    self.color = Color::Red;
                    agent.borrow_mut().color = Color::Red;

                    // Swap agents directions
                    let direction_h = self.v_direction;
                    let direction_v = self.h_direction;
                    self.h_direction = agent.borrow().h_direction;
                    self.v_direction = agent.borrow().v_direction;
                    agent.borrow_mut().v_direction = direction_h;
                    agent.borrow_mut().h_direction = direction_v;
                }
            };
        }

        if out_of_bound_v {
            self.h_direction = self.h_direction.invert();
        }

        if out_of_bound_h {
            self.v_direction = self.v_direction.invert();
        }
    }

    fn look_ahead(&self) -> (u32, u32) {
        (
            match self.v_direction {
                //v
                VDirection::Up if self.x != 0 => self.x - 1, // UP
                VDirection::Down => self.x + 1,              // Down
                _ => self.x,
            },
            match self.h_direction {
                //h
                HDirection::Right => self.y + 1, // right
                HDirection::Left if self.y != 0 => self.y - 1, // left
                _ => self.y,
            },
        ) as (u32, u32)
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

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::Black => "black",
            Color::Red => "red",
        }
    }
}
