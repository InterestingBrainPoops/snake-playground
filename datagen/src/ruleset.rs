use crate::board::{Board, Direction};

pub trait Ruleset {
    fn name(&self) -> String;
    fn generate_board(&self) -> Board;
    fn step_board(&self, board: Board, moves: Vec<Move>) -> Board;
    fn game_over(&self, board: &Board) -> bool;
}

#[derive(Clone, Copy)]
pub struct Move {
    pub idx: usize,
    pub direction: Direction,
}

impl Move {
    pub fn new(direction: Direction, idx: usize) -> Move {
        Move { idx, direction }
    }
}
