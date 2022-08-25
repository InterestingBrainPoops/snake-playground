use crate::{
    board::{Board, Direction},
    ruleset::Ruleset,
};

pub trait GetMove {
    fn get_move(&mut self, board: &Board, you_idx: usize) -> Direction;
}

pub struct Generator<R>
where
    R: Ruleset,
{
    games: Vec<Board>,
    players: Vec<Box<dyn GetMove>>,
    ruleset: R,
}
impl<R> Generator<R>
where
    R: Ruleset,
{
    // make a new game from the players and
    pub fn new(players: Vec<Box<dyn GetMove>>, ruleset: R) -> Self {
        Self {
            games: vec![],
            players,
            ruleset,
        }
    }

    pub fn initialize(&mut self, num_concurrent_games: u64) {
        (0..num_concurrent_games).for_each(|x| {
            self.games.push(self.ruleset.generate_board());
        });
    }

    pub fn generate_data(&mut self, total_games: u64) -> String {}
}
