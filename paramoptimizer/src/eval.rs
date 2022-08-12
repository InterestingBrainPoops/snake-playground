use crate::board::{Board, Coordinate, Position, NUM_PARAMS};
use pathfinding::prelude::astar;

// this is always from the perspective of the first snake (hacky fix, but it works)
pub fn score(position: &Position) -> [f64; NUM_PARAMS] {
    // me
    let me = position.board.snakes[0].clone();
    // them
    let other = position.board.snakes[1].clone();
    // the length difference between me and them
    let length_difference = (me.body.len() - other.body.len()) as i32;
    // my distance to center - their distance to center
    let distance_to_center = (manhattan(&me.body[0], &Coordinate::new(6, 6))
        - manhattan(&other.body[0], &Coordinate::new(6, 6)));
    // my heatlh - their health
    let health_diff = (position.my_health - position.their_health) as i32;
    // my nearest food
    let mut my_nearest = 0;
    // their nearest food
    let mut their_nearest = 0;
    for food in &position.board.food {
        // my path to the food
        let my_path = astar(
            &position.board.snakes[0].body[0],
            |p| successors(p, &position.board, position.all_bb),
            |p| manhattan(p, food),
            |p| *p == *food,
        );
        // their path to the same food
        let their_path = astar(
            &position.board.snakes[1].body[0],
            |p| successors(p, &position.board, position.all_bb),
            |p| manhattan(p, food),
            |p| *p == *food,
        );
        // my distance to the food
        let my_dist = match my_path {
            None => 1000,                  // if i have no path, set the path length to 1k
            Some((path, _)) => path.len(), // otherwise set it to the length of the path
        };
        // their distance to the food
        let their_dist = match their_path {
            None => 1000,                  // if they have no path, set the path length to 1k
            Some((path, _)) => path.len(), // otherwise set it to the length of the path
        };
        // give credit based on whose path is shorter
        if my_dist < their_dist {
            // if my path is shorter, then credit me
            my_nearest += 1;
        } else {
            // if their path is shorter, then credit them
            their_nearest += 1;
        }
    }
    // my_nearest foods - their_nearest-foods
    let food_ownership_difference = (my_nearest - their_nearest);
    // my owned squares
    let mut my_squares = 0;
    // their owned squares
    let mut their_squares = 0;
    // go through all the squares on the board
    for x in 0..11 {
        for y in 0..11 {
            // the curent coordinate
            let thing = &Coordinate::new(x, y);
            // if the square is in either persons body, ignore it
            if position.board.snakes[0].body.contains(thing)
                || position.board.snakes[1].body.contains(thing)
            {
                continue;
            }

            // my path to the square
            let my_path = astar(
                &position.board.snakes[0].body[0],
                |p| successors(p, &position.board, position.all_bb),
                |p| manhattan(p, thing),
                |p| *p == *thing,
            );

            // their path to the square
            let their_path = astar(
                &position.board.snakes[1].body[0],
                |p| successors(p, &position.board, position.all_bb),
                |p| manhattan(p, thing),
                |p| *p == *thing,
            );
            // my distance to the square
            let my_dist = match my_path {
                None => 1000,                  // if i dont have a path, set it to 1k
                Some((path, _)) => path.len(), // if i do, set it to the length of the path
            };
            let their_dist = match their_path {
                None => 1000,                  // if they dont have a path, set it to 1k
                Some((path, _)) => path.len(), // if they do, set to the length of the path
            };
            // credit based on who is closer
            if my_dist < their_dist {
                my_squares += 1;
            } else {
                their_squares += 1;
            }
        }
    }

    // the difference between the owned squares
    let square_ownership_difference = (my_squares - their_squares);

    [
        length_difference as f64,
        distance_to_center as f64,
        health_diff as f64,
        food_ownership_difference as f64,
        square_ownership_difference as f64,
    ]
}

// successors for a given coordinate
fn successors(coord: &Coordinate, board: &Board, bb: u128) -> Vec<(Coordinate, i32)> {
    // possible successors
    let possible = [
        Coordinate::new(0, 1),
        Coordinate::new(0, -1),
        Coordinate::new(-1, 0),
        Coordinate::new(1, 0),
    ];

    // possible ending squares
    let mut new_possible = vec![];
    for thing in &possible {
        new_possible.push(*thing + *coord);
    }

    let mut out = vec![];
    // go through each possible end square
    for thing in &new_possible {
        // if im oob, dont include it
        if thing.x < 0 || thing.x > 10 || thing.y < 0 || thing.y > 10 {
            continue;
        }
        // if the bitboard has this, then dont include it
        if bb & u128::from(*thing) != 0 {
            continue;
        }
        // add it to the out vector
        out.push(*thing);
    }
    // add a weight to each one
    out.iter().map(|p| (*p, 1)).collect()
}

// manhattan distance between two coordinates
fn manhattan(c1: &Coordinate, c2: &Coordinate) -> i32 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}
