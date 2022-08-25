use crate::board::{Board, Direction};

pub trait Ruleset {
    fn generate_board(&self) -> Board;
    fn step_board(&self, board: Board, moves: Vec<Move>) -> Board;
}

pub struct Move {
    idx: usize,
    direction: Direction,
}
