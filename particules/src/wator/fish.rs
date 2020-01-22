use crate::Point;
use crate::Direction;

pub struct Fish {
    pub direction: Direction,
    pub coordinate: Point,
    pub previous_coordinate: Point,
    pub collision: bool,
}