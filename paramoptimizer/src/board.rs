use std::ops::Add;

use deepsize::DeepSizeOf;
use serde::{Deserialize, Serialize};

/// represents a coordinate
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

/// battlesnake struct
#[derive(Serialize, Deserialize, Debug, DeepSizeOf, Clone)]
pub struct Battlesnake {
    pub body: Vec<Coordinate>,
    id: String,
}

/// board struct
#[derive(Serialize, Deserialize, Debug, DeepSizeOf, Clone)]
pub struct Board {
    pub snakes: Vec<Battlesnake>,
    pub food: Vec<Coordinate>,
}

/// game struct, holds all the boards
#[derive(Serialize, Deserialize, Debug, DeepSizeOf)]
#[serde(transparent)]
pub struct Game {
    pub positions: Vec<Board>,
}
// number of parameters
pub const NUM_PARAMS: usize = 5;

// position
#[derive(DeepSizeOf, Serialize, Clone, Deserialize)]
pub struct Position {
    /// end status
    pub status: Status,
    /// board of the position
    pub board: Board,
    /// my health
    pub my_health: u8,
    /// their health
    pub their_health: u8,
    /// current parameter values
    pub param_values: [f64; NUM_PARAMS],
    /// future parameter values
    pub future_param_values: [f64; NUM_PARAMS],
    /// occupancy bitbaord
    pub all_bb: u128,
}

/// Status of the board
#[derive(Clone, Copy, DeepSizeOf, Serialize, Deserialize)]
pub enum Status {
    /// I won
    Win,
    /// I lost
    Loss,
    /// Draw
    Draw,
}

impl Into<f64> for Status {
    fn into(self) -> f64 {
        match self {
            Status::Win => 1.0,  // win is a 1.0
            Status::Loss => 0.0, // loss is 0.0
            Status::Draw => 0.5, // draw is a 0.5
        }
    }
}

impl From<Coordinate> for u128 {
    fn from(input: Coordinate) -> Self {
        1 << (input.x + input.y * 11)
    }
}

impl Battlesnake {
    /// am i dead
    pub fn dead(&self, state: &Board, health: u8) -> bool {
        // my head
        let my_head = self.body[0];
        // the other snakes
        let other_snakes: Vec<Battlesnake> = state
            .snakes
            .iter()
            .filter(|x| x.id != self.id)
            .cloned()
            .collect();

        // if my health is 0
        if health == 0 {
            return true;
        }
        // am i out of bounds?
        if my_head.x < 0 || my_head.x > 10 || my_head.y < 0 || my_head.y > 10 {
            return true;
        }
        // am i in the other snakes body?
        if other_snakes[0].body[1..].contains(&my_head) {
            return true;
        }
        // did I lose to head to head?
        if other_snakes[0].body[0] == my_head && other_snakes[0].body.len() >= self.body.len() {
            return true;
        }
        // am I in my body?
        if self.body[1..].contains(&my_head) {
            return true;
        }
        false
    }
}
