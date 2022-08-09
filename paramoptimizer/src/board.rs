use std::ops::Add;

use deepsize::DeepSizeOf;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, DeepSizeOf, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, DeepSizeOf, Clone)]
pub struct Battlesnake {
    pub body: Vec<Coordinate>,
    id: String,
}
#[derive(Serialize, Deserialize, Debug, DeepSizeOf, Clone)]
pub struct Board {
    pub snakes: Vec<Battlesnake>,
    pub food: Vec<Coordinate>,
}

#[derive(Serialize, Deserialize, Debug, DeepSizeOf)]
#[serde(transparent)]
pub struct Game {
    pub positions: Vec<Board>,
}

#[derive(DeepSizeOf)]
pub struct Position {
    pub status: Status,
    pub board: Board,
    pub my_health: u8,
    pub their_health: u8,
}

#[derive(Clone, Copy, DeepSizeOf)]
pub enum Status {
    Win,
    Loss,
    Draw,
}

impl Into<f64> for Status {
    fn into(self) -> f64 {
        match self {
            Status::Win => 1.0,
            Status::Loss => 0.0,
            Status::Draw => 0.5,
        }
    }
}

impl Battlesnake {
    pub fn dead(&self, state: &Board, health: u8) -> bool {
        let my_head = self.body[0];
        let other_snakes: Vec<Battlesnake> = state
            .snakes
            .iter()
            .filter(|x| x.id != self.id)
            .cloned()
            .collect();
        if health == 0 {
            return true;
        }

        if my_head.x < 0 || my_head.x > 10 || my_head.y < 0 || my_head.y > 10 {
            return true;
        }

        if other_snakes[0].body[1..].contains(&my_head) {
            return true;
        }

        if other_snakes[0].body[0] == my_head && other_snakes[0].body.len() >= self.body.len() {
            return true;
        }

        false
    }
}
