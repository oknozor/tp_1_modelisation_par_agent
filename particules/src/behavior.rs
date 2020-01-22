use super::environment::Environment;
use super::Direction;
pub trait AgentImpl {
    fn decide(&mut self, environment: &mut Environment);
    fn update(&mut self, environment: &mut Environment);
    fn new(x: i32, y: i32, direction: Direction) -> Self;
}
