use crate::{board::Board, ruleset::{Ruleset, Move}};

pub struct Standard {}

impl Ruleset for Standard {
    fn generate_board(&self) -> Board {
        todo!()
    }

    fn step_board(&self, board: Board, moves: Vec<Move>) -> Board {
        todo!()
    }
}
