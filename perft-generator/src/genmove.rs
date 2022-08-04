use crate::{
    board::{Move, Request},
    cartprod::cartesian_product,
};

pub fn generate_moves_you(request: &Request) -> Vec<Move> {
    if request.game_over() {
        return vec![];
    }
    request.you.get_moves()
}

pub fn generate_moves_all(request: &Request, you_move: Move) -> Vec<Vec<Move>> {
    if request.game_over() {
        return vec![];
    }
    let mut all_moves = vec![];
    for snake in &request.board.snakes {
        if snake.id != request.you.id {
            all_moves.push(snake.get_moves());
        }
    }
    all_moves.push(vec![you_move]);
    cartesian_product(all_moves)
}
