use crate::board::{Coordinate, Position};

// this is always from the perspective of the first snake (hacky fix, but it works)
pub fn score(position: &Position, values: &Vec<i32>) -> i32 {
    let me = position.board.snakes[0].clone();
    let other = position.board.snakes[1].clone();
    let length_difference = (me.body.len() - other.body.len()) as i32 * values[0];
    let distance_to_center = (manhattan(me.body[0], Coordinate::new(6, 6))
        - manhattan(other.body[0], Coordinate::new(6, 6)))
        * values[1];

    length_difference + distance_to_center
}

fn manhattan(c1: Coordinate, c2: Coordinate) -> i32 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}
