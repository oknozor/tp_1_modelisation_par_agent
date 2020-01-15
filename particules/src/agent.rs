use crate::environment::Cell;
use crate::environment::Environment;

pub struct Agent {
    pub h_direction: HDirection,
    pub v_direction: VDirection,
    pub x: u32,
    pub y: u32,
}

impl Agent {
    pub fn update(&mut self, environment: &mut Environment) {
        let (x_forward, y_forward) = self.get_position_forward();
        let idx = environment.get_index(x_forward, y_forward);

        if !environment.is_out_of_bound(x_forward, y_forward)
            && match environment.cells[idx] {
                Cell::Filled => false,
                Cell::Empty => true,
            }
        {
            self.x = x_forward;
            self.y = y_forward;
            self.v_direction = self.v_direction;
            self.h_direction = self.h_direction;
        } else {
            let (x, y) = self.get_position_backward();

            self.x = x;
            self.y = y;
            self.v_direction = self.v_direction.invert();
            self.h_direction = self.h_direction.invert();
        }

        let current_idx = environment.get_index(self.x, self.y);
        environment.cells[current_idx] = Cell::Empty;
        let new_idx = environment.get_index(self.x, self.y);
        environment.cells[new_idx] = Cell::Filled;
    }

    fn get_position_forward(&self) -> (u32, u32) {
        let next_x = match self.h_direction {
            HDirection::Left => self.x + 1,
            HDirection::Right => {
                if self.x == 0 {
                    0
                } else {
                    self.x - 1
                }
            }
        } as u32;

        let next_y = match self.v_direction {
            VDirection::Up => self.y + 1,
            VDirection::Down => {
                if self.y == 0 {
                    0
                } else {
                    self.y - 1
                }
            }
        } as u32;

        (next_x, next_y)
    }

    fn get_position_backward(&self) -> (u32, u32) {
        let next_x = match self.h_direction {
            HDirection::Left => {
                if self.x == 0 {
                    0
                } else {
                    self.x - 1
                }
            }
            HDirection::Right => self.x + 1,
        } as u32;

        let next_y = match self.v_direction {
            VDirection::Up => {
                if self.y == 0 {
                    0
                } else {
                    self.y - 1
                }
            }
            VDirection::Down => self.y + 1,
        } as u32;

        (next_x, next_y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDirection {
    Up,
    Down,
}

impl VDirection {
    fn invert(&self) -> VDirection {
        match self {
            VDirection::Up => VDirection::Down,
            VDirection::Down => VDirection::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDirection {
    Left,
    Right,
}

impl HDirection {
    fn invert(&self) -> HDirection {
        match self {
            HDirection::Left => HDirection::Right,
            HDirection::Right => HDirection::Left,
        }
    }
}
