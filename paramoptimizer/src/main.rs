mod board;
mod eval;
mod optimize;

use crate::{
    board::{Game, Position, Status},
    optimize::Optimizer,
};
use brotli2::read::BrotliDecoder;
use deepsize::DeepSizeOf;
use rusqlite::{Connection, Result};

use std::io::{prelude::*, Cursor};

#[derive(Debug)]
struct GameData {
    id: i32,
    game_id: String,
    game_type: String,
    unique_snake_count: u8,
    compressed_frames_json: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open("./two_snake_snakedump.sqlite")?;

    let mut stmt = conn.prepare("SELECT id, game_id , game_type, unique_snake_count , compressed_frames_json FROM snake_games")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(GameData {
            id: row.get(0)?,
            game_id: row.get(1)?,
            game_type: row.get(2)?,
            unique_snake_count: row.get(3)?,
            compressed_frames_json: row.get(4)?,
        })
    })?;
    let mut games = vec![];
    for contents in person_iter {
        let mut decompressor = BrotliDecoder::new(Cursor::new(
            contents.unwrap().compressed_frames_json.unwrap(),
        ));
        let mut contents = String::new();
        decompressor.read_to_string(&mut contents).unwrap();
        let game: Game = serde_json::from_str(&contents).unwrap();
        let mut me_health = 100;
        let mut they_health = 100;
        for (idx, position) in game.positions.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            let old_food = game.positions[idx - 1].food.clone();
            let my_head = game.positions[idx].snakes[0].body[0];
            let other_head = game.positions[idx].snakes[1].body[0];
            me_health -= 1;
            they_health -= 1;
            if old_food.contains(&my_head) {
                me_health = 100;
            }
            if old_food.contains(&other_head) {
                they_health = 100;
            }
        }

        let last = game.positions.last().unwrap();
        let they_dead = last.snakes[1].dead(last, they_health);
        let me_dead = last.snakes[0].dead(last, me_health);
        let status = if me_dead && they_dead {
            Status::Draw
        } else if !me_dead && they_dead {
            Status::Win
        } else {
            Status::Loss
        };
        let mut positions = vec![];
        for (idx, position) in game.positions.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            let old_food = game.positions[idx - 1].food.clone();
            let my_head = game.positions[idx].snakes[0].body[0];
            let other_head = game.positions[idx].snakes[1].body[0];
            me_health -= 1;
            they_health -= 1;
            if old_food.contains(&my_head) {
                me_health = 100;
            }
            if old_food.contains(&other_head) {
                they_health = 100;
            }
            positions.push(Position {
                status,
                my_health: me_health,
                their_health: they_health,
                board: position.clone(),
            })
        }
        games.append(&mut positions);
    }
    println!("{:?}", games.deep_size_of());
    let x = Optimizer { positions: games };
    let new_params = x.local_optimize(vec![30, 4]);
    println!("{:?}", new_params);
    Ok(())
}
