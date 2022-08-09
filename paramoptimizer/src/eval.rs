use crate::board::{Board, Coordinate, Position};
use pathfinding::prelude::astar;

// this is always from the perspective of the first snake (hacky fix, but it works)
pub fn score(position: &Position) -> Vec<i32> {
    let me = position.board.snakes[0].clone();
    let other = position.board.snakes[1].clone();
    let length_difference = (me.body.len() - other.body.len()) as i32;
    let distance_to_center = (manhattan(&me.body[0], &Coordinate::new(6, 6))
        - manhattan(&other.body[0], &Coordinate::new(6, 6)));
    let health_diff = (position.my_health - position.their_health) as i32;

    let mut my_nearest = 0;
    let mut their_nearest = 0;
    for food in &position.board.food {
        let my_path = astar(
            &position.board.snakes[0].body[0],
            |p| successors(p, &position.board),
            |p| manhattan(p, food),
            |p| *p == *food,
        );
        let their_path = astar(
            &position.board.snakes[1].body[0],
            |p| successors(p, &position.board),
            |p| manhattan(p, food),
            |p| *p == *food,
        );
        let my_dist = match my_path {
            None => 1000,
            Some((path, _)) => path.len(),
        };
        let their_dist = match their_path {
            None => 1000,
            Some((path, _)) => path.len(),
        };
        if my_dist < their_dist {
            my_nearest += 1;
        } else {
            their_nearest += 1;
        }
    }
    let food_ownership_difference = (my_nearest - their_nearest);
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
            let my_path = astar(
                &position.board.snakes[0].body[0],
                |p| successors(p, &position.board),
                |p| manhattan(p, thing),
                |p| *p == *thing,
            );
            let their_path = astar(
                &position.board.snakes[1].body[0],
                |p| successors(p, &position.board),
                |p| manhattan(p, thing),
                |p| *p == *thing,
            );
            let my_dist = match my_path {
                None => 1000,
                Some((path, _)) => path.len(),
            };
            let their_dist = match their_path {
                None => 1000,
                Some((path, _)) => path.len(),
            };

            if my_dist < their_dist {
                my_squares += 1;
            } else {
                their_squares += 1;
            }
        }
    }
    let square_ownership_difference = (my_squares - their_squares);

    vec![
        length_difference,
        distance_to_center,
        health_diff,
        food_ownership_difference,
        square_ownership_difference,
    ]
}

fn successors(coord: &Coordinate, board: &Board) -> Vec<(Coordinate, i32)> {
    let possible = vec![
        Coordinate::new(0, 1),
        Coordinate::new(0, -1),
        Coordinate::new(-1, 0),
        Coordinate::new(1, 0),
    ];
    let mut new_possible = vec![];
    for thing in &possible {
        new_possible.push(*thing + *coord);
    }

    let mut out = vec![];

    for thing in &new_possible {
        if thing.x < 0 || thing.x > 10 || thing.y < 0 || thing.y > 10 {
            continue;
        }
        if board.snakes[0].body.contains(thing) || board.snakes[1].body.contains(thing) {
            continue;
        }
        out.push(*thing);
    }

    out.iter().map(|p| (*p, 1)).collect()
}

fn manhattan(c1: &Coordinate, c2: &Coordinate) -> i32 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}
