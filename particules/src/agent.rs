use crate::environment::Cell;
use crate::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    pub h_direction: HDirection,
    pub v_direction: VDirection,
    pub x: u32,
    pub y: u32,
    pub color: Color
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red
}

impl Default for Color {
    fn default() -> Self { Color::Black }
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::Black => "black",
            Color::Red => "red",
        }
    }
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
        let out_of_bound_v =self.y == 0 || environment.is_out_of_bound_v(y_forward);

        if !out_of_bound_h && ! out_of_bound_v {
            let forward_idx = environment.get_index(x_forward, y_forward);
            let cell_forward = environment.cells[forward_idx].clone();

            match cell_forward {
                Cell::Empty => (),
                Cell::Filled(agent) => {
                    self.color = Color::Red;
                    agent.borrow_mut().color = Color::Red;

                    // Swap agents directions
                    let direction_h = self.h_direction;
                    let direction_v = self.v_direction;
                    self.v_direction = agent.borrow().v_direction;
                    self.h_direction = agent.borrow().h_direction;
                    agent.borrow_mut().h_direction = direction_h;
                    agent.borrow_mut().v_direction = direction_v;

                }
            };
        }

        if out_of_bound_v {
            self.v_direction = self.v_direction.invert();
        }

        if out_of_bound_h {
            self.h_direction = self.h_direction.invert();
        }
    }

    fn look_ahead(&self) -> (u32, u32) {
        (match self.h_direction {
            HDirection::Left if self.x != 0 => self.x - 1,
            HDirection::Right => self.x + 1,
            _ => self.x,
        }, match self.v_direction {
            VDirection::Up => self.y + 1,
            VDirection::Down if self.y != 0 => self.y - 1,
            _ => self.y,
        }) as (u32, u32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDirection {
    None,
    Up,
    Down,
}

impl VDirection {
    fn invert(&self) -> VDirection {
        match self {
            VDirection::None => VDirection::None,
            VDirection::Up => VDirection::Down,
            VDirection::Down => VDirection::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDirection {
    None,
    Left,
    Right,
}

impl HDirection {
    fn invert(&self) -> HDirection {
        match self {
            HDirection::None => HDirection::None,
            HDirection::Left => HDirection::Right,
            HDirection::Right => HDirection::Left,
        }
    }
}
