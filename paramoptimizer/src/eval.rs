use crate::board::{Board, Coordinate, Position};
use pathfinding::prelude::astar;

// this is always from the perspective of the first snake (hacky fix, but it works)
pub fn score(position: &Position, values: &Vec<i32>) -> i32 {
    let me = position.board.snakes[0].clone();
    let other = position.board.snakes[1].clone();
    let length_difference = (me.body.len() - other.body.len()) as i32 * values[0];
    let distance_to_center = (manhattan(&me.body[0], &Coordinate::new(6, 6))
        - manhattan(&other.body[0], &Coordinate::new(6, 6)))
        * values[1];
    let health_diff = (position.my_health - position.their_health) as i32 * values[2];

    let mut my_nearest = 0;
    let mut their_nearest = 0;
    let my_head = me.body[0];
    let their_head = other.body[0];
    for food in &position.board.food {
        if manhattan(&my_head, food) < manhattan(&their_head, food) {
            my_nearest += 1;
        } else {
            their_nearest += 1;
        }
    }
    let food_ownership_difference = (my_nearest - their_nearest) * values[3];
    let mut my_squares = 0;
    let mut their_squares = 0;
    for x in 0..11 {
        for y in 0..11 {
            let thing = &Coordinate::new(x, y);
            if position.board.snakes[0].body.contains(thing)
                || position.board.snakes[1].body.contains(thing)
            {
                continue;
            }
            let my_dist = manhattan(&my_head, thing);
            let their_dist = manhattan(&my_head, thing);
            if my_dist < their_dist {
                my_squares += 1;
            } else {
                their_squares += 1;
            }
        }
    }
    let square_ownership_difference = (my_squares - their_squares) * values[4];

    length_difference
        + distance_to_center
        + health_diff
        + food_ownership_difference
        + square_ownership_difference
}

fn manhattan(c1: &Coordinate, c2: &Coordinate) -> i32 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}
