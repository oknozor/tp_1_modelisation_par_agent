use super::AgentKind;
use super::AgentRef;
use super::Point;

#[derive(Clone)]
pub enum Cell {
    Empty,
    Filled(AgentRef),
}

impl Cell {
    pub(crate) fn is_fish(&self) -> bool {
        match self {
            Cell::Filled(agent) => {
                if let AgentKind::Fish = agent.kind {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub(crate) fn is_empty_cell(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }

    pub(crate) fn into_agent(&self) -> &AgentRef {
        match self {
            Cell::Filled(a) => a,
            _ => panic!("Expected an agent"),
        }
    }
}

pub struct Environment {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) cells: Vec<Cell>,
    pub(crate) borderless: bool,
    pub(crate) fish_breed_time: u8,
    pub(crate) shark_breed_time: u8,
    pub(crate) shark_starve_time: u8,
}

impl Environment {
    pub fn debug(&self) {
        self.cells.iter().enumerate().for_each(|(i, cell)| {
            if let Cell::Filled(a) = cell {
                println!("cell {}:{}", a.coordinate().x, a.coordinate().y);
                let agent_idx = self.get_index(a.coordinate());
                println!("cell idx = {}, agent coord to idx = {}", i, agent_idx);
            } else {
                println!("empty cell");
            }
        })
    }
    pub fn is_out_of_bound_y(&self, y: i32) -> bool {
        if self.borderless {
            false
        } else if y > self.height - 1 || y < 0 {
            true
        } else {
            false
        }
    }

    pub fn is_out_of_bound_x(&self, x: i32) -> bool {
        if self.borderless {
            false
        } else if x > self.width - 1 || x < 0 {
            true
        } else {
            false
        }
    }

    pub fn swap(&mut self, a: Point, b: Point) {
        let a = self.get_index(a);
        let b = self.get_index(b);
        let cell_a = self.cells[a].clone();
        self.cells[a] = self.cells[b].clone();
        self.cells[b] = cell_a;
    }

    fn out_of_bound(&self, point: Point) -> bool {
        self.is_out_of_bound_x(point.x) || self.is_out_of_bound_y(point.y)
    }

    pub fn get_index(&self, point: Point) -> usize {
        (point.y * self.width + point.x) as usize
    }

    pub fn get_cell(&self, point: Point) -> Option<&Cell> {
        if self.out_of_bound(point) {
            None
        } else {
            let idx = self.get_index(point);
            Some(&self.cells[idx])
        }
    }

    pub fn get_mut_cell(&mut self, point: Point) -> Option<&mut Cell> {
        if self.out_of_bound(point) {
            None
        } else {
            let idx = self.get_index(point);
            Some(&mut self.cells[idx])
        }
    }

    pub fn set_cell(&mut self, point: Point, cell: Cell) -> Result<(), &str> {
        if self.out_of_bound(point) {
            Err("Cannot set cell out of bounds!")
        } else {
            let idx = self.get_index(point);
            self.cells.push(cell);
            self.cells.swap_remove(idx);
            Ok(())
        }
    }

    pub fn set_agent_cell(&mut self, from: Point, to: Point) {
        let current = self.get_index(from);
        let agent = self.cells[current].into_agent().clone();
        let idx = self.get_index(to);

        self.cells[current] = Cell::Empty;
        agent.set_coordinate(to);
        self.cells[idx] = Cell::Filled(agent.clone());
    }

    pub fn new(width: i32, height: i32, borderless: bool) -> Self {
        let mut cells = vec![];
        let size = width * height;

        for _ in 0..size {
            cells.push(Cell::Empty);
        }

        Environment {
            width,
            height,
            cells,
            borderless,
            fish_breed_time: 0,
            shark_breed_time: 0,
            shark_starve_time: 0,
        }
    }

    pub fn new_fish_shark(
        width: i32,
        height: i32,
        borderless: bool,
        fish_breed_time: u8,
        shark_breed_time: u8,
        shark_starve_time: u8,
    ) -> Self {
        let mut cells = vec![];
        let size = width * height;

        for _ in 0..size {
            cells.push(Cell::Empty);
        }

        Environment {
            width,
            height,
            cells,
            borderless,
            shark_breed_time,
            fish_breed_time,
            shark_starve_time,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::environment::Cell;
    use crate::environment::Environment;
    use crate::wator::fish::Decision;
    use crate::wator::fish::Fish;
    use crate::AgentRef;
    use crate::Point;

    #[test]
    fn should_get_index() {
        let env = Environment::new(5, 5, false);

        assert_eq!(env.get_index(Point::new(0, 0)), 0);
        assert_eq!(env.get_index(Point::new(1, 0)), 1);
        assert_eq!(env.get_index(Point::new(2, 0)), 2);
        assert_eq!(env.get_index(Point::new(3, 0)), 3);
        assert_eq!(env.get_index(Point::new(4, 0)), 4);
        assert_eq!(env.get_index(Point::new(0, 1)), 5);
        assert_eq!(env.get_index(Point::new(1, 1)), 6);
        assert_eq!(env.get_index(Point::new(1, 2)), 11);
    }

    #[test]
    fn should_set_cell() {
        let mut env = Environment::new(5, 5, false);

        let agent = Fish {
            coordinate: Point::new(0, 0),
            decision: Decision::Stall,
            breed_count_down: 0,
        };

        env.set_cell(Point::new(0, 0), Cell::Filled(AgentRef::from_fish(agent)))
            .unwrap();

        let res = env.get_cell(Point::new(0, 0)).unwrap();
        match res {
            Cell::Filled(a) => {
                assert_eq!(0, a.coordinate().x);
                assert_eq!(0, a.coordinate().y);
            }
            _ => panic!("expected an agent"),
        }
    }

    #[test]
    fn should_swap_cells() {
        let mut env = Environment::new(5, 5, false);

        let agent = Fish {
            coordinate: Point::new(0, 0),
            decision: Decision::Stall,
            breed_count_down: 0,
        };

        let agent = &AgentRef::from_fish(agent);
        env.set_agent_cell(Point::new(0, 0), agent.coordinate());
        env.set_agent_cell(Point::new(1, 0), agent.coordinate());

        let expected_filled = env.get_cell(Point::new(1, 0)).unwrap();
        let expected_empty = env.get_cell(Point::new(0, 0)).unwrap();

        env.debug();

        match expected_filled {
            Cell::Filled(a) => {
                assert_eq!(1, a.coordinate().x);
                assert_eq!(0, a.coordinate().y);
            }
            _ => panic!("expected an agent"),
        };

        match expected_empty {
            Cell::Empty => (),
            _ => panic!("expected empty"),
        };
    }
}
