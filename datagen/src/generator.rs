use rayon::prelude::*;
use uuid::Uuid;

use crate::{
    board::{Board, Direction},
    ruleset::{Move, Ruleset},
};

pub struct Game {
    boards: Vec<Board>,
    game_id: String,
    ruleset_name: String,
}

pub trait GetMove: Send + Sync {
    fn get_move(&mut self, board: &Board, you_idx: usize) -> Direction;
    fn clone_dyn(&self) -> Box<dyn GetMove>;
}

impl Clone for Box<dyn GetMove> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

pub struct Generator<R>
where
    R: Ruleset,
{
    players: Vec<Box<dyn GetMove>>,
    ruleset: R,
}
impl<R> Generator<R>
where
    R: Ruleset + Sync,
{
    // make a new game from the players and
    pub fn new(players: Vec<Box<dyn GetMove>>, ruleset: R) -> Self {
        Self { players, ruleset }
    }

    pub fn generate_data(&mut self, total_games: u64) -> Vec<Game> {
        let mut games = vec![];
        for _ in 0..total_games {
            games.push(Game {
                boards: vec![self.ruleset.generate_board()],
                game_id: Uuid::new_v4().to_string(),
                ruleset_name: self.ruleset.name(),
            });
        }
        games.par_iter_mut().for_each(|x| {
            let mut players = self.players.clone();
            let mut board = x.boards[0].clone();
            while !self.ruleset.game_over(&board) {
                let moves = players
                    .iter_mut()
                    .enumerate()
                    .map(|(idx, player)| Move::new(player.get_move(&board, idx), idx))
                    .collect::<Vec<Move>>();
                board = self.ruleset.step_board(board, moves);
                x.boards.push(board.clone());
            }
        });
        games
    }
}
