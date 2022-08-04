use crate::{
    coordinate::{Coordinate, Direction},
    snake::Snake,
};

#[derive(Clone)]
pub struct Request {
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}
#[derive(Clone)]
pub struct Board {
    pub height: u8,
    pub width: u8,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<Snake>,
}

#[derive(Clone)]
pub struct Move {
    pub direction: Direction,
    pub id: String,
}

impl Move {
    pub fn new(direction: Direction, id: String) -> Self {
        Self { direction, id }
    }
}

impl Request {
    pub fn game_over(&self) -> bool {
        // am i dead
        if self.board.snakes.iter().any(|x| x.id == self.you.id) {
            return true;
        }
        // is there only 1 person alive
        if self.board.snakes.len() == 1 {
            return true;
        }
        // is noone alive
        if self.board.snakes.len() == 0 {
            return true;
        }
        false
    }

    pub fn make_moves(&mut self, moves: &Vec<Move>) {
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
        if let Some(idx) = self.board.snakes.iter().position(|s| s.id == self.you.id) {
            self.you = self.board.snakes[idx].clone();
        }
    }
}
