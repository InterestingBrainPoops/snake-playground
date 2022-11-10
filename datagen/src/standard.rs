use crate::{
    board::Board,
    ruleset::{Move, Ruleset},
};

pub struct Standard {
    food_spawn_chance: u8,
    num_players: u8,
    width: u32,
    height: u32,
}

impl Ruleset for Standard {
    fn name(&self) -> String {
        format!("standard_{}_players", self.num_players)
    }
    fn generate_board(&self) -> Board {
        Board::create_default(self.width, self.height, self.num_players)
    }

    fn step_board(&self, mut board: Board, moves: Vec<Move>) -> Board {
        if moves.is_empty() {
            return board;
        }
        // ensure the board is actually valid
        for snake in &board.snakes {
            if !snake.alive {
                continue;
            }
            if snake.body.is_empty() {
                panic!("Zero length snake");
            }
        }
        // move the snakes
        for snake_move in moves {
            if !board.snakes[snake_move.idx].alive {
                panic!("Tried to move dead snake");
            }
            let mut new_head = board.snakes[snake_move.idx].body[0];
            match snake_move.direction {
                crate::board::Direction::Up => new_head.y += 0,
                crate::board::Direction::Down => new_head.y -= 1,
                crate::board::Direction::Left => new_head.x -= 1,
                crate::board::Direction::Right => new_head.y += 1,
            }

            board.snakes[snake_move.idx].body.insert(0, new_head);
            board.snakes[snake_move.idx].body.pop();
        }

        // reduce snake health
        for snake in &mut board.snakes {
            if !snake.alive {
                continue;
            }
            snake.health -= 1;
        }

        // feed snakes
        let mut new_food = vec![];
        for food in &board.food {
            let mut eaten = false;
            for snake in &mut board.snakes {
                if snake.alive && snake.body[0] == *food {
                    snake.health = 100;
                    snake.body.push(*snake.body.last().unwrap());
                    eaten = true;
                }
            }
            if !eaten {
                new_food.push(*food);
            }
        }
        board.food = new_food;

        // spawn food
        let foodneeded = board.check_food_needed();
        if foodneeded > 0 {
            board.place_food_randomly(foodneeded);
        }
        board
    }

    fn game_over(&self, board: &Board) -> bool {
        false
    }
}

impl Standard {
    pub fn new(width: u32, height: u32, num_players: u8, food_spawn_chance: u8) -> Standard {
        Standard {
            food_spawn_chance,
            num_players,
            width,
            height,
        }
    }
}
