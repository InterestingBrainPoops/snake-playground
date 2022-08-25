pub struct Board {
    food: Vec<Coordinate>,
    snakes: Vec<Snake>,
    hazards: Vec<Coordinate>,
    hazard_damage: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Coordinate {
    x: i32,
    y: i32,
}

pub struct Snake {
    body: Vec<Coordinate>,
    health: i8,
}
