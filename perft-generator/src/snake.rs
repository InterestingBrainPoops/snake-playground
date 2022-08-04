use crate::coordinate::{Coordinate, Direction};

#[derive(Clone, Debug)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: u8,
    pub body: Vec<Coordinate>,
    pub latency: String,
    pub head: Coordinate,
    pub length: usize,
    pub shout: String,
    pub squad: String,
}

impl Snake {
    pub fn feed(&mut self) {
        self.duplicate_tail();
        self.health = 100;
        self.length += 1;
    }

    fn duplicate_tail(&mut self) {
        self.body.push(*self.body.last().unwrap());
    }
    pub fn apply_move(&mut self, direction: Direction) {
        self.body.pop();
        let new_head = self.body[0] + direction;
        self.body.insert(0, new_head);
        self.health -= 1;
    }

    pub fn out_of_bounds(&self, width: u8, height: u8) -> bool {
        self.head.x < 0
            || self.head.x >= width as i32
            || self.head.y < 0
            || self.head.y >= height as i32
    }

    pub fn snake_body_collision(snake1: &Snake, snake2: &Snake) -> bool {
        snake2.body[1..].contains(&snake1.head)
    }

    pub fn snake_lost_head_collision(snake1: &Snake, snake2: &Snake) -> bool {
        snake1.head == snake2.head && snake1.length <= snake2.length
    }
}
