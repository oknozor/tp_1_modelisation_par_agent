use crate::{AgentCommand};

use super::Direction;
use super::environment::Cell;
use super::environment::Environment;
use super::HDirection;
use super::Point;
use super::VDirection;

pub trait AgentBehavior {
    fn decide(&mut self, environment: &Environment);
    fn update(&mut self, environment: &mut Environment) -> AgentCommand;
    fn collision(&self) -> bool;
    fn direction(&self) -> Direction;
    fn coordinate(&self) -> Point;
    fn set_coordinate(&mut self, point: Point);
    fn previous_coordinate(&self) -> Point;
    fn set_previous_coordinate(&mut self, point: Point);
    fn set_collision(&mut self, collision: bool);
    fn set_direction(&mut self, direction: Direction);
    fn get_color(&self) -> (f32, f32, f32);

    fn peek(&self, environment: &Environment, x_offset: i32, y_offset: i32) -> Option<Point> {
        let position = Point::new(
            self.coordinate().x + x_offset,
            self.coordinate().y + y_offset,
        );
        if let Some(Cell::Empty) = environment.get_cell(position) {
            Some(position)
        } else {
            None
        }
    }

    fn peek_cell<'a>(
        &self,
        environment: &'a Environment,
        x_offset: i32,
        y_offset: i32,
    ) -> Option<(&'a Cell, Point)> {
        let position = Point::new(
            self.coordinate().x + x_offset,
            self.coordinate().y + y_offset,
        );
        environment.get_cell(position).map(|cell| (cell, position))
    }

    fn move_forward(&mut self, environment: &mut Environment) {
        self.set_previous_coordinate(self.coordinate());

        let forward_position = if environment.borderless {
            self.look_ahead_borderless(environment.width, environment.height)
        } else {
            self.look_ahead()
        };

        match environment.get_cell(forward_position) {
            Some(Cell::Empty) => {
                let agent = environment.get_cell(self.coordinate()).unwrap().clone();
                self.set_coordinate(forward_position);

                environment
                    .set_cell(self.previous_coordinate(), Cell::Empty)
                    .unwrap();
                environment.set_cell(self.coordinate(), agent).unwrap();
            }
            _ => (),
        }
    }

    fn look_ahead(&self) -> Point {
        Point {
            x: match self.direction().x {
                HDirection::Right => self.coordinate().x + 1,
                HDirection::Left => self.coordinate().x - 1,
                _ => self.coordinate().x,
            },
            y: match self.direction().y {
                VDirection::Up => self.coordinate().y - 1,
                VDirection::Down => self.coordinate().y + 1,
                _ => self.coordinate().y,
            },
        }
    }

    fn look_ahead_borderless(&self, width: i32, height: i32) -> Point {
        Point {
            x: match self.direction().x {
                HDirection::Right => {
                    if self.coordinate().x + 1 > width - 1 {
                        0
                    } else {
                        self.coordinate().x + 1
                    }
                }
                HDirection::Left => {
                    if self.coordinate().x - 1 < 0 {
                        width - 1
                    } else {
                        self.coordinate().x - 1
                    }
                }
                _ => self.coordinate().x,
            },
            y: match self.direction().y {
                VDirection::Up => {
                    if self.coordinate().y - 1 < 0 {
                        height - 1
                    } else {
                        self.coordinate().y - 1
                    }
                }
                VDirection::Down => {
                    if self.coordinate().y + 1 > height - 1 {
                        0
                    } else {
                        self.coordinate().y + 1
                    }
                }
                _ => self.coordinate().y,
            },
        }
    }
}
