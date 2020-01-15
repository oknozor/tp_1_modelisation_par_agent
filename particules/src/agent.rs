use crate::environment::Cell;
use crate::environment::Environment;

pub struct Agent {
    pub h_direction: HDirection,
    pub v_direction: VDirection,
    pub x: u32,
    pub y: u32,
}

impl Agent {
    pub fn draw(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Filled;
    }

    pub fn clear(&mut self, environment: &mut Environment) {
        let idx = environment.get_index(self.x, self.y);
        environment.cells[idx] = Cell::Empty;
    }

    pub fn update(&mut self, environment: &mut Environment) {
        self.clear(environment);

        let mut y_forward = self.get_v_forward();
        let mut x_forward = self.get_h_forward();
        let out_of_bound_h = self.x == 0 || environment.is_out_of_bound_h(x_forward);
        let out_of_bound_v =self.y == 0 || environment.is_out_of_bound_v(y_forward);

        if !out_of_bound_h && ! out_of_bound_v {
            let forward_idx = environment.get_index(x_forward, y_forward);
            if let Cell::Filled = environment.cells[forward_idx] {
                self.v_direction = self.v_direction.invert();
                self.h_direction = self.h_direction.invert();
                y_forward = self.get_v_forward();
                x_forward = self.get_h_forward();
            }
        }

        if out_of_bound_v {
            self.v_direction = self.v_direction.invert();
            y_forward = self.get_v_forward();
        }

        if out_of_bound_h {
            self.h_direction = self.h_direction.invert();
            x_forward = self.get_h_forward();
        }

        self.y = y_forward;
        self.x = x_forward;
        self.draw(environment);
    }

    fn get_h_forward(&self) -> u32 {
        (match self.h_direction {
            HDirection::Left if self.x != 0 => self.x - 1,
            HDirection::Right => self.x + 1,
            _ => self.x,
        }) as u32
    }

    fn get_v_forward(&self) -> u32 {
        (match self.v_direction {
            VDirection::Up => self.y + 1,
            VDirection::Down if self.y != 0 => self.y - 1,
            _ => self.y,
        }) as u32
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
