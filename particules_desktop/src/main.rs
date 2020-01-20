#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use nannou::prelude::*;

use particules::agent::Agent;
use particules::Direction;
use particules::HDirection;
use particules::sma::Sma;
use particules::VDirection;

mod user_config;

lazy_static! {
    static ref CONFIG: user_config::Config = {
        let mut file = File::open("config.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).expect("expected json")
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();


    nannou::app(model).update(update).run();
}

#[derive(Clone)]
struct Cell {
    w: f32,
    state: CellState,
    previous: Option<(usize, usize)>,
}

#[derive(Clone)]
pub enum CellState {
    Empty,
    Collision,
    Fill,
}

impl Cell {
    fn new(w: f32, state: CellState) -> Self {
        let w = w;
        let previous = None;
        Cell { w, state, previous }
    }

    fn display(&self, draw: &app::Draw, x: f32, y: f32) {
        let fill = match self.state {
            CellState::Empty => rgb(1.0, 1.0, 1.0),
            CellState::Collision => rgb(1.0, 0.0, 0.0),
            CellState::Fill => rgb(0.0, 0.0, 0.0),
        };

        draw.rect()
            .x_y(x, y)
            .w_h(self.w, self.w)
            .rgb(fill.red, fill.green, fill.blue)
            .stroke(rgb(0.0, 0.0, 0.0));
    }
}

struct Grid {
    w: usize,
    columns: usize,
    rows: usize,
    board: Vec<Vec<Cell>>,
    pub sma: Sma,
}

impl Grid {
    fn new(rect: Rect, w: usize) -> Self {
        let columns = rect.w() as usize / w;
        let rows = rect.h() as usize / w;
        //let mut board = vec![vec![Cell::new(w as f32); rows]; columns];
        let mut board: Vec<Vec<Cell>> = (0..columns)
            .map(|_| {
                (0..rows)
                    .map(|_| Cell::new(w as f32, CellState::Empty))
                    .collect()
            })
            .collect();

        board[0][0].previous = Some((0, 0));

        let mut sma = Sma::new(columns as i32, rows as i32);

        sma.add_agent(Agent::new(
            0,
            0,
            Direction {
                x: HDirection::Right,
                y: VDirection::Up,
            },
        ));

        let mut grid = Grid {
            w,
            columns,
            rows,
            board,
            sma,
        };
        grid.init();
        grid
    }

    fn init(&mut self) {
        self.board = (0..self.columns)
            .map(|_| {
                (0..self.rows)
                    .map(|_| Cell::new(self.w as f32, CellState::Empty))
                    .collect()
            })
            .collect();
    }

    fn generate(&mut self) {
        for agent in self.sma.agents.iter() {
            let (x_agent, y_agent) = (
                agent.borrow().coordinate.x as usize,
                agent.borrow().coordinate.y as usize,
            );
            self.board[x_agent][y_agent].state = CellState::Empty;
        }
        self.sma.tick();
        for agent in self.sma.agents.iter() {
            let (x_agent, y_agent) = (
                agent.borrow().coordinate.x as usize,
                agent.borrow().coordinate.y as usize,
            );

            if agent.borrow().collision {
                self.board[x_agent][y_agent].state = CellState::Collision;
            } else {
                self.board[x_agent][y_agent].state = CellState::Fill;
            }
        }
    }

    // This is the easy part, just draw the cells fill white if 1, black if 0
    fn display(&self, draw: &app::Draw, rect: &Rect) {

        for i in 0..self.columns {
            for j in 0..self.rows {
                let x = (i * self.w) as f32 - rect.right() as f32;
                let y = (j * self.w) as f32 - rect.top() as f32;
                let offset = self.w as f32 / 2.0;
                self.board[i][j].display(&draw, x + offset, y + offset);
            }
        }
    }
}

struct Model {
    pub grid: Grid,
}


fn model(app: &App) -> Model {
    let cell_w = CONFIG.cell_size;
    let h = CONFIG.y;
    let w = CONFIG.x;
    let rect = Rect::from_w_h(w * cell_w, h * cell_w);
    app.new_window()
        .with_maximized(true)
        .mouse_pressed(mouse_pressed)
        .event(window_event)
        .view(view)
        .build()
        .unwrap();

    let grid = Grid::new(rect, cell_w as usize);
    Model { grid }
}

fn mouse_pressed(_app: &App, m: &mut Model, _button: MouseButton) {
    // Reset board when mouse is pressed
    m.grid.init();
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    m.grid.generate();
}

fn view(app: &App, m: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(rgb(1.0, 1.0, 1.0));

    m.grid.display(&draw, &app.window_rect());

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(code) => println!("{:?}", code),
        DroppedFile(path) => {
//            let rect = Rect::from_w_h(config.x as f32x * cell_w, config.x as f32 * cell_w);
//            let grid = Grid::new(rect);
//            let mut sma = Sma::new(config.y as i32, config.x as i32);
//
//            sma.add_agent(Agent::new(
//                0,
//                0,
//                Direction {
//                    x: HDirection::Right,
//                    y: VDirection::Up,
//                },
//            ));
//
//            model.grid = grid;
        }
        _ => {}
    }
}
