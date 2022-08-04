use crate::{
    coordinate::{Coordinate, Direction},
    snake::Snake,
};

pub struct Request {
    turn: u32,
    board: Board,
    you: Snake,
}
pub struct Board {
    height: u8,
    width: u8,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Snake>,
}
pub struct Move {
    direction: Direction,
    id: String,
}

impl Request {
    pub fn make_moves(&mut self, moves: Vec<Move>) {
        // move all the snakes
        for snake in &mut self.board.snakes {
            let move_pos = moves.iter().position(|s| s.id == snake.id).unwrap();
            snake.apply_move(moves[move_pos].direction);
        }
        // feed the snakes
        let mut new_food = vec![];
        for food in &self.board.food {
            let mut eaten = false;
            for snake in &mut self.board.snakes {
                if *food == snake.head {
                    snake.feed();
                    eaten = true;
                }
            }
            if !eaten {
                new_food.push(*food);
            }
        }
        self.board.food = new_food;

        // out of bounds eliminations
        let mut new_snakes = vec![];
        for snake in &self.board.snakes {
            if snake.out_of_bounds(self.board.width, self.board.height) {
                continue;
            }

            if snake.health == 0 {
                continue;
            }

            new_snakes.push(snake.clone());
        }
        self.board.snakes = new_snakes;

        // collision eliminations
        let mut new_snakes = vec![];
        for snake in &self.board.snakes {
            if Snake::snake_body_collision(snake, snake) {
                continue;
            }

            let mut bodycollision = false;
            for other in &self.board.snakes {
                if snake.id != other.id && Snake::snake_body_collision(snake, other) {
                    bodycollision = true;
                    break;
                }
            }

            if bodycollision {
                continue;
            }

            let mut headcollision = false;

            for other in &self.board.snakes {
                if snake.id != other.id && Snake::snake_lost_head_collision(snake, other) {
                    headcollision = true;
                    break;
                }
            }

            if headcollision {
                continue;
            }
            new_snakes.push(snake.clone());
        }
        self.board.snakes = new_snakes;
    }
}
