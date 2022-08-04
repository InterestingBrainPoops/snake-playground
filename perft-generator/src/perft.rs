use crate::{
    board::{Move, Request},
    genmove::{generate_moves_all, generate_moves_you},
};

fn perft(request: &Request, depth: u8, temp_move: Option<Move>) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut sum = 0;
    match temp_move {
        Some(you_mov) => {
            for mov in &generate_moves_all(&request, you_mov) {
                let mut request = (*request).clone();
                request.make_moves(mov);
                sum += perft(&request, depth - 1, None);
            }
            return sum;
        }

        None => {
            for mov in &generate_moves_you(&request) {
                sum += perft(request, depth - 1, Some(mov.clone()));
            }
            return sum;
        }
    }
}
