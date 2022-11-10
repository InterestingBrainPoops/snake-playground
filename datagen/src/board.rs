use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone)]
pub struct Board {
    pub food: Vec<Coordinate>,
    pub snakes: Vec<Snake>,
    pub hazards: Vec<Coordinate>,
    pub hazard_damage: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct Snake {
    pub id: usize,
    pub body: Vec<Coordinate>,
    pub health: i8,
    pub alive: bool,
}
impl Snake {
    pub fn new(body: Vec<Coordinate>, health: i8, id: usize) -> Snake {
        Snake {
            body,
            id,
            health,
            alive: true,
        }
    }
}
impl Board {
    pub fn check_food_needed(&self, min_food: u32, spawn_chance: u8) -> u32 {
        let num_current_food = self.food.len() as u32;

        if num_current_food < min_food {
            return min_food - num_current_food;
        }

        if spawn_chance > 0 && (100 - rand::random::<u8>() % 100) < spawn_chance {
            return 1;
        }
        return 0;
    }
    pub fn place_food_randomly(&mut self, num_food: u32) {
        let unoccupied = self.get_unoccupied_points(false, false);
        self.place_food_randomly_at_positions(num_food, unoccupied);
    }

    pub fn get_unoccupied_points(&self, possible_moves: bool, hazards: bool) -> Vec<Coordinate> {
        let mut occupied_points = vec![vec![false; self.height]; self.width];
        for x in &self.food {
            occupied_points[x.x as usize][x.y as usize] = true;
        }

        for snake in &self.snakes {
            if !snake.alive {
                continue;
            }
            for (idx, segment) in snake.body.iter().enumerate() {
                if 
            }
        }
    }
    pub fn place_food_randomly_at_positions(&mut self, num_food: u32, unoccupied: Vec<Coordinate>) {
    }

    pub fn create_default(width: u32, height: u32, num_snakes: u8) -> Board {
        let mut board = Board {
            food: vec![],
            snakes: vec![],
            hazards: vec![],
            hazard_damage: 0,
            width,
            height,
        };
        board.place_snakes_automatic(num_snakes);
        board.place_food_automatic();
        board
    }

    pub fn place_snakes_automatic(&mut self, num_snakes: u8) {
        if self.square() {
            if num_snakes > 8 && self.width < 7 {
                panic!(
                    "Too many snakes for a small board. {} snakes on a {}x{} board doesnt work.",
                    num_snakes, self.width, self.height
                );
            }
            if num_snakes <= 8 && self.width > 7 {
                self.place_snakes_fixed(num_snakes);
                return;
            }
            if self.width > 11 {
                self.place_snakes_distributed(num_snakes);
            }
        }

        self.place_snakes_randomly(num_snakes);
    }
    pub fn place_snakes_randomly(&mut self, num_snakes: u8) {
        self.snakes = (0..num_snakes)
            .map(|x| Snake::new(vec![], 100, x as usize))
            .collect::<Vec<Snake>>();
        todo!()
    }
    pub fn place_snakes_distributed(&mut self, num_snakes: u8) {
        todo!()
    }
    pub fn place_snakes_fixed(&mut self, num_snakes: u8) {
        let mut snakes = (0..num_snakes)
            .map(|x| Snake::new(vec![], 100, x as usize))
            .collect::<Vec<Snake>>();
        let (mn, md, mx) = (1, (self.width as i32 - 1) / 2, self.width as i32 - 2);
        let mut corner_points = vec![
            Coordinate::new(mn, mn),
            Coordinate::new(mn, mx),
            Coordinate::new(mx, mn),
            Coordinate::new(mx, mx),
        ];
        let mut cardinal_points = vec![
            Coordinate::new(mn, md),
            Coordinate::new(md, mn),
            Coordinate::new(md, mx),
            Coordinate::new(mx, md),
        ];

        if snakes.len() > 8 {
            panic!("Too many snakes {} snakes", snakes.len());
        }

        let mut rng = thread_rng();
        corner_points.shuffle(&mut rng);
        cardinal_points.shuffle(&mut rng);

        let mut start_points = vec![];
        if rand::random() {
            start_points.append(&mut corner_points);
            start_points.append(&mut cardinal_points);
        } else {
            start_points.append(&mut cardinal_points);
            start_points.append(&mut corner_points);
        }

        for (idx, snake) in snakes.iter_mut().enumerate() {
            snake.body = (0..3)
                .map(|_| start_points[idx])
                .collect::<Vec<Coordinate>>();
        }
    }
    pub fn place_food_automatic(&mut self) {
        if self.square() && self.width < 7 {
            self.place_food_fixed();
        }
        self.place_food_random();
    }
    pub fn place_food_fixed(&mut self) {
        let mut rng = thread_rng();

        let center_coord =
            Coordinate::new((self.width as i32 - 1) / 2, (self.height as i32 - 1) / 2);

        let small = self.width * self.height < 11 * 11;
        if self.snakes.len() < 4 || !small {
            for snake in &self.snakes {
                let head = snake.body[0];

                let possible_food_locations = vec![
                    Coordinate::new(head.x - 1, head.y - 1),
                    Coordinate::new(head.x - 1, head.y + 1),
                    Coordinate::new(head.x + 1, head.y - 1),
                    Coordinate::new(head.x + 1, head.y + 1),
                ];

                let mut available_food_locations = vec![];
                for food in possible_food_locations {
                    if food == center_coord {
                        continue;
                    }

                    if self.is_occupied(&food, true, true, true) {
                        continue;
                    }

                    let away_from_center = (food.x < head.x && head.x < center_coord.x)
                        || (center_coord.x < head.x && head.x < food.x)
                        || (food.y < head.y && head.y < center_coord.y)
                        || (center_coord.y < head.y && head.y < food.y);

                    if !away_from_center {
                        continue;
                    }
                    if (food.x == 0 || food.x == (self.width as i32 - 1))
                        && (food.y == 0 || food.y == (self.height as i32 - 1))
                    {
                        continue;
                    }
                    available_food_locations.push(food);
                }
                if available_food_locations.is_empty() {
                    panic!("No room for food");
                }

                self.food
                    .push(*available_food_locations.choose(&mut rng).unwrap());
            }
        }

        if !self.is_occupied(&center_coord, true, true, true) {
            self.food.push(center_coord);
        }
    }

    fn is_occupied(&self, point: &Coordinate, snakes: bool, hazards: bool, food: bool) -> bool {
        (food && self.food.contains(point))
            || (snakes && self.snakes.iter().any(|ssnake| ssnake.body.contains(point)))
            || (hazards && self.hazards.contains(point))
    }
    pub fn place_food_random(&mut self) {
        todo!()
    }
    pub fn square(&self) -> bool {
        self.width == self.height
    }
    pub fn wrap(&mut self) {
        for x in &mut self.snakes {
            x.body[0].x %= self.width as i32;
            x.body[0].y %= self.height as i32;
        }
    }
}
